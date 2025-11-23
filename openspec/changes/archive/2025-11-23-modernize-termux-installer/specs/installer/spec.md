## MODIFIED Requirements

### Requirement: Real Progress Bar Implementation
The system SHALL implement actual progress tracking for downloads and extractions instead of simulating progress.

#### Scenario: Track download progress with Content-Length
- **WHEN** downloading a distribution file
- **THEN** the system SHALL read the Content-Length header
- **AND** calculate the download percentage based on bytes received
- **AND** update the progress bar in real-time

#### Scenario: Track extraction progress
- **WHEN** extracting a tar.xz archive
- **THEN** the system SHALL count the total number of files in the archive
- **AND** update the progress bar as each file is extracted
- **AND** display the current file being extracted

#### Scenario: Handle unknown content length
- **WHEN** downloading a file without Content-Length header
- **THEN** the system SHALL display an indeterminate progress indicator
- **AND** show the downloaded size in MB/GB

### Requirement: Post-Installation Launch Prompt
The system SHALL ask the user whether to start the installed system immediately after installation completes.

#### Scenario: Prompt user after successful installation
- **WHEN** installation completes successfully
- **THEN** the system SHALL display a prompt asking "是否立即启动系统？"
- **AND** provide Yes/No options

#### Scenario: User chooses to start immediately
- **WHEN** user selects "Yes" to start the system
- **THEN** the system SHALL execute the start.sh script for the installed system
- **AND** enter the Linux environment

#### Scenario: User chooses not to start
- **WHEN** user selects "No" to start the system
- **THEN** the system SHALL return to the main menu
- **AND** display the manual start command for future reference

### Requirement: Distribution-Specific Initialization Commands
The system SHALL support {distro}-init configuration in the config file to execute custom commands during installation.

#### Scenario: Parse {distro}-init from config
- **WHEN** reading the configuration file
- **THEN** the system SHALL parse {distro}-init entries for each distribution
- **AND** store the initialization commands

#### Scenario: Execute single-line init command
- **WHEN** installing a distribution with a single-line {distro}-init command
- **THEN** the system SHALL execute the command inside the installed system
- **AND** log the execution result

#### Scenario: Execute multi-line init script
- **WHEN** installing a distribution with a multi-line {distro}-init command (using --- --- format)
- **THEN** the system SHALL create a temporary script file
- **AND** execute it inside the installed system according to the shebang
- **AND** clean up the temporary file after execution

#### Scenario: Skip init if not configured
- **WHEN** installing a distribution without {distro}-init configuration
- **THEN** the system SHALL skip the initialization step
- **AND** continue with the standard installation process

### Requirement: Rename "Minimal Installation" to "Minimal Install"
The system SHALL use "最小安装" instead of "最小化安装" in all UI text.

#### Scenario: Display minimal install option
- **WHEN** showing installation mode options
- **THEN** the system SHALL display "最小安装" as the option text
- **AND** use this term in all language files

#### Scenario: Update language files
- **WHEN** language files are loaded
- **THEN** the system SHALL use "最小安装" for the minimal installation mode
- **AND** update both Chinese and English translations
