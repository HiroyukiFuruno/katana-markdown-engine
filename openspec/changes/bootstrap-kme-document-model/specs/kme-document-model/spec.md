## ADDED Requirements

### Requirement: KME exposes a renderer-neutral document model

KME SHALL parse supported Markdown fixtures into a renderer-neutral document model.

#### Scenario: Parse canonical fixture

- **WHEN** KME parses KatanA `assets/fixtures/sample.md`
- **THEN** the resulting model contains block and inline nodes for current KatanA-supported Markdown behavior
- **THEN** each node has a stable id, source range, line-column range, and raw snippet

### Requirement: KME uses shared AST lint governance

KME SHALL use the shared P0 AST lint governance as its quality gate.

#### Scenario: Run KME quality gate

- **WHEN** KME runs AST lint
- **THEN** it uses `katana-ast-lint` common rules through a KME repository adapter
- **THEN** KME does not introduce a repository-local lint drift as the baseline

### Requirement: KME keeps metadata external

KME MUST treat metadata as an external document associated with Markdown source.

#### Scenario: Resolve metadata target

- **WHEN** KME receives Markdown source and `README.md.metadata.json`
- **THEN** KME resolves metadata targets to document nodes where possible
- **THEN** unresolved targets are returned without deletion

### Requirement: KME does not expose parser internals

KME MUST NOT expose third-party parser AST types as its public contract.

#### Scenario: Downstream consumes KME model

- **WHEN** kdp, kle, kcf, or KatanA consumes KME output
- **THEN** it receives KME-owned DTOs
- **THEN** it does not depend on Comrak, pulldown-cmark, markdown-rs, or vendor parser internals
