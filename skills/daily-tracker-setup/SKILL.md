---
name: daily-tracker-setup
description: Use to perform first-time setup for the daily-tracker CLI — register an account, verify email, log in, and mint an API key — so subsequent agent runs can call the CLI with DAILY_TRACKER_API_KEY. Trigger when the user says they have not yet set up the CLI, or asks how to get an API key for an LLM agent.
---

# Daily Tracker CLI — first-time setup

Use this skill once per machine/agent. Goal: end with a `DAILY_TRACKER_API_KEY` value the user can stash in their shell profile or agent config.

The CLI is invoked directly from GitHub via `npx` — no clone or local checkout is needed. The canonical form is:

```
npx --yes github:Jerrypoi/daily-tracker daily-tracker <command>
```

To save typing, set an alias for the current shell session:

```
alias dt='npx --yes github:Jerrypoi/daily-tracker daily-tracker'
```

The steps below assume that alias is set; substitute the full `npx …` form if not.

## 1. Confirm the backend URL

The default is `http://localhost:8080/api/v1`. If the user runs a hosted instance, ask them for the base URL and export it:

```
export DAILY_TRACKER_API_URL="https://<host>/api/v1"
```

## 2. Register or log in

If the user already has an account, skip to step 3.

```
dt auth register --username "<u>" --email "<e>" --password "<p>"
```

Then verify the email with the code from the verification email:

```
dt auth verify-email --email "<e>" --code "<code>"
```

## 3. Log in to obtain a JWT

```
dt auth login --username "<u>" --password "<p>"
```

Capture `token` from the JSON output:

```
JWT=$(dt auth login --username "<u>" --password "<p>" | jq -r .token)
```

## 4. Mint an API key

API-key management is JWT-only. Pass the JWT explicitly and ensure no stale `DAILY_TRACKER_API_KEY` is set:

```
DAILY_TRACKER_API_KEY= DAILY_TRACKER_JWT="$JWT" \
  dt api-keys create --name "agent-name"
```

The response includes a `token` field starting with `dt_`. **It is shown exactly once — capture it now.** Recommended:

```
export DAILY_TRACKER_API_KEY="dt_..."   # add to ~/.zshrc / agent secret store
```

## 5. Verify

```
dt whoami
dt topics list
```

`whoami` should show `auth.type: "api_key"`. `topics list` should return an array (possibly empty) with no error.

## Notes

- Treat the `dt_...` token like a password — store it in the user's secret manager, not in the repo.
- If the user loses the token, mint a new one and revoke the old via `dt api-keys revoke <id>`.
- The JWT expires after 7 days; the API key does not expire unless revoked.
- For reproducible setups, pin to a release: replace `github:Jerrypoi/daily-tracker` with `github:Jerrypoi/daily-tracker#v0.1.0` (or any tag / commit SHA).
