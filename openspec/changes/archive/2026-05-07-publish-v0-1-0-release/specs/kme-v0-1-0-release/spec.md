## ADDED Requirements

### Requirement: KME releases v0.1.0 only after prerequisite changes are complete

KME MUST publish `v0.1.0` only after all prerequisite OpenSpec changes are complete.

#### Scenario: v0.1.0 release is requested early

- **WHEN** any prerequisite OpenSpec change still has incomplete release-blocking tasks
- **THEN** KME does not publish `v0.1.0`
- **THEN** the remaining prerequisite is reported instead

### Requirement: KME uses master-based release branches after v0.1.0

KME SHALL use KML-style release branches with `master` as the default branch.

#### Scenario: Prepare a release after v0.1.0

- **WHEN** KME prepares a release
- **THEN** the integration branch is named `release/vX.Y.Z`
- **THEN** any auxiliary branch is named `feature/vX.Y.Z-<short-slug>`
- **THEN** the release pull request targets `master`

### Requirement: KME verifies publication after release

KME SHALL verify GitHub Release and crates.io publication after `v0.1.0` is published.

#### Scenario: v0.1.0 publication completes

- **WHEN** the release workflow finishes
- **THEN** KME verifies the GitHub Release target
- **THEN** KME verifies that crates.io exposes `katana-markdown-engine` version `0.1.0`

### Requirement: KME documents release execution before publication

KME SHALL document release execution before `v0.1.0` is published.

#### Scenario: Release PR is prepared

- **WHEN** `release/v0.1.0` is prepared for `master`
- **THEN** `docs/release-runbook.md` defines the release PR, secret, publication, failure, verify, and branch hygiene steps
- **THEN** `docs/release-notes/v0.1.0.md` exists
- **THEN** real publication tasks remain incomplete until after merge
