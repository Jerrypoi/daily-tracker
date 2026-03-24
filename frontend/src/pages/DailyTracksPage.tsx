import { useEffect, useMemo, useRef, useState } from 'react'

import {
  createDailyTrack,
  deleteDailyTrack,
  listDailyTracks,
  updateDailyTrack,
} from '../api/dailyTracks'
import type { CSSProperties, PointerEvent as ReactPointerEvent } from 'react'
import { TopicCascadeSelect } from '../components/TopicCascadeSelect'
import { getErrorMessage } from '../api/errors'
import type { DailyTrack } from '../api/generated'
import { DEFAULT_TOPIC_COLOR, listTopics } from '../api/topics'
import type { Topic } from '../api/topics'

const VISIBLE_DAY_COUNT = 4
type ViewMode = 'week' | 'month' | 'year'

type ModalState =
  | { mode: 'create'; day: Date; hour: number }
  | { mode: 'edit'; day: Date; hour: number; track: DailyTrack }

type HoveredBoardCell = {
  day: Date
  hour: number
  entries: DailyTrack[]
}

type DragSelection = {
  day: Date
  dayKey: string
  anchorHour: number
  currentHour: number
}

function parseApiDateTime(value: unknown): Date {
  if (typeof value === 'number') {
    return new Date(value * 1000)
  }
  return new Date(String(value))
}

function toDateKey(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

function startOfDefaultWindow(date: Date): Date {
  const normalized = new Date(date)
  normalized.setHours(0, 0, 0, 0)
  normalized.setDate(normalized.getDate() - 1)
  return normalized
}

function addDays(date: Date, offset: number): Date {
  const shifted = new Date(date)
  shifted.setDate(shifted.getDate() + offset)
  return shifted
}

function addMonths(date: Date, offset: number): Date {
  const shifted = new Date(date)
  shifted.setMonth(shifted.getMonth() + offset)
  return shifted
}

function startOfMonth(date: Date): Date {
  const normalized = new Date(date)
  normalized.setHours(0, 0, 0, 0)
  normalized.setDate(1)
  return normalized
}

function endOfMonth(date: Date): Date {
  const end = new Date(date.getFullYear(), date.getMonth() + 1, 0)
  end.setHours(0, 0, 0, 0)
  return end
}

function startOfYear(year: number): Date {
  const start = new Date(year, 0, 1)
  start.setHours(0, 0, 0, 0)
  return start
}

function endOfYear(year: number): Date {
  const end = new Date(year, 11, 31)
  end.setHours(0, 0, 0, 0)
  return end
}

function enumerateDays(start: Date, end: Date): Date[] {
  const days: Date[] = []
  const cursor = new Date(start)
  cursor.setHours(0, 0, 0, 0)
  while (cursor <= end) {
    days.push(new Date(cursor))
    cursor.setDate(cursor.getDate() + 1)
  }
  return days
}

function weekLabel(weekStart: Date): string {
  const weekEnd = addDays(weekStart, VISIBLE_DAY_COUNT - 1)
  return `${weekStart.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })} - ${weekEnd.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' })}`
}

function slotKey(day: Date, hour: number): string {
  return `${toDateKey(day)}-${String(hour).padStart(2, '0')}`
}

function isSameDate(left: Date, right: Date): boolean {
  return (
    left.getFullYear() === right.getFullYear() &&
    left.getMonth() === right.getMonth() &&
    left.getDate() === right.getDate()
  )
}

function dateFromKey(dayKey: string): Date | null {
  const [yearText, monthText, dayText] = dayKey.split('-')
  const year = Number(yearText)
  const month = Number(monthText)
  const day = Number(dayText)
  if (!Number.isInteger(year) || !Number.isInteger(month) || !Number.isInteger(day)) {
    return null
  }
  return new Date(year, month - 1, day)
}

export function DailyTracksPage() {
  const [tracks, setTracks] = useState<DailyTrack[]>([])
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [weekStart, setWeekStart] = useState(() => startOfDefaultWindow(new Date()))
  const [viewMode, setViewMode] = useState<ViewMode>('week')
  const [monthStart, setMonthStart] = useState(() => startOfMonth(new Date()))
  const [yearValue, setYearValue] = useState(() => new Date().getFullYear())
  const [modalState, setModalState] = useState<ModalState | null>(null)
  const [topicId, setTopicId] = useState('')
  const [comment, setComment] = useState('')
  /** Exclusive end hour on the same day as start (e.g. start 9 + end 14 → slots 9–13). */
  const [endHourExclusive, setEndHourExclusive] = useState('2')
  const [saveError, setSaveError] = useState<string | null>(null)
  const [saving, setSaving] = useState(false)
  const [deletingTrackId, setDeletingTrackId] = useState<number | null>(null)
  const [dragSelection, setDragSelection] = useState<DragSelection | null>(null)
  const [hoveredBoardCell, setHoveredBoardCell] = useState<HoveredBoardCell | null>(null)
  const activePointerIdRef = useRef<number | null>(null)

  const weekDays = useMemo(
    () => Array.from({ length: VISIBLE_DAY_COUNT }, (_, index) => addDays(weekStart, index)),
    [weekStart],
  )
  const scheduleGridStyle = useMemo(
    () =>
      ({
        '--day-count': String(weekDays.length),
      }) as CSSProperties,
    [weekDays.length],
  )
  const today = useMemo(() => {
    const now = new Date()
    now.setHours(0, 0, 0, 0)
    return now
  }, [])

  const visibleRange = useMemo(() => {
    if (viewMode === 'month') {
      const start = startOfMonth(monthStart)
      const end = endOfMonth(start)
      return {
        start,
        end,
        label: start.toLocaleDateString(undefined, { month: 'long', year: 'numeric' }),
      }
    }

    if (viewMode === 'year') {
      const start = startOfYear(yearValue)
      const end = endOfYear(yearValue)
      return {
        start,
        end,
        label: String(yearValue),
      }
    }

    const start = weekStart
    const end = addDays(weekStart, VISIBLE_DAY_COUNT - 1)
    return { start, end, label: weekLabel(weekStart) }
  }, [viewMode, monthStart, yearValue, weekStart])

  const boardDays = useMemo(
    () => enumerateDays(visibleRange.start, visibleRange.end),
    [visibleRange],
  )

  async function loadTracksForRange(start: Date, end: Date) {
    setLoading(true)
    setError(null)

    try {
      const startDate = toDateKey(start)
      const endDate = toDateKey(end)
      const data = await listDailyTracks({ startDate, endDate })
      setTracks(data)
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    async function loadTopicsOnly() {
      try {
        const topicData = await listTopics()
        setTopics(topicData)
      } catch (err) {
        setError(getErrorMessage(err))
      }
    }

    void loadTopicsOnly()
  }, [])

  useEffect(() => {
    void loadTracksForRange(visibleRange.start, visibleRange.end)
  }, [visibleRange])

  useEffect(() => {
    function clearDrag() {
      activePointerIdRef.current = null
      setDragSelection(null)
    }
    window.addEventListener('pointercancel', clearDrag)
    window.addEventListener('blur', clearDrag)
    return () => {
      window.removeEventListener('pointercancel', clearDrag)
      window.removeEventListener('blur', clearDrag)
    }
  }, [])


  const topicNameById = useMemo(() => {
    const map = new Map<number, string>()
    for (const topic of topics) {
      map.set(topic.id, topic.topic_name)
    }
    return map
  }, [topics])

  const topicColorById = useMemo(() => {
    const map = new Map<number, string>()
    for (const topic of topics) {
      map.set(topic.id, topic.display_color)
    }
    return map
  }, [topics])

  const tracksBySlot = useMemo(() => {
    const grouped = new Map<string, DailyTrack[]>()
    for (const track of tracks) {
      const date = parseApiDateTime(track.start_time)
      const key = slotKey(date, date.getHours())
      const bucket = grouped.get(key) ?? []
      bucket.push(track)
      grouped.set(key, bucket)
    }

    for (const [key, bucket] of grouped.entries()) {
      bucket.sort(
        (a, b) =>
          parseApiDateTime(a.start_time).getTime() -
          parseApiDateTime(b.start_time).getTime(),
      )
      grouped.set(key, bucket)
    }

    return grouped
  }, [tracks])

  const mergedSegmentsByDay = useMemo(() => {
    const merged = new Map<
      string,
      Array<{
        track: DailyTrack
        startHour: number
        endHourExclusive: number
      }>
    >()

    for (const day of weekDays) {
      const dayKey = toDateKey(day)
      const segments: Array<{
        track: DailyTrack
        startHour: number
        endHourExclusive: number
      }> = []

      let hour = 0
      while (hour < 24) {
        const slotEntries = tracksBySlot.get(slotKey(day, hour)) ?? []
        if (slotEntries.length !== 1) {
          hour += 1
          continue
        }

        const track = slotEntries[0]
        let endHourExclusive = hour + 1
        while (endHourExclusive < 24) {
          const nextEntries = tracksBySlot.get(slotKey(day, endHourExclusive)) ?? []
          if (nextEntries.length !== 1 || nextEntries[0].topic_id !== track.topic_id) {
            break
          }
          endHourExclusive += 1
        }

        segments.push({ track, startHour: hour, endHourExclusive })
        hour = endHourExclusive
      }

      merged.set(dayKey, segments)
    }

    return merged
  }, [tracksBySlot, weekDays])

  function openCreateModal(day: Date, hour: number, duration = 1) {
    setSaveError(null)
    setTopicId('')
    setComment('')
    const endExclusive = Math.min(hour + duration, 24)
    setEndHourExclusive(String(Math.max(endExclusive, hour + 1)))
    setModalState({ mode: 'create', day, hour })
  }

  function openEditModal(track: DailyTrack) {
    const dt = parseApiDateTime(track.start_time)
    setSaveError(null)
    setTopicId(String(track.topic_id))
    setComment(track.comment ?? '')
    setModalState({ mode: 'edit', day: dt, hour: dt.getHours(), track })
  }

  function closeModal() {
    setModalState(null)
    setTopicId('')
    setComment('')
    setSaveError(null)
  }

  function beginDrag(day: Date, hour: number, pointerId: number) {
    activePointerIdRef.current = pointerId
    setDragSelection({
      day,
      dayKey: toDateKey(day),
      anchorHour: hour,
      currentHour: hour,
    })
  }

  function updateDrag(day: Date, hour: number, pointerId: number) {
    if (activePointerIdRef.current !== pointerId) {
      return
    }
    setDragSelection((current) => {
      if (!current || current.dayKey !== toDateKey(day)) {
        return current
      }
      return { ...current, currentHour: hour }
    })
  }

  function endDrag(day: Date, hour: number, pointerId: number) {
    if (activePointerIdRef.current !== pointerId) {
      return
    }
    activePointerIdRef.current = null
    setDragSelection((current) => {
      if (!current || current.dayKey !== toDateKey(day)) {
        return null
      }
      const startHour = Math.min(current.anchorHour, hour)
      const duration = Math.abs(current.anchorHour - hour) + 1
      openCreateModal(current.day, startHour, duration)
      return null
    })
  }

  function slotFromPointerPosition(clientX: number, clientY: number): { day: Date; hour: number } | null {
    const target = document
      .elementFromPoint(clientX, clientY)
      ?.closest<HTMLButtonElement>('button[data-slot-day][data-slot-hour]')
    if (!target) {
      return null
    }
    const dayKey = target.dataset.slotDay
    const hourText = target.dataset.slotHour
    if (!dayKey || !hourText) {
      return null
    }
    const day = dateFromKey(dayKey)
    const hour = Number(hourText)
    if (!day || !Number.isInteger(hour)) {
      return null
    }
    return { day, hour }
  }

  function onSlotPointerDown(day: Date, hour: number, event: ReactPointerEvent<HTMLButtonElement>) {
    if (event.button !== 0) {
      return
    }
    event.preventDefault()
    beginDrag(day, hour, event.pointerId)
    event.currentTarget.setPointerCapture(event.pointerId)
  }

  function onSlotPointerEnter(day: Date, hour: number, event: ReactPointerEvent<HTMLButtonElement>) {
    updateDrag(day, hour, event.pointerId)
  }

  function onSlotPointerMove(day: Date, hour: number, event: ReactPointerEvent<HTMLButtonElement>) {
    const hoveredSlot = slotFromPointerPosition(event.clientX, event.clientY)
    if (hoveredSlot) {
      updateDrag(hoveredSlot.day, hoveredSlot.hour, event.pointerId)
      return
    }
    updateDrag(day, hour, event.pointerId)
  }

  function onSlotPointerUp(day: Date, hour: number, event: ReactPointerEvent<HTMLButtonElement>) {
    const hoveredSlot = slotFromPointerPosition(event.clientX, event.clientY)
    const finalDay = hoveredSlot?.day ?? day
    const finalHour = hoveredSlot?.hour ?? hour
    endDrag(finalDay, finalHour, event.pointerId)
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId)
    }
  }

  function onSlotPointerCancel(event: ReactPointerEvent<HTMLButtonElement>) {
    if (activePointerIdRef.current !== event.pointerId) {
      return
    }
    activePointerIdRef.current = null
    setDragSelection(null)
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId)
    }
  }

  function isHourSelectedByDrag(day: Date, hour: number): boolean {
    if (!dragSelection || dragSelection.dayKey !== toDateKey(day)) {
      return false
    }
    const { anchorHour, currentHour } = dragSelection
    if (anchorHour === currentHour) {
      return hour === anchorHour
    }
    if (currentHour > anchorHour) {
      return hour >= anchorHour && hour <= currentHour
    }
    return hour >= currentHour && hour <= anchorHour
  }


  async function submitModal() {
    if (!modalState) {
      return
    }

    setSaveError(null)
    const parsedTopicId = topicId ? Number(topicId) : undefined
    if (parsedTopicId === undefined || Number.isNaN(parsedTopicId)) {
      setSaveError('topic is required')
      return
    }

    setSaving(true)
    try {
      if (modalState.mode === 'create') {
        const startHour = modalState.hour
        const parsedEndExclusive = Number(endHourExclusive)
        if (
          !Number.isInteger(parsedEndExclusive) ||
          parsedEndExclusive <= startHour ||
          parsedEndExclusive > 24
        ) {
          setSaveError('end time must be after start and on the same day (before midnight)')
          setSaving(false)
          return
        }

        const parsedDuration = parsedEndExclusive - startHour

        const conflictHour = Array.from({ length: parsedDuration }, (_, offset) => {
          const slotDateTime = new Date(modalState.day)
          slotDateTime.setHours(modalState.hour + offset, 0, 0, 0)
          return {
            key: slotKey(slotDateTime, slotDateTime.getHours()),
            label: `${slotDateTime.toLocaleDateString()} ${String(slotDateTime.getHours()).padStart(2, '0')}:00`,
          }
        }).find(({ key }) => (tracksBySlot.get(key) ?? []).length > 0)

        if (conflictHour) {
          setSaveError(`slot already occupied: ${conflictHour.label}`)
          setSaving(false)
          return
        }

        const slotDateTime = new Date(modalState.day)
        slotDateTime.setHours(modalState.hour, 0, 0, 0)

        for (let offset = 0; offset < parsedDuration; offset += 1) {
          const start = new Date(slotDateTime)
          start.setHours(start.getHours() + offset)
          await createDailyTrack({
            startTime: start.toISOString(),
            topicId: parsedTopicId,
            comment: comment.trim() || undefined,
          })
        }
      } else {
        await updateDailyTrack(modalState.track.id, {
          topicId: parsedTopicId,
          comment: comment.trim() || undefined,
        })
      }

      closeModal()
      await loadTracksForRange(visibleRange.start, visibleRange.end)
    } catch (err) {
      setSaveError(getErrorMessage(err))
    } finally {
      setSaving(false)
    }
  }

  async function onDeleteTrack() {
    if (!modalState || modalState.mode !== 'edit') {
      return
    }

    setSaveError(null)
    setSaving(true)
    try {
      await deleteDailyTrack(modalState.track.id)
      closeModal()
      await loadTracksForRange(visibleRange.start, visibleRange.end)
    } catch (err) {
      setSaveError(getErrorMessage(err))
    } finally {
      setSaving(false)
    }
  }

  async function deleteTrackQuick(track: DailyTrack) {
    const confirmDelete = window.confirm('Delete this hourly track?')
    if (!confirmDelete) {
      return
    }

    setError(null)
    setDeletingTrackId(track.id)
    try {
      await deleteDailyTrack(track.id)
      await loadTracksForRange(visibleRange.start, visibleRange.end)
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setDeletingTrackId(null)
    }
  }

  function goPreviousRange() {
    if (viewMode === 'month') {
      setMonthStart((current) => startOfMonth(addMonths(current, -1)))
      return
    }
    if (viewMode === 'year') {
      setYearValue((current) => current - 1)
      return
    }
    setWeekStart((current) => addDays(current, -VISIBLE_DAY_COUNT))
  }

  function goTodayRange() {
    if (viewMode === 'month') {
      setMonthStart(startOfMonth(new Date()))
      return
    }
    if (viewMode === 'year') {
      setYearValue(new Date().getFullYear())
      return
    }
    setWeekStart(startOfDefaultWindow(new Date()))
  }

  function goNextRange() {
    if (viewMode === 'month') {
      setMonthStart((current) => startOfMonth(addMonths(current, 1)))
      return
    }
    if (viewMode === 'year') {
      setYearValue((current) => current + 1)
      return
    }
    setWeekStart((current) => addDays(current, VISIBLE_DAY_COUNT))
  }

  return (
    <section className="page daily-tracks-page">
      <h2>Daily Tracks</h2>

      <div className="panel calendar-panel">
        <div className="week-head">
          <h3>Calendar Board</h3>
          <p className="calendar-month">{visibleRange.label}</p>
        </div>
        <div className="view-switch" role="tablist" aria-label="View mode">
          <button
            type="button"
            role="tab"
            aria-selected={viewMode === 'week'}
            className={`view-switch-btn ${viewMode === 'week' ? 'active' : ''}`}
            onClick={() => setViewMode('week')}
          >
            Week
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={viewMode === 'month'}
            className={`view-switch-btn ${viewMode === 'month' ? 'active' : ''}`}
            onClick={() => setViewMode('month')}
          >
            Month
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={viewMode === 'year'}
            className={`view-switch-btn ${viewMode === 'year' ? 'active' : ''}`}
            onClick={() => setViewMode('year')}
          >
            Year
          </button>
        </div>
        <div className="calendar-toolbar">
          <button
            type="button"
            className="toolbar-btn"
            onClick={goPreviousRange}
          >
            Previous
          </button>
          <button
            type="button"
            className="toolbar-btn toolbar-btn-primary"
            onClick={goTodayRange}
          >
            {viewMode === 'week' ? 'Today Window' : viewMode === 'month' ? 'This Month' : 'This Year'}
          </button>
          <button
            type="button"
            className="toolbar-btn"
            onClick={goNextRange}
          >
            Next
          </button>
        </div>
        {loading && <p>Loading...</p>}
        {error && <p className="error">{error}</p>}
        {!loading && !error && (
          <>
            {viewMode === 'week' ? (
              <>
                <div className="schedule-grid schedule-head" style={scheduleGridStyle}>
                  <span className="hour-cell" />
                  {weekDays.map((day) => (
                    <span
                      key={toDateKey(day)}
                      className={`day-cell ${isSameDate(day, today) ? 'is-today' : ''}`}
                    >
                      {day.toLocaleDateString(undefined, { weekday: 'short' })} {day.getDate()}
                    </span>
                  ))}
                </div>
                <div className={`schedule-body ${dragSelection ? 'is-dragging' : ''}`}>
                  <div className="schedule-board" style={scheduleGridStyle}>
                    <div className="hour-lane">
                      {Array.from({ length: 24 }, (_, hour) => (
                        <span key={`hour-${hour}`} className="hour-cell">
                          {String(hour).padStart(2, '0')}:00
                        </span>
                      ))}
                    </div>
                    {weekDays.map((day) => {
                      const dayKey = toDateKey(day)
                      const mergedSegments = mergedSegmentsByDay.get(dayKey) ?? []

                      return (
                        <div
                          key={dayKey}
                          className={`day-lane ${isSameDate(day, today) ? 'today-slot' : ''}`}
                        >
                          {Array.from({ length: 24 }, (_, hour) => {
                            const entries = tracksBySlot.get(slotKey(day, hour)) ?? []
                            if (entries.length === 0) {
                              return (
                                <button
                                  type="button"
                                  key={slotKey(day, hour)}
                                  data-slot-day={dayKey}
                                  data-slot-hour={String(hour)}
                                  className={`slot empty-slot ${
                                    isHourSelectedByDrag(day, hour) ? 'drag-selected' : ''
                                  }`}
                                  onPointerDown={(event) => onSlotPointerDown(day, hour, event)}
                                  onPointerEnter={(event) => onSlotPointerEnter(day, hour, event)}
                                  onPointerMove={(event) => onSlotPointerMove(day, hour, event)}
                                  onPointerUp={(event) => onSlotPointerUp(day, hour, event)}
                                  onPointerCancel={onSlotPointerCancel}
                                >
                                  +
                                </button>
                              )
                            }

                            if (entries.length === 1) {
                              const track = entries[0]
                              const isDeleting = deletingTrackId === track.id
                              const topicColor =
                                topicColorById.get(track.topic_id) ?? DEFAULT_TOPIC_COLOR
                              return (
                                <div
                                  key={slotKey(day, hour)}
                                  className="slot occupied-slot"
                                  style={{ '--topic-color': topicColor } as CSSProperties}
                                >
                                  <button
                                    type="button"
                                    className="occupied-slot-edit-hit"
                                    onClick={() => openEditModal(track)}
                                    aria-label={`Edit track at ${String(hour).padStart(2, '0')}:00`}
                                    title="Edit this hour"
                                  />
                                  <button
                                    type="button"
                                    className="occupied-slot-delete"
                                    disabled={isDeleting}
                                    onClick={(event) => {
                                      event.stopPropagation()
                                      void deleteTrackQuick(track)
                                    }}
                                    aria-label={`Delete track at ${String(hour).padStart(2, '0')}:00`}
                                    title="Delete this hour"
                                  >
                                    {isDeleting ? '…' : '×'}
                                  </button>
                                </div>
                              )
                            }

                            return (
                              <div key={slotKey(day, hour)} className="slot filled-slot multi-slot">
                                {entries.map((track) => {
                                  const topicColor =
                                    topicColorById.get(track.topic_id) ?? DEFAULT_TOPIC_COLOR
                                  return (
                                    <button
                                      key={track.id}
                                      type="button"
                                      className="slot-item slot-item-button"
                                      style={{ '--topic-color': topicColor } as CSSProperties}
                                      onClick={() => openEditModal(track)}
                                    >
                                      <strong>
                                        {parseApiDateTime(track.start_time).toLocaleTimeString([], {
                                          hour: '2-digit',
                                          minute: '2-digit',
                                        })}
                                      </strong>
                                      <span>{topicNameById.get(track.topic_id) ?? 'Topic'}</span>
                                    </button>
                                  )
                                })}
                              </div>
                            )
                          })}
                          <div className="merged-overlay">
                            {mergedSegments.map(({ track, startHour, endHourExclusive }) => {
                              const topicColor =
                                topicColorById.get(track.topic_id) ?? DEFAULT_TOPIC_COLOR
                              const durationHours = endHourExclusive - startHour
                              const isCompact = durationHours <= 1
                              return (
                                <div
                                  key={`${track.id}-${startHour}-${endHourExclusive}`}
                                  className={`slot-item slot-item-button merged-track-item ${
                                    isCompact ? 'compact' : ''
                                  }`}
                                  style={
                                    {
                                      '--topic-color': topicColor,
                                      top: `${startHour * 64}px`,
                                      height: `${durationHours * 64}px`,
                                    } as CSSProperties
                                  }
                                >
                                  <strong>
                                    {String(startHour).padStart(2, '0')}:00 -{' '}
                                    {String(endHourExclusive).padStart(2, '0')}:00
                                  </strong>
                                  <span>{topicNameById.get(track.topic_id) ?? 'Topic'}</span>
                                  {!isCompact && (
                                    <span className="track-merge-badge">
                                      {durationHours} {durationHours === 1 ? 'hour' : 'hours'}
                                    </span>
                                  )}
                                </div>
                              )
                            })}
                          </div>
                        </div>
                      )
                    })}
                  </div>
                </div>
                {tracks.length === 0 && (
                  <p className="empty-hint">No records yet. Click any empty hour slot to create one.</p>
                )}
              </>
            ) : (
              <>
                <div className="hour-matrix-wrap">
                  <div className="hour-matrix">
                    <span className="matrix-corner" />
                    {Array.from({ length: 24 }, (_, hour) => (
                      <span key={`head-${hour}`} className="matrix-hour-head">
                        {String(hour).padStart(2, '0')}
                      </span>
                    ))}

                    {boardDays.map((day) => {
                      const dayKey = toDateKey(day)
                      return (
                        <div key={`row-${dayKey}`} className="matrix-row-fragment">
                          <span className={`matrix-day-label ${isSameDate(day, today) ? 'is-today' : ''}`}>
                            {day.toLocaleDateString(undefined, {
                              month: 'short',
                              day: 'numeric',
                            })}
                          </span>
                          {Array.from({ length: 24 }, (_, hour) => {
                            const entries = tracksBySlot.get(slotKey(day, hour)) ?? []
                            const uniqueColors = Array.from(
                              new Set(
                                entries.map((item) => topicColorById.get(item.topic_id) ?? DEFAULT_TOPIC_COLOR),
                              ),
                            )
                            const colorPaint =
                              uniqueColors.length <= 1
                                ? (uniqueColors[0] ?? '#f1f5f9')
                                : `linear-gradient(135deg, ${uniqueColors
                                    .map((color, index) => {
                                      const start = (index / uniqueColors.length) * 100
                                      const end = ((index + 1) / uniqueColors.length) * 100
                                      return `${color} ${start}% ${end}%`
                                    })
                                    .join(', ')})`

                            return (
                              <button
                                type="button"
                                key={`${dayKey}-${hour}`}
                                className={`matrix-cell ${entries.length > 0 ? 'has-data' : 'is-empty'}`}
                                style={{ '--cell-color': colorPaint } as CSSProperties}
                                onMouseEnter={() => setHoveredBoardCell({ day, hour, entries })}
                                onFocus={() => setHoveredBoardCell({ day, hour, entries })}
                                onMouseLeave={() =>
                                  setHoveredBoardCell((current) =>
                                    current && current.day.getTime() === day.getTime() && current.hour === hour
                                      ? null
                                      : current,
                                  )
                                }
                                onBlur={() =>
                                  setHoveredBoardCell((current) =>
                                    current && current.day.getTime() === day.getTime() && current.hour === hour
                                      ? null
                                      : current,
                                  )
                                }
                                onClick={() => {
                                  if (entries.length === 0) {
                                    openCreateModal(day, hour, 1)
                                    return
                                  }
                                  openEditModal(entries[0])
                                }}
                                aria-label={`${day.toLocaleDateString()} ${String(hour).padStart(2, '0')}:00`}
                              />
                            )
                          })}
                        </div>
                      )
                    })}
                  </div>
                </div>
                <div className="matrix-hover-panel" aria-live="polite">
                  {hoveredBoardCell ? (
                    <>
                      <p className="matrix-hover-title">
                        {hoveredBoardCell.day.toLocaleDateString(undefined, {
                          month: 'short',
                          day: 'numeric',
                          year: 'numeric',
                        })}{' '}
                        {String(hoveredBoardCell.hour).padStart(2, '0')}:00
                      </p>
                      {hoveredBoardCell.entries.length === 0 ? (
                        <p>No track in this hour.</p>
                      ) : (
                        <p>
                          {hoveredBoardCell.entries
                            .map((track) => topicNameById.get(track.topic_id) ?? 'Topic')
                            .join(' / ')}
                        </p>
                      )}
                    </>
                  ) : (
                    <p className="density-hint">Hover any cell to preview details.</p>
                  )}
                </div>
                {tracks.length === 0 && (
                  <p className="empty-hint">No records yet. Click any hour cell to create one.</p>
                )}
              </>
            )}
          </>
        )}
      </div>

      {modalState && (
        <div className="modal-backdrop" role="presentation">
          <div className="modal-panel" role="dialog" aria-modal="true">
            <h3>
              {modalState.mode === 'create'
                ? 'Create Daily Track'
                : 'Edit Daily Track'}
            </h3>
            <p className="modal-meta">
              {modalState.mode === 'create' ? (
                <>
                  {modalState.day.toLocaleDateString()}{' '}
                  <strong>
                    {String(modalState.hour).padStart(2, '0')}:00
                  </strong>
                  {' → '}
                  <strong>{String(endHourExclusive).padStart(2, '0')}:00</strong>
                  <span className="modal-meta-hint">
                    {' '}
                    (
                    {Math.max(0, Number(endHourExclusive) - modalState.hour) || '—'}{' '}
                    {Number(endHourExclusive) - modalState.hour === 1 ? 'hour' : 'hours'})
                  </span>
                </>
              ) : (
                <>
                  {modalState.day.toLocaleDateString()}{' '}
                  {String(modalState.hour).padStart(2, '0')}:00
                </>
              )}
            </p>
            <label>
              Topic
              <TopicCascadeSelect topics={topics} value={topicId} onChange={setTopicId} />
            </label>
            {modalState.mode === 'create' && (
              <label>
                End time
                <select
                  value={endHourExclusive}
                  onChange={(event) => setEndHourExclusive(event.target.value)}
                >
                  {Array.from({ length: 24 - modalState.hour }, (_, index) => {
                    const endH = modalState.hour + 1 + index
                    const span = endH - modalState.hour
                    return (
                      <option key={endH} value={String(endH)}>
                        {String(endH).padStart(2, '0')}:00 ({span}{' '}
                        {span === 1 ? 'hour' : 'hours'})
                      </option>
                    )
                  })}
                </select>
                <span className="field-hint">The end hour is not included (same as drag-select).</span>
              </label>
            )}
            <label>
              Comment (optional)
              <textarea
                value={comment}
                onChange={(event) => setComment(event.target.value)}
                rows={3}
              />
            </label>
            {saveError && <p className="error">{saveError}</p>}
            <div className="modal-actions">
              <button type="button" onClick={closeModal}>
                Cancel
              </button>
              {modalState.mode === 'edit' && (
                <button
                  type="button"
                  disabled={saving}
                  className="danger-button"
                  onClick={onDeleteTrack}
                >
                  {saving ? 'Deleting...' : 'Delete'}
                </button>
              )}
              <button type="button" disabled={saving} onClick={submitModal}>
                {saving
                  ? modalState.mode === 'create'
                    ? 'Creating...'
                    : 'Saving...'
                  : modalState.mode === 'create'
                    ? 'Create Track'
                    : 'Save Changes'}
              </button>
            </div>
          </div>
        </div>
      )}
    </section>
  )
}
