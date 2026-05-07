## ADDED Requirements

### Requirement: KME defines downstream handoff contracts

KME SHALL define handoff contracts for downstream repositories after the v0.1.0 API boundary is stable.

#### Scenario: Downstream starts KME adoption

- **WHEN** KDV、KLE、KCF、またはKatanAがKME adoptionを始める
- **THEN** it consumes KME-owned public DTOs
- **THEN** it does not depend on KME parser internals
- **THEN** it uses `docs/downstream-handoff.md` as the handoff source

### Requirement: KME prevents parallel metadata schemas

KME SHALL define metadata contracts before downstream repositories implement metadata behavior.

#### Scenario: Editor or export needs metadata

- **WHEN** KLE、KDV、KCF、またはKatanAがmetadata target behaviorを必要とする
- **THEN** it uses the KME metadata contract
- **THEN** it does not create a substitute schema
- **THEN** unresolved or conflicted metadata is not deleted automatically

### Requirement: KME separates KDV viewer/export from KCF external rendering

KME SHALL define KDV as the Markdown viewer/export consumer and KCF as the external rendering consumer.

#### Scenario: KDV and KCF consume KME

- **WHEN** KDV consumes KME
- **THEN** KDV uses `KmeDocument` as viewer/export input
- **WHEN** KCF consumes KME-related information
- **THEN** KCF only receives external rendering inputs or KME metadata purposes needed for external rendering
- **THEN** KCF does not add new HTML/PDF/PNG/JPG export ownership after the KDV boundary is defined

### Requirement: KatanA owns editor-viewer synchronization control

KME SHALL provide synchronization materials without owning synchronization control.

#### Scenario: KatanA coordinates editor and viewer

- **WHEN** KatanA needs to align editor and viewer state
- **THEN** KatanA uses KME node id, source range, line-column, raw snippet, and fingerprint
- **THEN** KatanA sends scroll, selection, or highlight commands to the viewer or editor
- **THEN** KatanA does not send viewer/editor control commands to KME

### Requirement: KME defines repository-specific downstream inputs

KME SHALL define the minimum downstream input per repository.

#### Scenario: Downstream repository consumes KME

- **WHEN** KDV consumes KME
- **THEN** KDV uses `KmeDocument` as viewer/export input
- **WHEN** KLE consumes KME
- **THEN** KLE uses `MetadataReconcileRequest` and `MetadataReconcileResult` for save-time metadata sync
- **WHEN** KCF consumes KME-related inputs
- **THEN** KCF treats Mermaid、Draw.io、PlantUML、math as external rendering inputs
