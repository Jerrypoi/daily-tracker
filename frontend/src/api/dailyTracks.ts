import { DailyTrackService } from './generated'

export type DailyTrackFilter = {
  startDate?: string
  endDate?: string
  topicId?: number
}

export type DailyTrackInput = {
  startTime: string
  topicId?: number
  comment?: string
}

export function listDailyTracks(filter: DailyTrackFilter) {
  return DailyTrackService.getDailyTracks(
    filter.startDate,
    filter.endDate,
    filter.topicId,
  )
}

export function createDailyTrack(input: DailyTrackInput) {
  return DailyTrackService.createDailyTrack({
    start_time: input.startTime,
    topic_id: input.topicId,
    comment: input.comment,
  })
}

export function isHalfHourBoundary(value: string): boolean {
  // Works for datetime-local values like YYYY-MM-DDTHH:mm
  const minute = Number(value.slice(-2))
  return minute === 0 || minute === 30
}
