# config Specification

## Purpose
TBD - created by archiving change modernize-architecture-and-ui. Update Purpose after archive.
## Requirements
### Requirement: Custom Shell Command Configuration
The system SHALL support custom shell commands in the configuration file for execution at login.

#### Scenario: Parse shell configuration
- **WHEN** a config file contains a `shell` multi-line string
- **THEN** the system SHALL parse and store the custom shell commands

#### Scenario: Execute custom shell commands at login
- **WHEN** a user logs into an installed system
- **AND** custom shell commands are configured
- **THEN** the system SHALL execute the custom commands before starting the shell

#### Scenario: Handle empty shell configuration
- **WHEN** no shell commands are configured
- **THEN** the system SHALL start the default shell without custom commands

### Requirement: Configuration File Location
The system SHALL use the new configuration directory structure.

#### Scenario: Load config from new location
- **WHEN** the system starts
- **THEN** it SHALL look for configuration in `$HOME/termos/config`

#### Scenario: Create default config
- **WHEN** no config file exists at the new location
- **THEN** the system SHALL create a default config file with standard settings

### Requirement: Mirror Configuration
The system SHALL continue to support mirror URL configuration for all distributions.

#### Scenario: Load Ubuntu mirror
- **WHEN** installing Ubuntu
- **AND** `ubuntu-mirror` is configured
- **THEN** the system SHALL use the configured mirror URL

#### Scenario: Load custom download link
- **WHEN** installing a distribution
- **AND** a custom download link is configured (e.g., `ubuntu-link`)
- **THEN** the system SHALL use the custom link instead of the mirror

#### Scenario: Fallback to default mirror
- **WHEN** a custom download link is not configured or fails
- **THEN** the system SHALL fall back to the configured mirror URL
- **AND** if no mirror is configured, use the built-in default

### Requirement: Configuration Directory Path
The system SHALL use `$HOME/termos/` as the configuration directory.

#### Scenario: Create config in new directory
- **WHEN** the system creates a configuration directory
- **THEN** it SHALL create it at `$HOME/termos/`

#### Scenario: Read existing config from old location
- **WHEN** no config exists at `$HOME/termos/config`
- **AND** a config exists at `$HOME/Ostermux/config`
- **THEN** the system SHALL read from the old location
- **AND** display a migration warning to the user

### Requirement: Default Mirror URLs
The system SHALL use optimized mirror URLs for better performance.

#### Scenario: Use optimized Ubuntu mirror
- **WHEN** installing Ubuntu without custom configuration
- **THEN** the system SHALL use `https://mirrors.ustc.edu.cn/ubuntu/`

#### Scenario: Use optimized Debian mirror
- **WHEN** installing Debian without custom configuration  
- **THEN** the system SHALL use `https://mirrors.163.com/debian/`

#### Scenario: Use optimized Kali mirror
- **WHEN** installing Kali without custom configuration
- **THEN** the system SHALL use `http://http.kali.org/kali/`

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

