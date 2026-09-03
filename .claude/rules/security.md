# Security

Constraints for production code. These are non-negotiable.

## Data Handling

- NEVER log PII, credentials, tokens, or API keys
- NEVER commit secrets to git — use environment variables
- NEVER hardcode connection strings, passwords, or keys
- Validate all user input at system boundaries
- Sanitize output to prevent XSS

## Database

- ALWAYS use parameterized queries — NEVER string concatenation for SQL
- Apply principle of least privilege for database roles
- Escape user input in NoSQL queries

## Authentication & Authorization

- NEVER store passwords in plaintext — use bcrypt or argon2
- NEVER expose internal IDs in URLs without authorization checks
- Validate JWTs on every request, not just at login

## Project-Specific

- No authentication exists yet — don't add auth-gated logic without discussing the approach first
- `CorsLayer::permissive()` is set in `src/main.rs` — fine for local dev, but flag this before any production deploy; it should be scoped to known origins
- Secrets are managed via Shuttle's `Secrets.toml` / `Secrets.dev.toml` (gitignored) — never hardcode credentials in source
- Database/Authentication sections above don't apply yet — no DB or auth layer is wired up (see `src/main.rs`); revisit once sqlx integration lands
