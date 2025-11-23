# installer Specification

## Purpose
TBD - created by archiving change modernize-architecture-and-ui. Update Purpose after archive.
## Requirements
### Requirement: Real User and Group Detection
The system SHALL detect the actual user and group information instead of using hardcoded values.

#### Scenario: Detect current user
- **WHEN** starting an installation
- **THEN** the system SHALL detect the actual username using system calls

#### Scenario: Detect user groups
- **WHEN** configuring a new installation
- **THEN** the system SHALL detect the user's actual group memberships

#### Scenario: Check write permissions
- **WHEN** preparing to install to a directory
- **THEN** the system SHALL verify actual write permissions
- **AND** display an error if permissions are insufficient

### Requirement: Enhanced Installation Modes
The system SHALL support multiple installation modes with different package selections.

#### Scenario: Perform minimal installation
- **WHEN** user selects minimal installation mode
- **THEN** the system SHALL install only essential packages

#### Scenario: Perform standard installation
- **WHEN** user selects standard installation mode  
- **THEN** the system SHALL install commonly used packages

#### Scenario: Perform custom installation
- **WHEN** user selects custom installation mode
- **THEN** the system SHALL prompt for package selection
- **AND** install only the selected packages

### Requirement: Progress Feedback with New Style
The system SHALL provide installation progress using the new ASCII art progress bar.

#### Scenario: Display download progress
- **WHEN** downloading distribution files
- **THEN** the system SHALL display progress using the new progress bar style

#### Scenario: Display extraction progress
- **WHEN** extracting the downloaded archive
- **THEN** the system SHALL display progress using the new progress bar style

#### Scenario: Display configuration progress
- **WHEN** configuring the installed system
- **THEN** the system SHALL display progress updates

### Requirement: Internationalized Installation Messages
The system SHALL display all installation messages in the selected language.

#### Scenario: Display Chinese installation messages
- **WHEN** language is set to Chinese
- **THEN** all installation prompts and status messages SHALL be in Chinese

#### Scenario: Display English installation messages
- **WHEN** language is set to English
- **THEN** all installation prompts and status messages SHALL be in English

### Requirement: Installation Directory Structure
The system SHALL install systems to `$HOME/termos/{system-id}/`.

#### Scenario: Install to new directory structure
- **WHEN** installing a new system
- **THEN** the system SHALL create the directory at `$HOME/termos/{system-id}/`

#### Scenario: Check installation directory
- **WHEN** listing installed systems
- **THEN** the system SHALL scan `$HOME/termos/` for system directories

### Requirement: Default System Configuration
The system SHALL use optimized default settings for better user experience.

#### Scenario: Apply optimized defaults
- **WHEN** installing with default settings
- **THEN** the system SHALL configure the system with optimized package selections
- **AND** set appropriate environment variables

#### Scenario: Minimal installation mode
- **WHEN** user selects minimal installation
- **THEN** the system SHALL install only essential packages
- **AND** skip additional configuration steps

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

