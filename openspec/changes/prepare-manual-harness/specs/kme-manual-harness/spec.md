## ADDED Requirements

### Requirement: KME provides a structure inspection harness

KME SHALL provide a development-only harness for structure inspection.

#### Scenario: Developer starts the harness

- **WHEN** a developer runs `just harness-up`
- **THEN** a structure inspection environment starts
- **THEN** the environment shows Markdown input, KME nodes, selected node source mapping, and metadata resolution details
- **THEN** the default input is `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` when it exists

### Requirement: KME keeps the harness outside the published library

KME MUST keep the manual harness outside the published library contract.

#### Scenario: KME is packaged

- **WHEN** `cargo package --locked --allow-dirty --list` is run
- **THEN** product CLI and product UI targets are not added to the KME crate
- **THEN** harness-only files do not become KME public API

### Requirement: KME uses harness confirmation as a release quality gate

KME SHALL treat fixture tests as the release readiness source of truth.

#### Scenario: v0.1.0 release is prepared

- **WHEN** KME prepares the `v0.1.0` release
- **THEN** representative fixtures are checked through automated tests
- **THEN** `just harness-up` remains optional inspection support
