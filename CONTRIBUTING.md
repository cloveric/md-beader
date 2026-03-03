# Contributing

Thank you for your interest in improving Markdown Studio.

## Development Setup

```powershell
cd C:\Users\hangw\md-reader
cargo test
cargo build --release
```

## Contribution Workflow

1. Create a branch from `main`.
2. Make focused changes with clear commit messages.
3. Run `cargo test` and ensure all tests pass.
4. Update docs when behavior or UX changes.
5. Open a pull request with:
   - problem statement
   - solution summary
   - verification steps

## Coding Notes

- Keep UI text concise and readable for Chinese users.
- Prefer offline-first behavior and avoid external runtime dependencies.
- Keep IPC contracts (`src/desktop.rs`) backward-compatible when possible.

## Commit Message Suggestion

Use a clear prefix:

- `feat:` new capability
- `fix:` bug fix
- `docs:` documentation only
- `refactor:` internal restructuring
- `chore:` maintenance

Example:

```text
feat: add new-file action and Ctrl+N shortcut
```
