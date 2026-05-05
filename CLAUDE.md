# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Daily Tracker is a full-stack app for tracking daily activities in 30-minute intervals. Users create hierarchical topics and log time slots against them.

## Common Commands

### Backend (Rust/Axum, from `backend/`)
```bash
cargo run                        # Run the server (listens on 0.0.0.0:8080)
cargo build                      # Build
cargo check                      # Type-check without building
diesel setup                     # Initialize database
diesel migration run             # Run pending migrations
diesel migration generate <name> # Create new migration
```

### Frontend (React/Vite/TypeScript, from `frontend/`)
```bash
npm install                      # Install deps
npm run dev                      # Dev server
npm run build                    # Production build (runs tsc then vite)
npm run lint                     # ESLint
npm run typecheck                # TypeScript check only
npm run generate:api             # Regenerate API client from ../swagger.json
```

### CLI (Node.js, sources in `cli/src/`)
Invoked via `npx` directly from the GitHub source — no local clone needed:
```bash
npx --yes github:Jerrypoi/daily-tracker daily-tracker help
DAILY_TRACKER_API_KEY=dt_... \
  npx --yes github:Jerrypoi/daily-tracker daily-tracker topics list
```
Zero-runtime-dep TypeScript CLI for scripting and LLM agents. The repo's root `package.json` declares the `daily-tracker` bin and a `prepare` script that compiles `cli/src/` → `cli/dist/` on install. Auth via `DAILY_TRACKER_API_KEY` (or `DAILY_TRACKER_JWT` for `/api-keys` endpoints). All output is JSON. See `cli/README.md` and skills under `skills/`.

### Makefile (from root)
```bash
make run_backend                 # cargo run from backend/
make frontend_dev                # npm run dev from frontend/
make frontend_generate_api       # Regenerate frontend API bindings
make cli_help                    # Print CLI help
```

## Architecture

### Backend
- **Entry point**: `backend/crates/server_impl/main.rs` — Axum router with CORS, request logging, and JWT auth middleware
- **Handlers**: `backend/crates/server_impl/handler.rs` — all HTTP handlers for topics, daily tracks, auth (register/login/verify-email)
- **Auth**: `backend/crates/server_impl/server_auth.rs` — JWT creation/validation, auth middleware that injects `user_id` via Axum `Extension`
- **Email**: `backend/crates/server_impl/email.rs` — verification email sending via lettre/SMTP
- **Database layer** (`backend/crates/storage/db/src/db.rs`): All Diesel queries. Uses a MySQL connection pool. Table IDs are `BIGINT`/Rust `i64`, and public API IDs use the same integer values.
- **DB models**: `backend/crates/db_model/` — Diesel schema and model structs
- **API models**: `backend/crates/models/` — request/response types, error types (`ApiError`), and conversions between DB models and API models

### Workspace crates
| Crate | Path | Purpose |
|-------|------|---------|
| `db_model` | `crates/db_model` | Diesel schema, DB model structs |
| `models` | `crates/models` | API request/response types, error handling |
| `db` | `crates/storage/db` | Database access functions |
| `logging` | `crates/logging` | Logging initialization |
| `utils` | `crates/utils` | Shared backend utilities, including Snowflake ID generation |

### Frontend
- React 19 + Vite + TypeScript
- API client auto-generated from `swagger.json` into `frontend/src/api/generated/` using `openapi-typescript-codegen`
- Pages in `frontend/src/pages/`, components in `frontend/src/components/`
- Routing via `react-router-dom`

### CLI
- TypeScript, Node 18+, zero runtime deps. Sources in `cli/src/*.ts`, compiled to `cli/dist/` by `tsc`.
- The npm package lives at the repo root: root `package.json` declares the `daily-tracker` bin (`cli/dist/bin.js`) and a `prepare` script that runs `tsc` against `tsconfig.json`. Root `tsconfig.json` has `rootDir: "cli/src"`, `outDir: "cli/dist"`. This layout is what makes `npx github:Jerrypoi/daily-tracker daily-tracker …` work — no local clone, no subdirectory specifier needed.
- Entry point: `cli/src/bin.ts` (compiled to `cli/dist/bin.js`).
- Reads `DAILY_TRACKER_API_KEY` / `DAILY_TRACKER_JWT` / `DAILY_TRACKER_API_URL` from env.
- All commands print one JSON document on success; one JSON error document on stderr on failure.
- LLM-agent skills live under `skills/<skill-name>/SKILL.md` (repo root) and can be symlinked into `~/.claude/skills/`.

### API Routes
All routes under `/api/v1/`:
- Auth (unauthenticated): `POST /auth/register`, `POST /auth/login`, `POST /auth/verify-email`
- Topics (JWT required): `GET|POST /topics`, `GET|PUT /topics/:id`
- Daily tracks (JWT required): `GET|POST /daily-tracks`, `GET|PUT|DELETE /daily-tracks/:id`

## Key Patterns

- **ID encoding**: Table IDs are Snowflake-style positive `BIGINT` values generated in the backend and represented as Rust `i64`; foreign key columns use the same type.
- **User data isolation**: Topics and daily tracks are scoped by `user_id` extracted from JWT in auth middleware.
- **API contract**: `swagger.json` is the source of truth for the API schema. After backend API changes, update `swagger.json` and run `make frontend_generate_api`.

## Environment

- Backend requires `.env` file in `backend/` with `DATABASE_URL` (MySQL) and `JWT_SECRET`
- macOS/Homebrew: build needs zstd and openssl link flags (configured in `backend/.cargo/config.toml`)
- Diesel CLI required for migrations (`cargo install diesel_cli --no-default-features --features mysql`)
