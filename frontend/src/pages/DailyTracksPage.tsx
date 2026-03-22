import { useEffect, useMemo, useState } from 'react'

import {
  createDailyTrack,
  deleteDailyTrack,
  listDailyTracks,
  updateDailyTrack,
} from '../api/dailyTracks'
import type { CSSProperties } from 'react'
import { TopicCascadeSelect } from '../components/TopicCascadeSelect'
import { getErrorMessage } from '../api/errors'
import type { DailyTrack } from '../api/generated'
import { DEFAULT_TOPIC_COLOR, listTopics } from '../api/topics'
import type { Topic } from '../api/topics'

const VISIBLE_DAY_COUNT = 4

type ModalState =
  | { mode: 'create'; day: Date; hour: number }
  | { mode: 'edit'; day: Date; hour: number; track: DailyTrack }

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

export function DailyTracksPage() {
  const [tracks, setTracks] = useState<DailyTrack[]>([])
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [weekStart, setWeekStart] = useState(() => startOfDefaultWindow(new Date()))
  const [modalState, setModalState] = useState<ModalState | null>(null)
  const [topicId, setTopicId] = useState('')
  const [comment, setComment] = useState('')
  /** Exclusive end hour on the same day as start (e.g. start 9 + end 14 → slots 9–13). */
  const [endHourExclusive, setEndHourExclusive] = useState('2')
  const [saveError, setSaveError] = useState<string | null>(null)
  const [saving, setSaving] = useState(false)
  const [dragSelection, setDragSelection] = useState<DragSelection | null>(null)

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

  async function loadTracksForWeek(targetWeekStart: Date) {
    setLoading(true)
    setError(null)

    try {
      const startDate = toDateKey(targetWeekStart)
      const endDate = toDateKey(addDays(targetWeekStart, VISIBLE_DAY_COUNT - 1))
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
    void loadTracksForWeek(weekStart)
  }, [weekStart])

  useEffect(() => {
    function clearDrag() {
      setDragSelection(null)
    }
    window.addEventListener('mouseup', clearDrag)
    return () => window.removeEventListener('mouseup', clearDrag)
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

  function beginDrag(day: Date, hour: number) {
    setDragSelection({
      day,
      dayKey: toDateKey(day),
      anchorHour: hour,
      currentHour: hour,
    })
  }

  function updateDrag(day: Date, hour: number) {
    setDragSelection((current) => {
      if (!current || current.dayKey !== toDateKey(day)) {
        return current
      }
      return { ...current, currentHour: hour }
    })
  }

  function endDrag(day: Date, hour: number) {
    setDragSelection((current) => {
      if (!current || current.dayKey !== toDateKey(day)) {
        return null
      }

      const startHour = Math.min(current.anchorHour, hour)
      const duration =
        current.anchorHour === hour ? 1 : Math.abs(current.anchorHour - hour)
      openCreateModal(current.day, startHour, duration)
      return null
    })
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
      return hour >= anchorHour && hour < currentHour
    }

    return hour >= currentHour && hour < anchorHour
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
      await loadTracksForWeek(weekStart)
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
      await loadTracksForWeek(weekStart)
    } catch (err) {
      setSaveError(getErrorMessage(err))
    } finally {
      setSaving(false)
    }
  }

  return (
    <section className="page">
      <h2>Daily Tracks</h2>

      <div className="panel">
        <div className="week-head">
          <h3>Calendar Board</h3>
          <p className="calendar-month">{weekLabel(weekStart)}</p>
        </div>
        <div className="calendar-toolbar">
          <button type="button" onClick={() => setWeekStart(addDays(weekStart, -VISIBLE_DAY_COUNT))}>
            Previous
          </button>
          <button type="button" onClick={() => setWeekStart(startOfDefaultWindow(new Date()))}>
            Today Window
          </button>
          <button type="button" onClick={() => setWeekStart(addDays(weekStart, VISIBLE_DAY_COUNT))}>
            Next
          </button>
        </div>
        {loading && <p>Loading...</p>}
        {error && <p className="error">{error}</p>}
        {!loading && !error && (
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
            <div className="schedule-body">
              {Array.from({ length: 24 }, (_, hour) => (
                <div key={hour} className="schedule-grid hour-row" style={scheduleGridStyle}>
                  <span className="hour-cell">
                    {String(hour).padStart(2, '0')}:00
                  </span>
                  {weekDays.map((day) => {
                    const entries = tracksBySlot.get(slotKey(day, hour)) ?? []
                    if (entries.length === 0) {
                      return (
                        <button
                          type="button"
                          key={slotKey(day, hour)}
                          className={`slot empty-slot ${
                            isSameDate(day, today) ? 'today-slot' : ''
                          } ${
                            isHourSelectedByDrag(day, hour) ? 'drag-selected' : ''
                          }`}
                          onMouseDown={() => beginDrag(day, hour)}
                          onMouseEnter={() => updateDrag(day, hour)}
                          onMouseUp={() => endDrag(day, hour)}
                        >
                          +
                        </button>
                      )
                    }

                    return (
                      <div
                        key={slotKey(day, hour)}
                        className={`slot filled-slot ${isSameDate(day, today) ? 'today-slot' : ''}`}
                      >
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
                </div>
              ))}
            </div>
            {tracks.length === 0 && (
              <p className="empty-hint">
                No records yet. Click any empty hour slot to create one.
              </p>
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
