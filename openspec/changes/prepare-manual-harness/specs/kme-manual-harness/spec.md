## ADDED Requirements

### Requirement: KME provides a manual harness

KME SHALL provide a manual harness for release-before visual inspection.

#### Scenario: Developer starts the harness

- **WHEN** a developer runs `just harness-up`
- **THEN** a manual inspection environment starts
- **THEN** the environment shows Markdown input and KME document model output

### Requirement: KME keeps the harness outside the published library

KME MUST keep the manual harness outside the published library contract.

#### Scenario: KME is packaged

- **WHEN** `cargo package --locked --allow-dirty --list` is run
- **THEN** product CLI and product UI targets are not added to the KME crate
- **THEN** harness-only files do not become KME public API

### Requirement: KME uses harness confirmation as a release quality gate

KME SHALL treat manual harness confirmation as part of release readiness.

#### Scenario: v0.1.0 release is prepared

- **WHEN** KME prepares the `v0.1.0` release
- **THEN** representative fixtures are checked through `just harness-up`
- **THEN** the confirmation result is recorded before release
