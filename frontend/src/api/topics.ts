import { TopicService } from './generated'
import type { Topic as GeneratedTopic } from './generated'

export const DEFAULT_TOPIC_COLOR = '#3b82f6'
const fallbackApiBaseUrl = 'http://localhost:8080/api/v1'

export type Topic = GeneratedTopic & {
  display_color: string
}

export type TopicInput = {
  topicName: string
  parentTopicId?: number
  displayColor?: string
}

export type UpdateTopicInput = {
  topicName: string
  displayColor: string
}

function normalizeColor(value: unknown): string {
  if (typeof value === 'string' && /^#[0-9a-fA-F]{6}$/.test(value)) {
    return value
  }
  return DEFAULT_TOPIC_COLOR
}

function normalizeTopic(topic: GeneratedTopic): Topic {
  const topicWithColor = topic as GeneratedTopic & { display_color?: unknown }
  return {
    ...topic,
    display_color: normalizeColor(topicWithColor.display_color),
  }
}

function apiBaseUrl() {
  const value = import.meta.env.VITE_API_BASE_URL
  return value && value.trim() ? value : fallbackApiBaseUrl
}

function parseErrorBody(body: unknown): string {
  if (typeof body === 'object' && body !== null) {
    const maybe = body as { error?: string; message?: string }
    if (maybe.message) {
      return maybe.error ? `${maybe.error}: ${maybe.message}` : maybe.message
    }
  }
  return 'Request failed'
}

export function listTopics(parentTopicId?: number) {
  return TopicService.getTopics(parentTopicId).then((topics) => topics.map(normalizeTopic))
}

export async function createTopic(input: TopicInput) {
  const response = await fetch(`${apiBaseUrl()}/topics`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      topic_name: input.topicName,
      parent_topic_id: input.parentTopicId,
      display_color: input.displayColor,
    }),
  })

  if (!response.ok) {
    const body = await response.json().catch(() => ({}))
    throw new Error(parseErrorBody(body))
  }

  const created = (await response.json()) as GeneratedTopic
  return normalizeTopic(created)
}

export async function updateTopic(id: number, input: UpdateTopicInput) {
  const response = await fetch(`${apiBaseUrl()}/topics/${id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      topic_name: input.topicName,
      display_color: input.displayColor,
    }),
  })

  if (!response.ok) {
    const body = await response.json().catch(() => ({}))
    throw new Error(parseErrorBody(body))
  }

  const updated = (await response.json()) as GeneratedTopic
  return normalizeTopic(updated)
}
