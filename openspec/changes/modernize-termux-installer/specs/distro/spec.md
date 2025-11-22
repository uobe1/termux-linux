## ADDED Requirements

### Requirement: Architecture Auto-Detection and Manual Selection
The system SHALL automatically detect the current architecture, with manual selection as fallback when detection fails.

#### Scenario: Auto-detect architecture successfully
- **WHEN** the system starts
- **THEN** it SHALL detect the current architecture using system calls
- **AND** map it to the appropriate architecture identifier (aarch64, arm, x86_64, i686, riscv64)
- **AND** use the detected architecture for URL selection

#### Scenario: Auto-detection fails, prompt manual selection
- **WHEN** architecture auto-detection fails
- **THEN** the system SHALL prompt the user to manually select the architecture
- **AND** display available architecture options
- **AND** use the user-selected architecture for URL selection

#### Scenario: Architecture-specific URL selection from hardcoded map
- **WHEN** a distribution is selected for installation
- **THEN** the system SHALL use the detected or manually selected architecture
- **AND** look up the corresponding URL from the hardcoded architecture-URL mapping
- **AND** use that URL for downloading the distribution

### Requirement: Hardcoded Distribution Definitions from UrlList.txt
The system SHALL include hardcoded distribution definitions for all distributions listed in UrlList.txt reference file.

#### Scenario: Define adelie distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for adelie distribution
- **AND** include URLs for aarch64, arm, and x86_64 architectures

#### Scenario: Define deepin distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for deepin distribution
- **AND** include URLs for aarch64 and x86_64 architectures

#### Scenario: Define debian distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for debian distribution
- **AND** include URLs for aarch64, arm, x86_64, and i686 architectures

#### Scenario: Define chimera distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for chimera distribution
- **AND** include URLs for aarch64, x86_64, and riscv64 architectures

#### Scenario: Define opensuse distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for opensuse distribution
- **AND** include URLs for aarch64, arm, x86_64, and i686 architectures

#### Scenario: Define artix distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for artix distribution
- **AND** include URL for aarch64 architecture

#### Scenario: Define manjaro distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for manjaro distribution
- **AND** include URL for aarch64 architecture

#### Scenario: Define archlinux distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for archlinux distribution
- **AND** include URLs for aarch64, arm, x86_64, and i686 architectures

#### Scenario: Define void distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for void distribution
- **AND** include URLs for aarch64, arm, x86_64, and i686 architectures

#### Scenario: Define fedora distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for fedora distribution
- **AND** include URLs for aarch64 and x86_64 architectures

#### Scenario: Define ubuntu distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for ubuntu distribution
- **AND** include URLs for aarch64, arm, and x86_64 architectures

#### Scenario: Define rockylinux distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for rockylinux distribution
- **AND** include URLs for aarch64 and x86_64 architectures

#### Scenario: Define alpine distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for alpine distribution
- **AND** include URLs for aarch64, arm, x86_64, i686, and riscv64 architectures

#### Scenario: Define pardus distribution
- **WHEN** the developer references UrlList.txt
- **THEN** the code SHALL contain a hardcoded definition for pardus distribution
- **AND** include URLs for aarch64, x86_64, and i686 architectures

### Requirement: Distribution Metadata Structure
The system SHALL define a structured format for distribution metadata including name, supported architectures, and URLs.

#### Scenario: Define distribution struct
- **WHEN** creating distribution definitions
- **THEN** the code SHALL use a struct with fields for:
  - name: String
  - urls: HashMap<String, String> (architecture -> URL mapping)
  - description: String
  - default_packages: Vec<String>

#### Scenario: Architecture URL mapping
- **WHEN** defining a distribution
- **THEN** the code SHALL map architecture identifiers to their corresponding URLs
- **AND** use the exact URLs from UrlList.txt reference

### Requirement: Manual Code Modification Process
The system SHALL require manual code updates when UrlList.txt reference changes.

#### Scenario: Update distribution URLs
- **WHEN** URLs in UrlList.txt reference are updated
- **THEN** a developer SHALL manually update the hardcoded URLs in the source code
- **AND** the system SHALL NOT automatically read UrlList.txt at runtime

#### Scenario: Add new distribution
- **WHEN** a new distribution is added to UrlList.txt reference
- **THEN** a developer SHALL manually add a new distribution definition to the code
- **AND** include all supported architectures and URLs
