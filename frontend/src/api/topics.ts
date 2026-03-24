import { TopicService } from './generated'
import type { Topic as GeneratedTopic } from './generated'

export const DEFAULT_TOPIC_COLOR = '#3b82f6'

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

// Deleted unused url helpers

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
  try {
    const created = await TopicService.createTopic({
      topic_name: input.topicName,
      parent_topic_id: input.parentTopicId,
      display_color: input.displayColor
    })
    return normalizeTopic(created)
  } catch (err: any) {
    throw new Error(parseErrorBody(err?.body || err))
  }
}

export async function updateTopic(id: number, input: UpdateTopicInput) {
  try {
    const updated = await TopicService.updateTopic(id, {
      topic_name: input.topicName,
      display_color: input.displayColor
    })
    return normalizeTopic(updated)
  } catch (err: any) {
    throw new Error(parseErrorBody(err?.body || err))
  }
}
