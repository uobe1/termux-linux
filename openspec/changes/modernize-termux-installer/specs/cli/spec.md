## MODIFIED Requirements

### Requirement: Interactive Menu Distribution Display
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
