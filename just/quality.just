# Apply rustfmt
fmt:
    cargo fmt --all

# Check rustfmt output
fmt-check:
    cargo fmt --all -- --check

# Run Clippy with zero warnings
lint:
    cargo clippy -j {{JOBS}} --workspace --all-targets --all-features --locked -- -D warnings

# Run repository AST lint through the shared KAL crate
ast-lint:
    cargo test -j {{JOBS}} --test repository_ast_lint --locked -- --nocapture

# Run unit and integration tests
test:
    cargo test --workspace --locked

# Validate active OpenSpec changes
openspec-check:
    @for change_dir in openspec/changes/*; do \
        [ -d "$change_dir" ] || continue; \
        change="$(basename "$change_dir")"; \
        [ "$change" = "archive" ] && continue; \
        scripts/openspec validate "$change" --strict; \
    done

# Local quality gate
check: fmt-check lint ast-lint test openspec-check
    @echo "All checks passed"

# Release readiness gate without publishing
release-check: check
    cargo package --locked --allow-dirty
    cargo publish --dry-run --locked --allow-dirty
