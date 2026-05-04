import { DailyTrackService } from './generated'
// Removed unused import

export type DailyTrackFilter = {
  startDate?: string
  endDate?: string
  topicId?: number
}

export type DailyTrackInput = {
  startTime: string
  topicId: number
  comment?: string
}

export type UpdateDailyTrackInput = {
  topicId: number
  comment?: string
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

export async function updateDailyTrack(id: number, input: UpdateDailyTrackInput) {
  try {
    return await DailyTrackService.updateDailyTrack(id, {
      topic_id: input.topicId,
      comment: input.comment
    })
  } catch (err: any) {
    throw new Error(parseErrorBody(err?.body || err))
  }
}

export async function deleteDailyTrack(id: number) {
  try {
    await DailyTrackService.deleteDailyTrack(id)
  } catch (err: any) {
    throw new Error(parseErrorBody(err?.body || err))
  }
}

export function isHalfHourBoundary(value: string): boolean {
  // Works for datetime-local values like YYYY-MM-DDTHH:mm
  const minute = Number(value.slice(-2))
  return minute === 0 || minute === 30
}
