---
name: daily-tracker
description: Use to read or modify a user's Daily Tracker data (topics and 30-minute time-slot tracks) from the command line via npx. Trigger when the user asks to log time, add a topic, list tracks, summarize a day/week, or otherwise interact with their Daily Tracker account. Requires DAILY_TRACKER_API_KEY in the environment.
---

# Daily Tracker CLI

The `daily-tracker` CLI is a thin wrapper around the Daily Tracker REST API. It is intended for scripting and LLM agent use: every successful command prints a single JSON document to stdout, and every error prints a single JSON document to stderr (with non-zero exit).

## Setup

Set credentials and (optionally) the API base URL before calling any command:

```
export DAILY_TRACKER_API_KEY="dt_..."        # required for all data ops
export DAILY_TRACKER_API_URL="https://your-host/api/v1"   # optional; default http://localhost:8080/api/v1
```

API keys are created from the web UI or via `daily-tracker api-keys create` (the latter requires a JWT — see "API key management").

Confirm setup before issuing requests:

```
npx --yes github:Jerrypoi/daily-tracker daily-tracker whoami
```

## Invocation

The CLI is published as an npm package living at the **root** of `github:Jerrypoi/daily-tracker`, so `npx` can install and run it directly from the GitHub source — no clone, no checked-out repo required:

```
# canonical form for ad-hoc / agent use (latest default branch):
npx --yes github:Jerrypoi/daily-tracker daily-tracker <command>

# pinned to a tag or commit (recommended for reproducibility):
npx --yes github:Jerrypoi/daily-tracker#v0.1.0 daily-tracker <command>
npx --yes github:Jerrypoi/daily-tracker#<sha>  daily-tracker <command>

# if the user has installed once with `npm i -g github:Jerrypoi/daily-tracker`:
daily-tracker <command>
```

When this skill runs commands, prefer the `npx --yes github:Jerrypoi/daily-tracker daily-tracker …` form unless the user has confirmed the binary is on PATH. Inside the same shell session you can also alias it once:

```
alias dt='npx --yes github:Jerrypoi/daily-tracker daily-tracker'
```

The first invocation downloads and compiles the CLI (~few seconds). Subsequent calls in the same npx cache window are fast.

## Commands

### Topics

A topic is a category for time entries. Topics can be hierarchical via `parent_topic_id`.

```
daily-tracker topics list [--parent <id>]
daily-tracker topics get <id>
daily-tracker topics create --name <name> [--parent <id>] [--color "#RRGGBB"]
daily-tracker topics update <id> --name <name> --color "#RRGGBB"
```

### Daily tracks (30-minute time slots)

A track records that the user spent a 30-minute slot on a topic. `start_time` must align to `:00` or `:30` and is ISO-8601 (use `Z` for UTC).

```
daily-tracker tracks list [--start YYYY-MM-DD] [--end YYYY-MM-DD] [--topic <id>]
daily-tracker tracks get <id>
daily-tracker tracks create --start-time <ISO> --topic <id> [--comment <text>]
daily-tracker tracks update <id> --topic <id> [--comment <text>]
daily-tracker tracks delete <id>
```

`tracks` and `daily-tracks` are accepted as synonyms.

### API key management (JWT only)

The backend forbids API keys from minting or revoking other API keys, so these commands require a JWT instead. Set `DAILY_TRACKER_JWT` (obtained via `auth login`) and unset or ignore `DAILY_TRACKER_API_KEY` for these calls — the CLI prefers `API_KEY` if both are set, so for these commands run with the JWT explicitly:

```
DAILY_TRACKER_API_KEY= DAILY_TRACKER_JWT="<jwt>" daily-tracker api-keys list
DAILY_TRACKER_API_KEY= DAILY_TRACKER_JWT="<jwt>" daily-tracker api-keys create --name "ci-bot"
DAILY_TRACKER_API_KEY= DAILY_TRACKER_JWT="<jwt>" daily-tracker api-keys revoke <id>
```

Only `api-keys create` returns the plaintext token; capture it from the JSON output immediately.

### Auth (no token required)

```
daily-tracker auth register --username <u> --email <e> --password <p>
daily-tracker auth verify-email --email <e> --code <c>
daily-tracker auth login --username <u> --password <p>
```

`auth login` returns `{ "token": "<jwt>" }`; export that as `DAILY_TRACKER_JWT` to manage API keys.

## Output and error contract

- Success: a single JSON document on stdout, exit 0. For `delete`/`revoke` (HTTP 204) the CLI emits `{"deleted": <id>}` / `{"revoked": <id>}` so downstream tools always get JSON.
- Failure: a single JSON document on stderr, non-zero exit. Shape:
  ```json
  {"error": "<CODE>", "status": <http_status_or_0>, "message": "<human msg>", "body": <server_body_or_null>}
  ```
- Common error codes:
  - `MISSING_CREDENTIALS` — no `DAILY_TRACKER_API_KEY` / `DAILY_TRACKER_JWT` set.
  - `MISSING_FLAG` / `MISSING_ARG` / `INVALID_ARG` / `INVALID_FLAG` — usage error (exit 2).
  - `UNKNOWN_COMMAND` — bad resource/action (exit 2).
  - `NETWORK_ERROR` — could not reach the server.
  - Any server `ApiError.error` value (e.g. `VALIDATION_ERROR`, `NOT_FOUND`, `CONFLICT`).

Always parse stdout as JSON. Do not screen-scrape error messages — branch on the `error` code field.

## Common workflows

### Log the current 30-minute slot to a topic

1. Resolve the topic id from a name:
   ```
   daily-tracker topics list | jq -r '.[] | select(.topic_name=="deep work") | .id'
   ```
2. Compute the slot start (round current time down to :00 or :30, UTC) — e.g. `2026-04-27T14:30:00Z`.
3. Create the track:
   ```
   daily-tracker tracks create --start-time 2026-04-27T14:30:00Z --topic <id> --comment "<what you did>"
   ```

If the slot is already taken the server returns 409 `CONFLICT`; in that case prefer `tracks update <existing-id>`.

### Summarize a day/week

```
daily-tracker tracks list --start 2026-04-21 --end 2026-04-27 > /tmp/week.json
```

The result is an array of track records — group by `topic_id`, multiply count by 30 minutes for hours.

### Bulk-create tracks

Loop in shell, one `tracks create` per slot. The server enforces the `:00`/`:30` and uniqueness rules, so let it reject malformed entries rather than pre-validating.

## Things to avoid

- Do not hard-code the API URL — read it from `DAILY_TRACKER_API_URL`.
- Do not invoke this CLI without confirming a destructive action (`tracks delete`, `topics update`, `api-keys revoke`) with the user first.
- Do not log the API key or JWT to files or stdout. `whoami` prints only the first 8 characters of the API key by design.
- Do not retry network errors silently — surface the JSON error so the user sees it.
