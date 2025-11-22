## MODIFIED Requirements

### Requirement: Interactive Menu Mode
The system SHALL provide an interactive menu mode when no arguments are provided, with robust error handling to prevent index out of bounds panics.

#### Scenario: Handle empty system list in uninstall
- **WHEN** user selects "卸载系统" (uninstall system) from the menu
- **AND** no systems are installed (list is empty)
- **THEN** the system SHALL display a message indicating no systems are available
- **AND** return to the main menu instead of panicking

#### Scenario: Handle valid system selection for uninstall
- **WHEN** user selects "卸载系统" and systems are installed
- **THEN** the system SHALL display the list of installed systems
- **AND** prompt for a valid selection
- **AND** uninstall the selected system

### Requirement: Installation Completion Flow
The system SHALL prompt the user to start the installed system immediately after successful installation.

#### Scenario: Prompt to start after installation
- **WHEN** a system installation completes successfully
- **THEN** the system SHALL ask the user if they want to start the system now
- **AND** if user confirms, start the system automatically
- **AND** if user declines, return to the main menu

#### Scenario: Display start command
- **WHEN** user declines to start immediately
- **THEN** the system SHALL display the command to start the system later
- **AND** show the system directory path

### Requirement: ANSI Color Support
The system SHALL support ANSI colors for enhanced visual feedback in CLI output, with proper handling of the --no-color flag.

#### Scenario: Display colored success messages
- **WHEN** an operation succeeds and --no-color is not specified
- **THEN** the system SHALL display the message in green color (\x1b[32m)

#### Scenario: Display colored error messages
- **WHEN** an operation fails and --no-color is not specified
- **THEN** the system SHALL display the error message in red color (\x1b[31m)

#### Scenario: Display colored warning messages
- **WHEN** displaying a warning and --no-color is not specified
- **THEN** the system SHALL display the warning in yellow color (\x1b[33m)

#### Scenario: Display colored info messages
- **WHEN** displaying information and --no-color is not specified
- **THEN** the system SHALL display the info in blue color (\x1b[34m)

#### Scenario: Respect --no-color flag
- **WHEN** user specifies --no-color flag
- **THEN** the system SHALL display all output without ANSI color codes

#### Scenario: Auto-detect terminal color support
- **WHEN** terminal does not support colors
- **THEN** the system SHALL automatically disable color output

### Requirement: Enhanced Installation Modes
The system SHALL rename "最小化安装" to "最小安装" for better clarity and conciseness.

#### Scenario: Display minimal installation option
- **WHEN** displaying installation mode options
- **THEN** the system SHALL show "最小安装" instead of "最小化安装"
- **AND** maintain the same functionality as before
