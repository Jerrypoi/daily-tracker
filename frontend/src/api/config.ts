import { OpenAPI } from './generated'

const FALLBACK_API_BASE_URL = 'http://localhost:8080/api/v1'

export function initializeApiConfig() {
  const envBaseUrl = import.meta.env.VITE_API_BASE_URL
  OpenAPI.BASE = envBaseUrl && envBaseUrl.trim() ? envBaseUrl : FALLBACK_API_BASE_URL
}
