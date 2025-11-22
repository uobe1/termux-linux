## ADDED Requirements

### Requirement: UrlList.txt Distribution Support
The system SHALL parse and support all distributions defined in UrlList.txt, mapping architecture-specific URLs to distribution installation requests.

#### Scenario: Parse UrlList.txt format
- **WHEN** the system reads UrlList.txt
- **THEN** it SHALL parse the format: `**{distro}**:`, `URL['{arch}']="{url}"`
- **AND** extract distribution names and their architecture-specific URLs

#### Scenario: Support all UrlList.txt distributions
- **WHEN** parsing UrlList.txt
- **THEN** the system SHALL recognize all distributions: adelie, deepin, debian, chimera, opensuse, artix, manjaro, archlinux, void, fedora, ubuntu, rockylinux, alpine, pardus
- **AND** ignore the .sample entry

#### Scenario: Map current architecture to URL
- **WHEN** user requests to install a distribution
- **THEN** the system SHALL detect the current architecture (aarch64, arm, x86_64, i686, riscv64)
- **AND** select the appropriate URL from UrlList.txt for that architecture

#### Scenario: Handle architecture-specific availability
- **WHEN** a distribution doesn't support the current architecture
- **THEN** the system SHALL display a clear error message
- **AND** list which architectures are supported for that distribution

### Requirement: Distribution Metadata Management
The system SHALL maintain metadata for all supported distributions, including available architectures and default configurations.

#### Scenario: Store distribution URLs by architecture
- **WHEN** UrlList.txt is parsed
- **THEN** the system SHALL store a mapping of distribution → architecture → URL
- **AND** make it available for installation operations

#### Scenario: Validate distribution availability
- **WHEN** user requests an unsupported distribution/architecture combination
- **THEN** the system SHALL validate against the parsed UrlList.txt data
- **AND** provide helpful error messages

### Requirement: Dynamic Distribution Discovery
The system SHALL dynamically discover available distributions from UrlList.txt at runtime, rather than using a hardcoded list.

#### Scenario: Auto-populate distribution list
- **WHEN** the system starts
- **THEN** it SHALL read and parse UrlList.txt
- **AND** dynamically build the list of available distributions

#### Scenario: Update distribution list without code changes
- **WHEN** UrlList.txt is updated with new distributions or URLs
- **THEN** the system SHALL automatically recognize the changes on next run
- **AND** no code modification is required
