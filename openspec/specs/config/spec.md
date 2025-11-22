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

