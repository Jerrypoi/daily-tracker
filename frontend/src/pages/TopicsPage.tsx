import { useEffect, useRef, useState } from 'react'
import type { FormEvent } from 'react'

import { getErrorMessage } from '../api/errors'
import { DEFAULT_TOPIC_COLOR, createTopic, listTopics, updateTopic } from '../api/topics'
import type { Topic } from '../api/topics'
import { TopicCascadeSelect } from '../components/TopicCascadeSelect'
import { TopicColorPicker } from '../components/TopicColorPicker'

type TopicTreeNode = {
  topic: Topic
  children: TopicTreeNode[]
}

export function TopicsPage() {
  const [topics, setTopics] = useState<Topic[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [topicName, setTopicName] = useState('')
  const [topicColor, setTopicColor] = useState(DEFAULT_TOPIC_COLOR)
  const [parentTopicId, setParentTopicId] = useState('')
  const [saving, setSaving] = useState(false)
  const [saveError, setSaveError] = useState<string | null>(null)
  const [expandedTopicIds, setExpandedTopicIds] = useState<Set<number>>(new Set())
  const [editingTopic, setEditingTopic] = useState<Topic | null>(null)
  const [editingName, setEditingName] = useState('')
  const [editingColor, setEditingColor] = useState(DEFAULT_TOPIC_COLOR)
  const [editing, setEditing] = useState(false)
  const [editError, setEditError] = useState<string | null>(null)
  const topicColorRef = useRef(DEFAULT_TOPIC_COLOR)
  const editingColorRef = useRef(DEFAULT_TOPIC_COLOR)

  async function loadTopics() {
    setLoading(true)
    setError(null)
    try {
      const data = await listTopics()
      setTopics(data)
      setExpandedTopicIds((current) => {
        if (current.size > 0) {
          return current
        }
        return new Set(data.map((topic) => topic.id))
      })
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    void loadTopics()
  }, [])

  useEffect(() => {
    if (!editingTopic) {
      return
    }

    function onKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        setEditingTopic(null)
        setEditError(null)
      }
    }

    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [editingTopic])

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setSaveError(null)
    setSaving(true)

    const parsedParentTopicId = parentTopicId ? Number(parentTopicId) : undefined
    const parsedColor = /^#[0-9a-fA-F]{6}$/.test(topicColorRef.current)
      ? topicColorRef.current.toLowerCase()
      : DEFAULT_TOPIC_COLOR

    try {
      await createTopic({
        topicName: topicName.trim(),
        parentTopicId: parsedParentTopicId,
        displayColor: parsedColor,
      })
      setTopicName('')
      setTopicColor(DEFAULT_TOPIC_COLOR)
      topicColorRef.current = DEFAULT_TOPIC_COLOR
      setParentTopicId('')
      await loadTopics()
    } catch (err) {
      setSaveError(getErrorMessage(err))
    } finally {
      setSaving(false)
    }
  }

  const topicTree = buildTopicTree(topics)

  function openEditModal(topic: Topic) {
    setEditingTopic(topic)
    setEditingName(topic.topic_name)
    const nextColor = topic.display_color || DEFAULT_TOPIC_COLOR
    setEditingColor(nextColor)
    editingColorRef.current = nextColor
    setEditError(null)
  }

  function closeEditModal() {
    setEditingTopic(null)
    setEditError(null)
  }

  async function submitEdit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (!editingTopic) {
      return
    }

    const nextName = editingName.trim()
    if (!nextName) {
      setEditError('topic name is required')
      return
    }

    setEditError(null)
    setEditing(true)
    try {
      await updateTopic(editingTopic.id, {
        topicName: nextName,
        displayColor: editingColorRef.current,
      })
      closeEditModal()
      await loadTopics()
    } catch (err) {
      setEditError(getErrorMessage(err))
    } finally {
      setEditing(false)
    }
  }

  function handleCreateColorChange(nextColor: string) {
    topicColorRef.current = nextColor
    setTopicColor(nextColor)
  }

  function handleEditColorChange(nextColor: string) {
    editingColorRef.current = nextColor
    setEditingColor(nextColor)
  }

  function toggleExpanded(topicId: number) {
    setExpandedTopicIds((current) => {
      const next = new Set(current)
      if (next.has(topicId)) {
        next.delete(topicId)
      } else {
        next.add(topicId)
      }
      return next
    })
  }

  function renderTopicNodes(nodes: TopicTreeNode[], depth = 0) {
    return (
      <ul className={`topic-tree depth-${depth}`}>
        {nodes.map((node) => {
          const hasChildren = node.children.length > 0
          const isExpanded = expandedTopicIds.has(node.topic.id)
          return (
            <li key={node.topic.id} className="topic-tree-node">
              <div className="topic-tree-row">
                {hasChildren ? (
                  <button
                    type="button"
                    className="topic-tree-toggle"
                    onClick={() => toggleExpanded(node.topic.id)}
                    aria-label={isExpanded ? 'Collapse topic' : 'Expand topic'}
                    aria-expanded={isExpanded}
                  >
                    {isExpanded ? '▾' : '▸'}
                  </button>
                ) : (
                  <span className="topic-tree-toggle placeholder" aria-hidden="true">
                    •
                  </span>
                )}
                <span className="topic-list-item">
                  <button
                    type="button"
                    className="topic-node-button"
                    onClick={() => openEditModal(node.topic)}
                  >
                    <span
                      className="topic-color-dot"
                      style={{ backgroundColor: node.topic.display_color }}
                      aria-hidden="true"
                    />
                    <strong>{node.topic.topic_name}</strong>
                  </button>
                </span>
              </div>
              {hasChildren && isExpanded && renderTopicNodes(node.children, depth + 1)}
            </li>
          )
        })}
      </ul>
    )
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
          <TopicCascadeSelect
            topics={topics}
            value={parentTopicId}
            onChange={setParentTopicId}
            allowNone
            noneLabel="None"
          />
        </label>
        <div className="field-block">
          <span className="field-label">Default Color</span>
          <TopicColorPicker
            value={topicColor}
            onChange={handleCreateColorChange}
            hexInputName="create-topic-color"
          />
        </div>
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
          <>
            {topicTree.length > 0 ? (
              renderTopicNodes(topicTree)
            ) : (
              <ul className="list">
                <li>No topics yet.</li>
              </ul>
            )}
          </>
        )}
      </div>

      {editingTopic && (
        <div className="modal-backdrop" role="presentation" onClick={closeEditModal}>
          <div
            className="modal-panel"
            role="dialog"
            aria-modal="true"
            onClick={(event) => event.stopPropagation()}
          >
            <h3>Edit Topic</h3>
            <p className="modal-meta">Update name and display color</p>
            <form className="panel topic-edit-form" onSubmit={submitEdit}>
              <label>
                Topic Name
                <input
                  required
                  value={editingName}
                  onChange={(event) => setEditingName(event.target.value)}
                />
              </label>
              <div className="field-block">
                <span className="field-label">Display Color</span>
                <TopicColorPicker
                  value={editingColor}
                  onChange={handleEditColorChange}
                  hexInputName="edit-topic-color"
                />
              </div>
              {editError && <p className="error">{editError}</p>}
              <div className="modal-actions">
                <button type="button" onClick={closeEditModal}>
                  Cancel
                </button>
                <button type="submit" disabled={editing}>
                  {editing ? 'Saving...' : 'Save Changes'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </section>
  )
}

function buildTopicTree(topics: Topic[]): TopicTreeNode[] {
  const byId = new Map<number, Topic>()
  const childrenByParent = new Map<number | 'root', Topic[]>()

  for (const topic of topics) {
    byId.set(topic.id, topic)
  }

  for (const topic of topics) {
    const parentId = topic.parent_topic_id
    const parentExists = parentId !== undefined && parentId !== null && byId.has(parentId)
    const key = parentExists ? (parentId as number) : 'root'
    const bucket = childrenByParent.get(key) ?? []
    bucket.push(topic)
    childrenByParent.set(key, bucket)
  }

  const sortTopics = (items: Topic[]) =>
    [...items].sort((a, b) => a.topic_name.localeCompare(b.topic_name))

  function toNodes(parentKey: number | 'root', visited: Set<number>): TopicTreeNode[] {
    const children = sortTopics(childrenByParent.get(parentKey) ?? [])
    return children.map((topic) => {
      if (visited.has(topic.id)) {
        return { topic, children: [] }
      }
      const nextVisited = new Set(visited)
      nextVisited.add(topic.id)
      return {
        topic,
        children: toNodes(topic.id, nextVisited),
      }
    })
  }

  return toNodes('root', new Set())
}
