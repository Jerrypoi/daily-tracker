// Thin HTTP client wrapping fetch.
//
// Auth resolution order:
//   1. DAILY_TRACKER_API_KEY    — API key (recommended for agents/automation)
//   2. DAILY_TRACKER_JWT        — escape hatch for endpoints that require JWT
//                                 (e.g. /api-keys management)
// Both are sent as `Authorization: Bearer <token>`. The server distinguishes
// API keys by their `dt_` prefix.

const DEFAULT_BASE_URL = "http://localhost:8080/api/v1";

export interface ApiErrorInit {
  status: number;
  code: string;
  message: string;
  body?: unknown;
}

export class ApiError extends Error {
  status: number;
  code: string;
  body?: unknown;

  constructor({ status, code, message, body }: ApiErrorInit) {
    super(message);
    this.status = status;
    this.code = code;
    this.body = body;
  }
}

function baseUrl(): string {
  return (process.env.DAILY_TRACKER_API_URL || DEFAULT_BASE_URL).replace(
    /\/+$/,
    "",
  );
}

function authToken(): string | undefined {
  return process.env.DAILY_TRACKER_API_KEY || process.env.DAILY_TRACKER_JWT;
}

export interface RequestOptions {
  auth?: boolean;
}

export async function request(
  method: string,
  path: string,
  body?: unknown,
  { auth = true }: RequestOptions = {},
): Promise<unknown> {
  const headers: Record<string, string> = {};
  if (body !== undefined) headers["Content-Type"] = "application/json";

  if (auth) {
    const token = authToken();
    if (!token) {
      throw new ApiError({
        status: 0,
        code: "MISSING_CREDENTIALS",
        message:
          "No credentials. Set DAILY_TRACKER_API_KEY (preferred) or DAILY_TRACKER_JWT.",
      });
    }
    headers["Authorization"] = `Bearer ${token}`;
  }

  let res: Response;
  try {
    res = await fetch(`${baseUrl()}${path}`, {
      method,
      headers,
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });
  } catch (e) {
    throw new ApiError({
      status: 0,
      code: "NETWORK_ERROR",
      message: e instanceof Error ? e.message : String(e),
    });
  }

  if (res.status === 204) return undefined;

  const text = await res.text();
  let parsed: unknown = undefined;
  if (text.length > 0) {
    try {
      parsed = JSON.parse(text);
    } catch {
      parsed = text;
    }
  }

  if (!res.ok) {
    const obj =
      parsed && typeof parsed === "object" ? (parsed as Record<string, unknown>) : undefined;
    const code =
      (obj && typeof obj.error === "string" && obj.error) || `HTTP_${res.status}`;
    const message =
      (obj && typeof obj.message === "string" && obj.message) ||
      (typeof parsed === "string" ? parsed : `Request failed: ${res.status}`);
    throw new ApiError({ status: res.status, code, message, body: parsed });
  }

  return parsed;
}
