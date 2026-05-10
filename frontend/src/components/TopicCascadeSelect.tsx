import { useMemo, useState } from 'react'

import type { Topic } from '../api/topics'

type TopicCascadeSelectProps = {
  topics: Topic[]
  value: string
  onChange: (value: string) => void
  allowNone?: boolean
  noneLabel?: string
}

type LevelNode = {
  parentId: string | 'root'
  options: Topic[]
}

function sortByName(topics: Topic[]): Topic[] {
  return [...topics].sort((a, b) => a.topic_name.localeCompare(b.topic_name))
}

export function TopicCascadeSelect({
  topics,
  value,
  onChange,
  allowNone = false,
  noneLabel = 'None',
}: TopicCascadeSelectProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [hoverPath, setHoverPath] = useState<string[]>([])

  const byId = useMemo(() => {
    const map = new Map<string, Topic>()
    for (const topic of topics) {
      map.set(topic.id, topic)
    }
    return map
  }, [topics])

  const childrenByParent = useMemo(() => {
    const map = new Map<string | 'root', Topic[]>()
    for (const topic of topics) {
      const parentId = topic.parent_topic_id
      const key =
        parentId !== undefined && parentId !== null && byId.has(parentId)
          ? parentId
          : 'root'
      const bucket = map.get(key) ?? []
      bucket.push(topic)
      map.set(key, bucket)
    }
    for (const [key, list] of map.entries()) {
      map.set(key, sortByName(list))
    }
    return map
  }, [topics, byId])

  const selectedId = value || undefined
  const selectedTopic = selectedId !== undefined ? byId.get(selectedId) : undefined

  const selectedPath = useMemo(() => {
    if (!selectedTopic) {
      return [] as Topic[]
    }

    const path: Topic[] = []
    const visited = new Set<string>()
    let cursor: Topic | undefined = selectedTopic
    while (cursor && !visited.has(cursor.id)) {
      visited.add(cursor.id)
      path.unshift(cursor)
      const parentId: string | undefined =
        cursor.parent_topic_id ?? undefined
      cursor =
        parentId !== undefined && parentId !== null ? byId.get(parentId) : undefined
    }
    return path
  }, [selectedTopic, byId])

  const label = selectedPath.length
    ? selectedPath.map((item) => item.topic_name).join(' / ')
    : allowNone
      ? noneLabel
      : 'Select topic'

  const levels = useMemo(() => {
    const rendered: LevelNode[] = []
    let parent: string | 'root' = 'root'
    let depth = 0

    while (true) {
      const options = childrenByParent.get(parent) ?? []
      if (options.length === 0) {
        break
      }
      rendered.push({ parentId: parent, options })

      const nextId = hoverPath[depth]
      if (nextId === undefined) {
        break
      }
      const childOptions = childrenByParent.get(nextId) ?? []
      if (childOptions.length === 0) {
        break
      }
      parent = nextId
      depth += 1
    }

    return rendered
  }, [childrenByParent, hoverPath])

  function openMenu() {
    setHoverPath(selectedPath.map((t) => t.id))
    setIsOpen(true)
  }

  function closeMenu() {
    setIsOpen(false)
  }

  function selectTopic(id: string) {
    onChange(id)
    closeMenu()
  }

  function clearSelection() {
    onChange('')
    closeMenu()
  }

  return (
    <div className="cascade-root">
      <button
        type="button"
        className="cascade-trigger"
        onClick={() => (isOpen ? closeMenu() : openMenu())}
      >
        <span className="cascade-trigger-label">{label}</span>
        <span className="cascade-arrow">▾</span>
      </button>
      {isOpen && (
        <div className="cascade-popover" onMouseLeave={closeMenu}>
          {allowNone && (
            <button type="button" className="cascade-option" onClick={clearSelection}>
              {noneLabel}
            </button>
          )}
          <div className="cascade-columns">
            {levels.map((level, levelIndex) => (
              <div key={`${level.parentId}-${levelIndex}`} className="cascade-column">
                {level.options.map((topic) => {
                  const hasChildren = (childrenByParent.get(topic.id) ?? []).length > 0
                  const isSelected = selectedId === topic.id
                  return (
                    <button
                      key={topic.id}
                      type="button"
                      className={`cascade-option ${isSelected ? 'is-selected' : ''}`}
                      onMouseEnter={() => {
                        const next = [...hoverPath.slice(0, levelIndex), topic.id]
                        setHoverPath(next)
                      }}
                      onClick={() => selectTopic(topic.id)}
                    >
                      <span className="cascade-option-content">
                        <span
                          className="topic-color-dot"
                          style={{ backgroundColor: topic.display_color }}
                          aria-hidden="true"
                        />
                        <span>{topic.topic_name}</span>
                      </span>
                      {hasChildren && <span className="cascade-sub-arrow">›</span>}
                    </button>
                  )
                })}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
