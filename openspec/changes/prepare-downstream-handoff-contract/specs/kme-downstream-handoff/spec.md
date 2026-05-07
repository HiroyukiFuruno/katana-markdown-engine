## ADDED Requirements

### Requirement: KME defines downstream handoff contracts

KME SHALL define handoff contracts for downstream repositories after the v0.1.0 API boundary is stable.

#### Scenario: Downstream starts KME adoption

- **WHEN** kdp, kle, kcf, or KatanA starts KME adoption
- **THEN** it consumes KME-owned public DTOs
- **THEN** it does not depend on KME parser internals
- **THEN** it uses `docs/downstream-handoff.md` as the handoff source

### Requirement: KME prevents parallel metadata schemas

KME SHALL define metadata contracts before downstream repositories implement metadata behavior.

#### Scenario: Editor or export needs metadata

- **WHEN** kle or kcf needs metadata target behavior
- **THEN** it uses the KME metadata contract
- **THEN** it does not create a substitute schema
- **THEN** unresolved or conflicted metadata is not deleted automatically

### Requirement: KME keeps KCF pending until prerequisites are stable

KME SHALL keep kcf handoff pending until KME model and metadata boundaries are stable.

#### Scenario: KCF export planning resumes

- **WHEN** kcf resumes export or paging integration
- **THEN** KME public DTOs are already fixed
- **THEN** KME metadata schema is already fixed
- **THEN** KUW or an explicit widget boundary is already defined

### Requirement: KME defines repository-specific downstream inputs

KME SHALL define the minimum downstream input per repository.

#### Scenario: Downstream repository consumes KME

- **WHEN** kdp consumes KME
- **THEN** kdp uses `KmeDocument` as preview input
- **WHEN** kle consumes KME
- **THEN** kle uses `MetadataReconcileRequest` and `MetadataReconcileResult` for save-time metadata sync
- **WHEN** kcf consumes KME
- **THEN** kcf uses KME metadata payload purposes for export and paging
