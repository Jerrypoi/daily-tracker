import { ApiKeyService } from './generated'
import type { ApiKey, CreateApiKeyResponse } from './generated'

export type { ApiKey, CreateApiKeyResponse }

export function listApiKeys() {
  return ApiKeyService.listApiKeys()
}

export function createApiKey(name: string) {
  return ApiKeyService.createApiKey({ name })
}

export function revokeApiKey(id: number) {
  return ApiKeyService.revokeApiKey(id)
}
