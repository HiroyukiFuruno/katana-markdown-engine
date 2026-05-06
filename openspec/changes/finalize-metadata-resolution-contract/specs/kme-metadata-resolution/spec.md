## ADDED Requirements

### Requirement: KME exposes complete metadata resolution states

KME SHALL expose metadata resolution states that downstream repositories can consume without inventing a parallel schema.

#### Scenario: Resolve metadata after save

- **WHEN** KME compares old Markdown, new Markdown, and metadata targets
- **THEN** each target is returned as resolved, moved, unresolved, or conflicted
- **THEN** the result uses KME-owned public DTOs

### Requirement: KME preserves unresolved metadata

KME MUST preserve unresolved metadata entries.

#### Scenario: Target cannot be matched

- **WHEN** a metadata target cannot be resolved after Markdown changes
- **THEN** KME returns an unresolved result
- **THEN** KME does not delete the metadata entry

### Requirement: KME reports ambiguous metadata as conflict

KME SHALL report ambiguous target recovery as a conflict.

#### Scenario: Multiple candidate nodes match a target

- **WHEN** KME finds multiple plausible nodes for one metadata target
- **THEN** KME returns a conflict result
- **THEN** KME does not silently choose one candidate
