import { ApiError } from './generated'

export function getErrorMessage(error: unknown): string {
  if (error instanceof ApiError) {
    const body = error.body as { error?: string; message?: string } | undefined
    if (body?.message) {
      return body.error ? `${body.error}: ${body.message}` : body.message
    }
    return `${error.status}: ${error.message}`
  }

  if (error instanceof Error) {
    return error.message
  }

  return 'Unexpected error'
}
