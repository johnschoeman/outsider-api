# Coding Conventions

Design conventions that require judgment — things linters can't enforce.

## Universal

- ALWAYS prefer simplicity — implement only what's requested, no speculative features
- ALWAYS match existing patterns in the codebase before inventing new ones
- Use early returns to reduce nesting
- Small functions (<40 lines) and small files (<500 lines)
- Name things for the domain, not the implementation
- Prefer composition over inheritance
- One concept per file — if a file does two unrelated things, split it

## Project-Specific

- ALWAYS return `Result<_, AppError>` from handlers — never `unwrap`/`expect`/`panic!` on request-path code
- ALWAYS route new error cases through an existing `AppError` variant (`DatabaseError`, `ValidateError`, `NotFound`, `InternalError`) before adding a new one
- New routes go in `src/handlers/<domain>.rs`, re-exported via `src/handlers/mod.rs` (matches `lobby.rs` pattern)
- New request/response types go in `src/models/<domain>.rs`, re-exported via `src/models/mod.rs`
- Validation lives on the request struct as a `validate(&self) -> Result<(), String>` method (see `CreateLobbyRequest`), called from the handler before use
- Don't wire up sqlx/DB calls in handlers — that layer is intentionally commented out in `src/main.rs` until the repo pattern is finalized; ask before uncommenting it
