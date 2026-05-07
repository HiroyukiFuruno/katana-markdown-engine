## ADDED Requirements

### Requirement: KME treats v0.1.1 as precision hardening

KME SHALL use `v0.1.1` for precision improvements discovered after `v0.1.0` adoption.

#### Scenario: Downstream adoption finds a parsing issue

- **WHEN** KDV, KLE, or KatanA finds a KME parsing mismatch after adopting `v0.1.0`
- **THEN** KME records the issue as a `v0.1.1` candidate
- **THEN** KME reproduces it with a repository-local fixture before changing implementation

### Requirement: KME keeps the renderer-neutral boundary

KME MUST NOT add renderer, export, UI, or synchronization control responsibilities in `v0.1.1`.

#### Scenario: A downstream repository needs rendered output

- **WHEN** rendered Markdown, HTML, PDF, PNG, JPG, scroll, selection, or highlight behavior is required
- **THEN** KME does not implement that behavior
- **THEN** KME only returns document model data, source mapping, metadata resolution, or synchronization materials

### Requirement: KME preserves metadata resolution safety

KME SHALL improve metadata target matching without weakening unresolved or conflict handling.

#### Scenario: Metadata target recovery is ambiguous

- **WHEN** KME finds multiple plausible targets for one metadata entry
- **THEN** KME returns conflict
- **THEN** KME does not silently choose one candidate

### Requirement: KME evaluates new sync anchor materials conservatively

KME SHALL add synchronization anchor materials only when existing public DTO fields are insufficient.

#### Scenario: KatanA cannot map editor and viewer nodes with existing fields

- **WHEN** node id, source range, line-column, raw snippet, and fingerprint are insufficient for synchronization mapping
- **THEN** KME may add renderer-neutral anchor material to the public DTO
- **THEN** KME still does not control editor or viewer synchronization
