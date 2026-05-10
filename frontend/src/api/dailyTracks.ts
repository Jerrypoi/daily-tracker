import { DailyTrackService } from './generated'
// Removed unused import

export type DailyTrackFilter = {
  startDate?: string
  endDate?: string
  topicId?: string
}

export type DailyTrackInput = {
  startTime: string
  topicId: string
  comment?: string
  durationMinutes: number
}

export type UpdateDailyTrackInput = {
  topicId: string
  comment?: string
  durationMinutes: number
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

function extractErrorBody(err: unknown): unknown {
  if (typeof err === 'object' && err !== null && 'body' in err) {
    return (err as { body: unknown }).body
  }
  return err
}

export function listDailyTracks(filter: DailyTrackFilter) {
  return DailyTrackService.getDailyTracks(
    filter.startDate,
    filter.endDate,
    filter.topicId,
  )
}

export async function createDailyTrack(input: DailyTrackInput) {
  try {
    return await DailyTrackService.createDailyTrack({
      start_time: input.startTime,
      topic_id: input.topicId,
      comment: input.comment,
      duration_minutes: input.durationMinutes,
    })
  } catch (err: unknown) {
    throw new Error(parseErrorBody(extractErrorBody(err)))
  }
}

export async function updateDailyTrack(id: string, input: UpdateDailyTrackInput) {
  try {
    return await DailyTrackService.updateDailyTrack(id, {
      topic_id: input.topicId,
      comment: input.comment,
      duration_minutes: input.durationMinutes,
    })
  } catch (err: unknown) {
    throw new Error(parseErrorBody(extractErrorBody(err)))
  }
}

export async function deleteDailyTrack(id: string) {
  try {
    await DailyTrackService.deleteDailyTrack(id)
  } catch (err: unknown) {
    throw new Error(parseErrorBody(extractErrorBody(err)))
  }
}

export function isHalfHourBoundary(value: string): boolean {
  // Works for datetime-local values like YYYY-MM-DDTHH:mm
  const minute = Number(value.slice(-2))
  return minute === 0 || minute === 30
}
