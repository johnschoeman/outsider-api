# Tech Stack

## Language

Rust (edition 2024)

## Framework

Axum 0.8, deployed via Shuttle (`shuttle-axum` / `shuttle-runtime` 0.57)

## Key Libraries

| Library | Purpose | Notes |
|---------|---------|-------|
| serde / serde_json | Serialization | derive feature enabled |
| chrono | Date/time | serde feature enabled |
| tower-http | Middleware | `fs`, `cors` features (CORS currently permissive) |
| reqwest | HTTP client | present in deps, not yet used in handlers |
| futures | Async utilities | present in deps, not yet used |

## Build & Dev

| Command | Purpose |
|---------|---------|
| `nix develop` | Enter build environment (flake.nix) |
| `cargo check` | Static analysis / type check |
| `shuttle dev` | Run API locally via Shuttle |

## Testing

None yet — no test suite exists.

## Database / Storage

Not yet wired up. `AppState.repo` / `OutsiderRepository` and sqlx migrations are commented out in `src/main.rs`, pending integration.
