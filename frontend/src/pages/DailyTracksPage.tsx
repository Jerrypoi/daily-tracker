import { useEffect, useState } from 'react'
import type { FormEvent } from 'react'

import {
  createDailyTrack,
  isHalfHourBoundary,
  listDailyTracks,
} from '../api/dailyTracks'
import { getErrorMessage } from '../api/errors'
import type { DailyTrack, Topic } from '../api/generated'
import { listTopics } from '../api/topics'

export function DailyTracksPage() {
  const [tracks, setTracks] = useState<DailyTrack[]>([])
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [startDateFilter, setStartDateFilter] = useState('')
  const [endDateFilter, setEndDateFilter] = useState('')
  const [topicIdFilter, setTopicIdFilter] = useState('')

  const [startTime, setStartTime] = useState('')
  const [topicId, setTopicId] = useState('')
  const [comment, setComment] = useState('')
  const [saveError, setSaveError] = useState<string | null>(null)
  const [saving, setSaving] = useState(false)

  function getCurrentFilter() {
    const parsedTopicId = topicIdFilter.trim() ? Number(topicIdFilter) : undefined
    return {
      startDate: startDateFilter || undefined,
      endDate: endDateFilter || undefined,
      topicId:
        parsedTopicId !== undefined && Number.isNaN(parsedTopicId)
          ? undefined
          : parsedTopicId,
    }
  }

  async function loadTracks(
    filter: {
      startDate?: string
      endDate?: string
      topicId?: number
    } = {},
  ) {
    setLoading(true)
    setError(null)

    try {
      const data = await listDailyTracks(filter)
      setTracks(data)
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    async function bootstrap() {
      try {
        const topicData = await listTopics()
        setTopics(topicData)
      } catch (err) {
        setError(getErrorMessage(err))
      }
      await loadTracks({})
    }

    void bootstrap()
  }, [])

  async function onFilterSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    await loadTracks(getCurrentFilter())
  }

  async function onCreateSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setSaveError(null)

    if (!startTime) {
      setSaveError('start_time is required')
      return
    }

    if (!isHalfHourBoundary(startTime)) {
      setSaveError('start_time must be at :00 or :30')
      return
    }

    const parsedTopicId = topicId.trim() ? Number(topicId) : undefined
    if (parsedTopicId === undefined || Number.isNaN(parsedTopicId)) {
      setSaveError('topic_id is required')
      return
    }

    setSaving(true)
    const isoStartTime = new Date(startTime).toISOString()

    try {
      await createDailyTrack({
        startTime: isoStartTime,
        topicId: parsedTopicId,
        comment: comment.trim() || undefined,
      })
      setStartTime('')
      setTopicId('')
      setComment('')
      await loadTracks(getCurrentFilter())
    } catch (err) {
      setSaveError(getErrorMessage(err))
    } finally {
      setSaving(false)
    }
  }

  return (
    <section className="page">
      <h2>Daily Tracks</h2>

      <form className="panel" onSubmit={onFilterSubmit}>
        <h3>Filters</h3>
        <label>
          Start Date
          <input
            type="date"
            value={startDateFilter}
            onChange={(event) => setStartDateFilter(event.target.value)}
          />
        </label>
        <label>
          End Date
          <input
            type="date"
            value={endDateFilter}
            onChange={(event) => setEndDateFilter(event.target.value)}
          />
        </label>
        <label>
          Topic
          <select
            value={topicIdFilter}
            onChange={(event) => setTopicIdFilter(event.target.value)}
          >
            <option value="">All topics</option>
            {topics.map((topic) => (
              <option key={topic.id} value={String(topic.id)}>
                {topic.topic_name}
              </option>
            ))}
          </select>
        </label>
        <button type="submit">Apply Filters</button>
      </form>

      <form className="panel" onSubmit={onCreateSubmit}>
        <h3>Create Daily Track</h3>
        <label>
          Start Time
          <input
            type="datetime-local"
            required
            value={startTime}
            onChange={(event) => setStartTime(event.target.value)}
          />
        </label>
        <label>
          Topic
          <select
            required
            value={topicId}
            onChange={(event) => setTopicId(event.target.value)}
          >
            <option value="">Select a topic</option>
            {topics.map((topic) => (
              <option key={topic.id} value={String(topic.id)}>
                {topic.topic_name}
              </option>
            ))}
          </select>
        </label>
        <label>
          Comment (optional)
          <textarea
            value={comment}
            onChange={(event) => setComment(event.target.value)}
            rows={3}
          />
        </label>
        <button type="submit" disabled={saving}>
          {saving ? 'Saving...' : 'Create Daily Track'}
        </button>
        {saveError && <p className="error">{saveError}</p>}
      </form>

      <div className="panel">
        <h3>Daily Track List</h3>
        {loading && <p>Loading...</p>}
        {error && <p className="error">{error}</p>}
        {!loading && !error && (
          <ul className="list">
            {tracks.map((track) => (
              <li key={track.id}>
                <strong>{new Date(track.start_time).toLocaleString()}</strong>
                {track.comment && <span> - {track.comment}</span>}
              </li>
            ))}
            {tracks.length === 0 && <li>No records found.</li>}
          </ul>
        )}
      </div>
    </section>
  )
}
