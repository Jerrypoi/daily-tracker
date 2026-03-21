import { TopicService } from './generated'

export type TopicInput = {
  topicName: string
  parentTopicId?: number
}

export function listTopics(parentTopicId?: number) {
  return TopicService.getTopics(parentTopicId)
}

export function createTopic(input: TopicInput) {
  return TopicService.createTopic({
    topic_name: input.topicName,
    parent_topic_id: input.parentTopicId,
  })
}
