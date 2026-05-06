## ADDED Requirements

### Requirement: KME defines downstream handoff contracts

KME SHALL define handoff contracts for downstream repositories after the v0.1.0 API boundary is stable.

#### Scenario: Downstream starts KME adoption

- **WHEN** kdp, kle, kcf, or KatanA starts KME adoption
- **THEN** it consumes KME-owned public DTOs
- **THEN** it does not depend on KME parser internals

### Requirement: KME prevents parallel metadata schemas

KME SHALL define metadata contracts before downstream repositories implement metadata behavior.

#### Scenario: Editor or export needs metadata

- **WHEN** kle or kcf needs metadata target behavior
- **THEN** it uses the KME metadata contract
- **THEN** it does not create a substitute schema

### Requirement: KME keeps KCF pending until prerequisites are stable

KME SHALL keep kcf handoff pending until KME model and metadata boundaries are stable.

#### Scenario: KCF export planning resumes

- **WHEN** kcf resumes export or paging integration
- **THEN** KME public DTOs are already fixed
- **THEN** KME metadata schema is already fixed
