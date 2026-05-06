# Quality Gates

## Local Targets

| Target | Responsibility | Blocking |
| --- | --- | --- |
| `just fmt-check` | Verify rustfmt output is committed | Yes |
| `just lint` | Run Clippy with zero warnings | Yes |
| `just ast-lint` | Run shared AST lint through KAL | Yes |
| `just test` | Run unit and integration tests | Yes |
| `just openspec-check` | Validate active OpenSpec change | Yes |
| `just check` | Run all local gates above | Yes |

## AST Lint

KME uses `katana-ast-lint` as the shared governance gate. The repository-specific
entry point is `tests/repository_ast_lint.rs`.

KME must not introduce a separate local lint baseline to avoid the shared rules.

## Release Readiness

KME is not release-wired yet. Before adding release workflow, keep `just check`
green and extend this document with package and publication checks.
