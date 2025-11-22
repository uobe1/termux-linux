## ADDED Requirements

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
The system SHALL provide an interactive menu mode when no arguments are provided.

#### Scenario: Launch interactive menu
- **WHEN** user executes `insOs` without arguments
- **THEN** the system SHALL display the interactive menu
- **AND** prompt the user for input

#### Scenario: Handle invalid menu choice
- **WHEN** user enters an invalid menu option
- **THEN** the system SHALL display an error message
- **AND** prompt again for valid input

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

