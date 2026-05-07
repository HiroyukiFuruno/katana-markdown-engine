set shell := ["bash", "-uc"]

JOBS := env_var_or_default("JOBS", "2")

export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings")

[private]
default: help

# Show available recipes
help:
    @just --list --unsorted

import 'just/quality.just'
import 'just/harness.just'
