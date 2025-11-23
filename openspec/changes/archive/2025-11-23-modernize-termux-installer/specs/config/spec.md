## MODIFIED Requirements

### Requirement: Distribution-Specific Initialization Configuration
The system SHALL support {distro}-init configuration entries in the config file for executing custom commands during installation.

#### Scenario: Parse single-line {distro}-init command
- **WHEN** the config file contains a line like `ubuntu-init = apt update && apt upgrade -y`
- **THEN** the system SHALL parse it as a single-line initialization command for Ubuntu
- **AND** store it for execution during Ubuntu installation

#### Scenario: Parse multi-line {distro}-init script
- **WHEN** the config file contains a multi-line {distro}-init entry using --- --- format:
  ```
  debian-init = ---
  #!/bin/bash
  apt update
  apt install -y vim git
  ---
  ```
- **THEN** the system SHALL parse the content between the --- markers as a script
- **AND** preserve the shebang and all commands

#### Scenario: Parse {distro}-init with different distributions
- **WHEN** the config file contains multiple {distro}-init entries like `ubuntu-init`, `debian-init`, `arch-init`
- **THEN** the system SHALL parse each entry separately
- **AND** associate each with the correct distribution

### Requirement: Config File Format for Initialization Scripts
The system SHALL support a specific format for multi-line initialization scripts in the config file.

#### Scenario: Validate --- --- format
- **WHEN** parsing a {distro}-init entry with --- --- markers
- **THEN** the system SHALL validate that the opening and closing markers exist
- **AND** extract the content between them
- **AND** treat it as a script to be executed

#### Scenario: Handle missing closing marker
- **WHEN** a {distro}-init entry has an opening --- but no closing ---
- **THEN** the system SHALL log a parsing error
- **AND** skip that initialization command
- **AND** continue processing other config entries

#### Scenario: Support comments in init scripts
- **WHEN** a {distro}-init script contains lines starting with #
- **THEN** the system SHALL preserve these comment lines in the script
- **AND** execute them as part of the script (if the interpreter supports comments)

### Requirement: Default Mirror Configuration
The system SHALL continue to support mirror configuration for distributions while adding {distro}-init support.

#### Scenario: Mirror and init config coexist
- **WHEN** the config file contains both mirror URLs and {distro}-init commands
- **THEN** the system SHALL parse both types of configuration
- **AND** use mirrors for downloading
- **AND** use init commands for post-installation setup

#### Scenario: Override UrlList.txt with mirror config
- **WHEN** a distribution has both a URL in UrlList.txt and a mirror config in the config file
- **THEN** the system SHALL prefer the mirror config URL
- **AND** use it for downloading the distribution
