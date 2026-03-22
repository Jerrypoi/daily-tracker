import { DailyTrackService } from './generated'
import type { DailyTrack } from './generated'

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

const fallbackApiBaseUrl = 'http://localhost:8080/api/v1'

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
  const response = await fetch(`${apiBaseUrl()}/daily-tracks/${id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      topic_id: input.topicId,
      comment: input.comment,
    }),
  })

  if (!response.ok) {
    const body = await response.json().catch(() => ({}))
    throw new Error(parseErrorBody(body))
  }

  return (await response.json()) as DailyTrack
}

export async function deleteDailyTrack(id: number) {
  const response = await fetch(`${apiBaseUrl()}/daily-tracks/${id}`, {
    method: 'DELETE',
  })

  if (!response.ok) {
    const body = await response.json().catch(() => ({}))
    throw new Error(parseErrorBody(body))
  }
}

export function isHalfHourBoundary(value: string): boolean {
  // Works for datetime-local values like YYYY-MM-DDTHH:mm
  const minute = Number(value.slice(-2))
  return minute === 0 || minute === 30
}
