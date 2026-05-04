export const HELP = `daily-tracker — CLI for the Daily Tracker backend

USAGE
  daily-tracker <resource> <action> [args] [flags]

ENV
  DAILY_TRACKER_API_KEY   API key (recommended). Sent as Bearer token.
  DAILY_TRACKER_JWT       JWT (only needed for /api-keys management).
  DAILY_TRACKER_API_URL   Base URL. Default: http://localhost:8080/api/v1

OUTPUT
  All commands print a single JSON document to stdout on success, or a single
  JSON error document to stderr (and exit non-zero) on failure.

TOPICS
  topics list [--parent <id>]
  topics get <id>
  topics create --name <name> [--parent <id>] [--color <#hex>]
  topics update <id> --name <name> --color <#hex>

DAILY TRACKS  (alias: \`tracks\`)
  tracks list [--start <YYYY-MM-DD>] [--end <YYYY-MM-DD>] [--topic <id>]
  tracks get <id>
  tracks create --start-time <ISO-8601> --topic <id> [--comment <text>]
  tracks update <id> --topic <id> [--comment <text>]
  tracks delete <id>

API KEYS  (JWT required — set DAILY_TRACKER_JWT)
  api-keys list
  api-keys create --name <label>
  api-keys revoke <id>

AUTH  (no token required)
  auth register --username <u> --email <e> --password <p>
  auth verify-email --email <e> --code <c>
  auth login --username <u> --password <p>

MISC
  whoami     Show which credentials and base URL are configured.
  help       Show this help.

EXAMPLES
  DAILY_TRACKER_API_KEY=dt_... daily-tracker topics list
  daily-tracker topics create --name "deep work" --color "#3b82f6"
  daily-tracker tracks create --start-time 2026-04-27T09:00:00Z --topic 12 \\
    --comment "review PRs"
  daily-tracker tracks list --start 2026-04-20 --end 2026-04-27`;
