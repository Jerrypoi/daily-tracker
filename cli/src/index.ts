// Entry point for the daily-tracker CLI.
//
// Output convention: every successful command writes a single JSON document to
// stdout (or nothing for 204 responses). Every error writes a single JSON
// document to stderr and exits non-zero. This makes the tool predictable for
// LLM agents and scripts.

import { parseArgs, type FlagValue } from "./args.js";
import { ApiError, request } from "./client.js";
import { HELP } from "./help.js";

type Flags = Record<string, FlagValue>;

const HELP_FLAGS = new Set(["help", "h"]);

export async function run(argv: string[]): Promise<void> {
  const parsed = parseArgs(argv);

  if (parsed.positional.length === 0 || parsed.positional[0] === "help") {
    process.stdout.write(HELP + "\n");
    return;
  }

  if (
    parsed.positional.length === 0 &&
    Object.keys(parsed.flags).some((f) => HELP_FLAGS.has(f))
  ) {
    process.stdout.write(HELP + "\n");
    return;
  }

  const [resource, action, ...rest] = parsed.positional;

  try {
    switch (resource) {
      case "topics":
        await topics(action, rest, parsed.flags);
        break;
      case "tracks":
      case "daily-tracks":
        await tracks(action, rest, parsed.flags);
        break;
      case "api-keys":
        await apiKeys(action, rest, parsed.flags);
        break;
      case "auth":
        await auth(action, rest, parsed.flags);
        break;
      case "whoami":
        await whoami();
        break;
      default:
        fail("UNKNOWN_COMMAND", `Unknown resource: ${resource}`, 2);
    }
  } catch (err) {
    if (err instanceof ApiError) {
      process.stderr.write(
        JSON.stringify({
          error: err.code,
          status: err.status,
          message: err.message,
          body: err.body,
        }) + "\n",
      );
      process.exit(1);
    }
    throw err;
  }
}

function emit(data: unknown): void {
  if (data === undefined || data === null) return;
  process.stdout.write(JSON.stringify(data, null, 2) + "\n");
}

function fail(code: string, message: string, exitCode = 1): never {
  process.stderr.write(JSON.stringify({ error: code, message }) + "\n");
  process.exit(exitCode);
}

function requireFlag(flags: Flags, name: string): string {
  const v = flags[name];
  if (v === undefined || v === true) {
    fail("MISSING_FLAG", `Missing required flag --${name}`, 2);
  }
  return v;
}

interface NumOpts {
  required?: boolean;
}

function intFlag(
  flags: Flags,
  name: string,
  { required = false }: NumOpts = {},
): number | undefined {
  const v = flags[name];
  if (v === undefined || v === true) {
    if (required) fail("MISSING_FLAG", `Missing required flag --${name}`, 2);
    return undefined;
  }
  const n = Number.parseInt(v, 10);
  if (Number.isNaN(n)) {
    fail("INVALID_FLAG", `--${name} must be an integer`, 2);
  }
  return n;
}

function strFlag(
  flags: Flags,
  name: string,
  { required = false }: NumOpts = {},
): string | undefined {
  const v = flags[name];
  if (v === undefined || v === true) {
    if (required) fail("MISSING_FLAG", `Missing required flag --${name}`, 2);
    return undefined;
  }
  return v;
}

function positionalInt(rest: string[], idx: number, label: string): number {
  const raw = rest[idx];
  if (raw === undefined) {
    fail("MISSING_ARG", `Missing positional argument: ${label}`, 2);
  }
  const n = Number.parseInt(raw, 10);
  if (Number.isNaN(n)) {
    fail("INVALID_ARG", `${label} must be an integer`, 2);
  }
  return n;
}

// ---------- topics ----------

async function topics(
  action: string | undefined,
  rest: string[],
  flags: Flags,
): Promise<void> {
  switch (action) {
    case "list": {
      const parent = intFlag(flags, "parent");
      const qs = parent !== undefined ? `?parent_topic_id=${parent}` : "";
      emit(await request("GET", `/topics${qs}`));
      return;
    }
    case "get": {
      const id = positionalInt(rest, 0, "topic id");
      emit(await request("GET", `/topics/${id}`));
      return;
    }
    case "create": {
      const body: Record<string, unknown> = {
        topic_name: requireFlag(flags, "name"),
      };
      const parent = intFlag(flags, "parent");
      if (parent !== undefined) body.parent_topic_id = parent;
      const color = strFlag(flags, "color");
      if (color) body.display_color = color;
      emit(await request("POST", "/topics", body));
      return;
    }
    case "update": {
      const id = positionalInt(rest, 0, "topic id");
      const body = {
        topic_name: requireFlag(flags, "name"),
        display_color: requireFlag(flags, "color"),
      };
      emit(await request("PUT", `/topics/${id}`, body));
      return;
    }
    default:
      fail(
        "UNKNOWN_COMMAND",
        `Unknown topics action: ${action ?? "(none)"}. Try: list, get, create, update.`,
        2,
      );
  }
}

// ---------- daily tracks ----------

async function tracks(
  action: string | undefined,
  rest: string[],
  flags: Flags,
): Promise<void> {
  switch (action) {
    case "list": {
      const params = new URLSearchParams();
      const start = strFlag(flags, "start");
      const end = strFlag(flags, "end");
      const topic = intFlag(flags, "topic");
      if (start) params.set("start_date", start);
      if (end) params.set("end_date", end);
      if (topic !== undefined) params.set("topic_id", String(topic));
      const qs = params.toString();
      emit(await request("GET", `/daily-tracks${qs ? `?${qs}` : ""}`));
      return;
    }
    case "get": {
      const id = positionalInt(rest, 0, "track id");
      emit(await request("GET", `/daily-tracks/${id}`));
      return;
    }
    case "create": {
      const body: Record<string, unknown> = {
        start_time: requireFlag(flags, "start-time"),
        topic_id: intFlag(flags, "topic", { required: true })!,
      };
      const comment = strFlag(flags, "comment");
      if (comment !== undefined) body.comment = comment;
      emit(await request("POST", "/daily-tracks", body));
      return;
    }
    case "update": {
      const id = positionalInt(rest, 0, "track id");
      const body: Record<string, unknown> = {
        topic_id: intFlag(flags, "topic", { required: true })!,
      };
      const comment = strFlag(flags, "comment");
      if (comment !== undefined) body.comment = comment;
      emit(await request("PUT", `/daily-tracks/${id}`, body));
      return;
    }
    case "delete": {
      const id = positionalInt(rest, 0, "track id");
      await request("DELETE", `/daily-tracks/${id}`);
      emit({ deleted: id });
      return;
    }
    default:
      fail(
        "UNKNOWN_COMMAND",
        `Unknown tracks action: ${action ?? "(none)"}. Try: list, get, create, update, delete.`,
        2,
      );
  }
}

// ---------- api keys (JWT-only on server) ----------

async function apiKeys(
  action: string | undefined,
  rest: string[],
  flags: Flags,
): Promise<void> {
  switch (action) {
    case "list":
      emit(await request("GET", "/api-keys"));
      return;
    case "create": {
      const body = { name: requireFlag(flags, "name") };
      emit(await request("POST", "/api-keys", body));
      return;
    }
    case "revoke": {
      const id = positionalInt(rest, 0, "api key id");
      await request("DELETE", `/api-keys/${id}`);
      emit({ revoked: id });
      return;
    }
    default:
      fail(
        "UNKNOWN_COMMAND",
        `Unknown api-keys action: ${action ?? "(none)"}. Try: list, create, revoke. Note: api-keys endpoints require a JWT, not an API key.`,
        2,
      );
  }
}

// ---------- auth (no token required) ----------

async function auth(
  action: string | undefined,
  _rest: string[],
  flags: Flags,
): Promise<void> {
  switch (action) {
    case "login": {
      const body = {
        username: requireFlag(flags, "username"),
        password: requireFlag(flags, "password"),
      };
      emit(await request("POST", "/auth/login", body, { auth: false }));
      return;
    }
    case "register": {
      const body = {
        username: requireFlag(flags, "username"),
        email: requireFlag(flags, "email"),
        password: requireFlag(flags, "password"),
      };
      emit(await request("POST", "/auth/register", body, { auth: false }));
      return;
    }
    case "verify-email": {
      const body = {
        email: requireFlag(flags, "email"),
        code: requireFlag(flags, "code"),
      };
      emit(await request("POST", "/auth/verify-email", body, { auth: false }));
      return;
    }
    default:
      fail(
        "UNKNOWN_COMMAND",
        `Unknown auth action: ${action ?? "(none)"}. Try: login, register, verify-email.`,
        2,
      );
  }
}

// ---------- whoami ----------

async function whoami(): Promise<void> {
  const url = process.env.DAILY_TRACKER_API_URL || "http://localhost:8080/api/v1";
  const apiKey = process.env.DAILY_TRACKER_API_KEY;
  const jwt = process.env.DAILY_TRACKER_JWT;
  emit({
    api_url: url,
    auth: apiKey
      ? { type: "api_key", key_prefix: apiKey.slice(0, 8) }
      : jwt
        ? { type: "jwt" }
        : { type: "none" },
  });
}
