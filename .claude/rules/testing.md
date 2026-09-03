# Testing Standards

## Principles

- Write tests alongside implementation, not after
- Test behavior, not implementation details — tests should survive refactoring
- One assertion per test when practical
- Descriptive test names that describe the scenario: `rejects expired tokens` not `test auth`
- Mock external services (APIs, databases), not internal modules
- ALWAYS run the full test suite before considering work complete

## Verification

After code changes, run:
```bash
cargo check    # Type check / static analysis
```

No test suite or linter is configured yet.

## Project-Specific

<!-- Add project-specific testing patterns below. Examples: -->
<!-- - Use [test framework] for all tests -->
<!-- - Integration tests go in tests/integration/, unit tests next to source files -->
<!-- - Use [factory/fixture library] for test data -->
