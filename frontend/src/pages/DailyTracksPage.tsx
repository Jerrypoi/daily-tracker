import { useEffect, useMemo, useState } from 'react'

import {
  createDailyTrack,
  deleteDailyTrack,
  listDailyTracks,
  updateDailyTrack,
} from '../api/dailyTracks'
import { getErrorMessage } from '../api/errors'
import type { DailyTrack, Topic } from '../api/generated'
import { listTopics } from '../api/topics'

const weekdayLabels = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']

type ModalState =
  | { mode: 'create'; day: Date; hour: number }
  | { mode: 'edit'; day: Date; hour: number; track: DailyTrack }

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

function startOfWeek(date: Date): Date {
  const normalized = new Date(date)
  normalized.setHours(0, 0, 0, 0)
  const day = normalized.getDay()
  const distanceFromMonday = (day + 6) % 7
  normalized.setDate(normalized.getDate() - distanceFromMonday)
  return normalized
}

function addDays(date: Date, offset: number): Date {
  const shifted = new Date(date)
  shifted.setDate(shifted.getDate() + offset)
  return shifted
}

function weekLabel(weekStart: Date): string {
  const weekEnd = addDays(weekStart, 6)
  return `${weekStart.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })} - ${weekEnd.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' })}`
}

function slotKey(day: Date, hour: number): string {
  return `${toDateKey(day)}-${String(hour).padStart(2, '0')}`
}

export function DailyTracksPage() {
  const [tracks, setTracks] = useState<DailyTrack[]>([])
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [weekStart, setWeekStart] = useState(() => startOfWeek(new Date()))
  const [modalState, setModalState] = useState<ModalState | null>(null)
  const [topicId, setTopicId] = useState('')
  const [comment, setComment] = useState('')
  const [durationHours, setDurationHours] = useState('1')
  const [saveError, setSaveError] = useState<string | null>(null)
  const [saving, setSaving] = useState(false)

  const weekDays = useMemo(
    () => Array.from({ length: 7 }, (_, index) => addDays(weekStart, index)),
    [weekStart],
  )

  async function loadTracksForWeek(targetWeekStart: Date) {
    setLoading(true)
    setError(null)

    try {
      const startDate = toDateKey(targetWeekStart)
      const endDate = toDateKey(addDays(targetWeekStart, 6))
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

  const topicNameById = useMemo(() => {
    const map = new Map<number, string>()
    for (const topic of topics) {
      map.set(topic.id, topic.topic_name)
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

  function openCreateModal(day: Date, hour: number) {
    setSaveError(null)
    setTopicId('')
    setComment('')
    setDurationHours('1')
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
        const parsedDuration = Number(durationHours)
        if (!Number.isInteger(parsedDuration) || parsedDuration < 1) {
          setSaveError('duration must be at least 1 hour')
          setSaving(false)
          return
        }

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
          <button type="button" onClick={() => setWeekStart(addDays(weekStart, -7))}>
            Previous Week
          </button>
          <button type="button" onClick={() => setWeekStart(startOfWeek(new Date()))}>
            This Week
          </button>
          <button type="button" onClick={() => setWeekStart(addDays(weekStart, 7))}>
            Next Week
          </button>
        </div>
        {loading && <p>Loading...</p>}
        {error && <p className="error">{error}</p>}
        {!loading && !error && (
          <>
            <div className="schedule-grid schedule-head">
              <span className="hour-cell" />
              {weekDays.map((day, index) => (
                <span key={toDateKey(day)} className="day-cell">
                  {weekdayLabels[index]} {day.getDate()}
                </span>
              ))}
            </div>
            <div className="schedule-body">
              {Array.from({ length: 24 }, (_, hour) => (
                <div key={hour} className="schedule-grid hour-row">
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
                          className="slot empty-slot"
                          onClick={() => openCreateModal(day, hour)}
                        >
                          +
                        </button>
                      )
                    }

                    return (
                      <div key={slotKey(day, hour)} className="slot filled-slot">
                        {entries.map((track) => (
                          <button
                            key={track.id}
                            type="button"
                            className="slot-item slot-item-button"
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
                        ))}
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
              {modalState.day.toLocaleDateString()} {String(modalState.hour).padStart(2, '0')}
              :00
            </p>
            <label>
              Topic
              <select value={topicId} onChange={(event) => setTopicId(event.target.value)}>
                <option value="">Select a topic</option>
                {topics.map((topic) => (
                  <option key={topic.id} value={String(topic.id)}>
                    {topic.topic_name}
                  </option>
                ))}
              </select>
            </label>
            {modalState.mode === 'create' && (
              <label>
                Duration
                <select
                  value={durationHours}
                  onChange={(event) => setDurationHours(event.target.value)}
                >
                  <option value="1">1 hour</option>
                  <option value="2">2 hours</option>
                  <option value="3">3 hours</option>
                  <option value="4">4 hours</option>
                  <option value="6">6 hours</option>
                  <option value="8">8 hours</option>
                </select>
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
