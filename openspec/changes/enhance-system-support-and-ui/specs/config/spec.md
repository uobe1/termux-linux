## ADDED Requirements

### Requirement: Distribution-Specific Initialization Configuration
The system SHALL support {distro}-init configuration entries in the config file to define custom initialization commands for specific distributions that execute once during installation.

#### Scenario: Parse Ubuntu init commands
- **WHEN** config file contains `ubuntu-init = ---
apt update
apt install -y vim
---`
- **THEN** the system SHALL parse and store the multi-line init commands for Ubuntu

#### Scenario: Parse single-line init command
- **WHEN** config file contains `debian-init = apt install -y curl`
- **THEN** the system SHALL parse and store the single-line init command for Debian

#### Scenario: Execute init commands during installation
- **WHEN** installing Ubuntu
- **AND** ubuntu-init commands are configured
- **THEN** the system SHALL execute the init commands after system extraction
- **AND** before creating the start script

#### Scenario: Skip init commands for distributions without configuration
- **WHEN** installing Fedora
- **AND** no fedora-init configuration exists
- **THEN** the system SHALL skip init command execution
- **AND** continue with normal installation

#### Scenario: Handle init command execution failure
- **WHEN** executing init commands during installation
- **AND** a command fails
- **THEN** the system SHALL log the error
- **AND** continue with installation (treat as non-fatal)

#### Scenario: Init commands execute only once
- **WHEN** a system is installed with init commands
- **THEN** the commands SHALL execute during installation only
- **AND** SHALL NOT execute on subsequent logins via start.sh
