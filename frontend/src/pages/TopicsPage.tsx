import { useEffect, useState } from 'react'
import type { FormEvent } from 'react'

import { getErrorMessage } from '../api/errors'
import { createTopic, listTopics } from '../api/topics'
import type { Topic } from '../api/generated'

export function TopicsPage() {
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [topicName, setTopicName] = useState('')
  const [parentTopicId, setParentTopicId] = useState('')
  const [saving, setSaving] = useState(false)
  const [saveError, setSaveError] = useState<string | null>(null)

  async function loadTopics() {
    setLoading(true)
    setError(null)
    try {
      const data = await listTopics()
      setTopics(data)
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    void loadTopics()
  }, [])

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setSaveError(null)
    setSaving(true)

    const parsedParentTopicId = parentTopicId ? Number(parentTopicId) : undefined

    try {
      await createTopic({
        topicName: topicName.trim(),
        parentTopicId: parsedParentTopicId,
      })
      setTopicName('')
      setParentTopicId('')
      await loadTopics()
    } catch (err) {
      setSaveError(getErrorMessage(err))
    } finally {
      setSaving(false)
    }
  }

  return (
    <section className="page">
      <h2>Topics</h2>
      <form className="panel" onSubmit={onSubmit}>
        <h3>Create Topic</h3>
        <label>
          Topic Name
          <input
            required
            value={topicName}
            onChange={(event) => setTopicName(event.target.value)}
            placeholder="working"
          />
        </label>
        <label>
          Parent Topic (optional)
          <select
            value={parentTopicId}
            onChange={(event) => setParentTopicId(event.target.value)}
          >
            <option value="">None</option>
            {topics.map((topic) => (
              <option key={topic.id} value={String(topic.id)}>
                {topic.topic_name}
              </option>
            ))}
          </select>
        </label>
        <button type="submit" disabled={saving}>
          {saving ? 'Saving...' : 'Create Topic'}
        </button>
        {saveError && <p className="error">{saveError}</p>}
      </form>

      <div className="panel">
        <h3>Topic List</h3>
        {loading && <p>Loading...</p>}
        {error && <p className="error">{error}</p>}
        {!loading && !error && (
          <ul className="list">
            {topics.map((topic) => (
              <li key={topic.id}>
                <strong>{topic.topic_name}</strong>
              </li>
            ))}
            {topics.length === 0 && <li>No topics yet.</li>}
          </ul>
        )}
      </div>
    </section>
  )
}
