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

