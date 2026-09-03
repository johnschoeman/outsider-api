# CLAUDE.md

## Overview

API for the Outsider game — Rust/Axum HTTP service deployed via Shuttle.

**Status:** early stage — handlers return mock data, DB layer (sqlx repo) not yet wired up.

## Key Commands

- `nix develop` - enter build environment
- `cargo check` - static analysis / type check
- `shuttle dev` - run the API locally

## Verification

After code changes, run `cargo check`. No test suite exists yet.

## Key Directories

- `src/handlers/` - Axum route handlers (`lobby.rs`, `error.rs`)
- `src/models/` - request/response and domain types (`lobby.rs`)
- `src/main.rs` - router setup, `AppState`, Shuttle entrypoint

## Behavioral Rules

- **Think before coding** — state assumptions explicitly. If something is unclear, stop and ask.
- **Simplicity first** — implement only what's requested. No speculative abstractions.
- **Surgical changes** — touch only what's needed, match existing style, don't "improve" adjacent code unprompted.
- **Goal-driven execution** — frame tasks as verifiable success criteria; run `cargo check` to confirm.

## Conventions

- ALWAYS return `Result<_, AppError>` from handlers — use the `AppError` enum (`src/handlers/error.rs`) instead of `unwrap`/`panic`
- Handlers currently return mock data — DB wiring (sqlx `OutsiderRepository`, `AppState.repo`) is commented out in `src/main.rs`; don't assume it's live

## Gotchas

- `AppState` currently only has a `count: u32` field — the intended `repo: OutsiderRepository` field is commented out pending sqlx integration
- `.gitignore` covers `Secrets.toml`/`.env` — never commit Shuttle secrets files
