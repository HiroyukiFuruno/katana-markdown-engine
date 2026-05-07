## ADDED Requirements

### Requirement: KME hides parser internals behind an adapter

KME MUST NOT expose parser engine AST types as public API.

#### Scenario: Downstream consumes parsed document

- **WHEN** downstream code uses KME parse output
- **THEN** it receives KME-owned DTOs
- **THEN** it cannot depend on parser engine AST types through the public contract

### Requirement: KME locks parser behavior with contract tests

KME SHALL protect KatanA-supported Markdown behavior with parser contract tests.

#### Scenario: Parser engine changes

- **WHEN** parser implementation changes
- **THEN** table, badge, alert, description list, footnote, diagram, math, and emoji contract tests still pass
- **THEN** source range and raw snippet assertions remain stable

### Requirement: KME preserves emoji model data

KME SHALL preserve Unicode emoji and shortcode emoji as document model data.

#### Scenario: Markdown contains emoji

- **WHEN** Markdown contains Unicode emoji or shortcode emoji
- **THEN** KME keeps the emoji information in KME-owned DTOs
- **THEN** KME does not make rendering or font fallback decisions
