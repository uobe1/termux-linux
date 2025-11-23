# cli Specification

## Purpose
TBD - created by archiving change modernize-architecture-and-ui. Update Purpose after archive.
## Requirements
### Requirement: Command-Line Argument Parsing
The system SHALL provide command-line argument parsing for all operations including install, uninstall, list, and help.

#### Scenario: Parse install command
- **WHEN** user executes `insOs --install ubuntu`
- **THEN** the system SHALL parse the distribution type and installation parameters
- **AND** initiate the installation process

#### Scenario: Parse uninstall command
- **WHEN** user executes `insOs --uninstall ubuntu1`
- **THEN** the system SHALL parse the system ID
- **AND** initiate the uninstallation process

#### Scenario: Parse list command
- **WHEN** user executes `insOs --list`
- **THEN** the system SHALL display all installed systems

### Requirement: Language Selection Flag
The system SHALL support a `--lang` flag to manually select the interface language.

#### Scenario: Select English language
- **WHEN** user executes `insOs --lang en`
- **THEN** the system SHALL display all text in English

#### Scenario: Select Chinese language
- **WHEN** user executes `insOs --lang zh`
- **THEN** the system SHALL display all text in Chinese

#### Scenario: Invalid language code
- **WHEN** user executes `insOs --lang invalid`
- **THEN** the system SHALL fall back to default language (Chinese)
- **AND** display a warning message

### Requirement: No-Color Mode Flag
The system SHALL support a `--no-color` flag to disable ANSI color output.

#### Scenario: Disable colors
- **WHEN** user executes `insOs --no-color`
- **THEN** the system SHALL display all output without ANSI color codes

#### Scenario: Auto-detect no-color
- **WHEN** the terminal does not support colors
- **THEN** the system SHALL automatically disable color output

### Requirement: Interactive Menu Mode
The system SHALL display all available distributions loaded from UrlList.txt in the interactive menu.

#### Scenario: Show dynamic distribution list
- **WHEN** the interactive menu is displayed
- **THEN** the system SHALL show all distributions parsed from UrlList.txt
- **AND** include distributions: adelie, deepin, debian, chimera, opensuse, artix, manjaro, archlinux, void, fedora, ubuntu, rockylinux, alpine, pardus
- **AND** display them in a numbered list for user selection

#### Scenario: Handle empty distribution list
- **WHEN** no distributions could be loaded from UrlList.txt
- **THEN** the system SHALL display an error message
- **AND** exit gracefully instead of panicking

### Requirement: Fix Index Out of Bounds Panic
The system SHALL validate user input to prevent index out of bounds errors in CLI operations.

#### Scenario: Validate menu selection range
- **WHEN** user enters a menu selection number
- **THEN** the system SHALL validate that the number is within the valid range
- **AND** display an error if the selection is invalid
- **AND** prompt the user again for valid input

#### Scenario: Validate distribution selection
- **WHEN** user selects a distribution by number
- **THEN** the system SHALL verify the index exists in the distribution list
- **AND** prevent index out of bounds access

#### Scenario: Handle empty input
- **WHEN** user presses Enter without providing input
- **THEN** the system SHALL treat it as invalid input
- **AND** prompt the user again

### Requirement: Architecture Display in CLI
The system SHALL display the detected architecture in CLI output where relevant.

#### Scenario: Show architecture in system info
- **WHEN** displaying system information or installation details
- **THEN** the system SHALL include the detected architecture (e.g., aarch64, x86_64)
- **AND** use it to filter compatible distributions

#### Scenario: Filter distributions by architecture
- **WHEN** displaying available distributions
- **THEN** the system SHALL only show distributions that have URLs for the detected architecture
- **AND** hide distributions that are not supported on the current architecture

### Requirement: Help Display
The system SHALL provide a help command that displays usage information.

#### Scenario: Display help
- **WHEN** user executes `insOs --help`
- **THEN** the system SHALL display usage information including all available commands and options
- **AND** show examples of common operations

### Requirement: Executable Name
The system SHALL use `insOs` as the executable name.

#### Scenario: Execute renamed binary
- **WHEN** user executes `insOs`
- **THEN** the system SHALL respond to the command
- **AND** display the interactive menu or process arguments as specified

#### Scenario: Execute with arguments
- **WHEN** user executes `insOs --install ubuntu`
- **THEN** the system SHALL parse the arguments correctly
- **AND** initiate the Ubuntu installation

### Requirement: Installation Directory Reference
The system SHALL reference `$HOME/termos/` as the installation directory.

#### Scenario: Install to new directory
- **WHEN** user installs a new system
- **THEN** the system SHALL create and use `$HOME/termos/` as the base directory

#### Scenario: Check installation directory
- **WHEN** listing installed systems
- **THEN** the system SHALL scan `$HOME/termos/` for system directories

