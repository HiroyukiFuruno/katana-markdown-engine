## ADDED Requirements

### Requirement: KME defines canonical fixtures

KME SHALL define canonical fixtures for current KatanA-supported Markdown behavior.

#### Scenario: Parse canonical sample

- **WHEN** KME parses the canonical KatanA sample fixture
- **THEN** the resulting document contains the expected KME-owned node kinds
- **THEN** each asserted node keeps source range, raw snippet, and fingerprint

### Requirement: KME avoids absolute-path test dependency

KME MUST NOT require a developer-specific absolute path to run standard fixture tests.

#### Scenario: Run fixture tests in a clean checkout

- **WHEN** fixture tests run in the KME repository
- **THEN** the tests use repository-local fixture inputs or a documented sync result
- **THEN** tests do not require `/Users/hiroyuki_furuno/works/private/katana` to exist
