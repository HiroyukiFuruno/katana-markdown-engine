# kme-release-readiness Specification

## Purpose
TBD - created by archiving change stabilize-release-readiness-gates. Update Purpose after archive.
## Requirements
### Requirement: KME defines a release readiness gate

KME SHALL provide a release readiness gate before crates.io publication.

#### Scenario: Run release readiness locally

- **WHEN** a developer runs `just release-check`
- **THEN** KME runs the normal quality gate
- **THEN** KME packages the crate with `cargo package --locked --allow-dirty`
- **THEN** KME runs `cargo publish --dry-run --locked --allow-dirty`
- **THEN** KME does not publish the crate

### Requirement: KME keeps release preflight wired into CI

KME SHALL run release readiness checks in GitHub Actions before release branches can merge.

#### Scenario: Release pull request is opened

- **WHEN** a pull request targets `master`
- **THEN** the release preflight workflow runs the release readiness gate
- **THEN** branch protection can require the workflow check by name

### Requirement: KME uses library-only release checks

KME MUST NOT add distribution checks that belong to non-library channels unless a later OpenSpec changes that boundary.

#### Scenario: Release readiness is expanded

- **WHEN** KME adds release readiness checks
- **THEN** npm, PyPI, Homebrew, binary artifact, MCPB, and editor extension checks are out of scope
- **THEN** KME keeps the release gate focused on the Rust library crate

