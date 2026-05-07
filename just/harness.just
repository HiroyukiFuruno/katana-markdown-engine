# Build the development-only manual harness
harness-check:
    cargo check --manifest-path tools/manual-harness/Cargo.toml --locked

# Open the development-only manual harness for release-before inspection
harness-up path="":
    if [ -n "{{path}}" ]; then \
        cargo run --manifest-path tools/manual-harness/Cargo.toml --locked -- "{{path}}"; \
    else \
        cargo run --manifest-path tools/manual-harness/Cargo.toml --locked; \
    fi
