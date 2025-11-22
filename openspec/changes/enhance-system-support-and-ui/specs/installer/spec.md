## MODIFIED Requirements

### Requirement: Multiple Distribution Support
The system SHALL support all distributions defined in UrlList.txt, including adelie, deepin, debian, chimera, opensuse, artix, manjaro, archlinux, void, fedora, ubuntu, rockylinux, alpine, pardus, across multiple architectures (aarch64, arm, x86_64, i686, riscv64).

#### Scenario: Install Alpine Linux from UrlList.txt
- **WHEN** user selects to install alpine
- **THEN** the system SHALL parse UrlList.txt
- **AND** extract the correct URL for the current architecture
- **AND** download and install Alpine Linux

#### Scenario: Install Arch Linux from UrlList.txt
- **WHEN** user selects to install archlinux
- **THEN** the system SHALL parse UrlList.txt
- **AND** extract the correct URL for the current architecture
- **AND** download and install Arch Linux

#### Scenario: Handle architecture detection
- **WHEN** installing a distribution
- **THEN** the system SHALL automatically detect the current architecture
- **AND** use the corresponding URL from UrlList.txt

#### Scenario: Fallback for missing architecture
- **WHEN** a distribution does not have URL for current architecture
- **THEN** the system SHALL display an error message
- **AND** suggest trying a different distribution

### Requirement: Real User and Group Detection
The system SHALL detect and use actual user and group information dynamically instead of using hardcoded values, including username, group memberships, and real-time timestamps.

#### Scenario: Detect current username dynamically
- **WHEN** starting an installation
- **THEN** the system SHALL query the actual username from the system
- **AND** use it in all file operations and configurations

#### Scenario: Detect user's primary group
- **WHEN** creating system directories
- **THEN** the system SHALL detect the user's actual primary group
- **AND** set appropriate ownership permissions

#### Scenario: Get real system time
- **WHEN** recording installation metadata
- **THEN** the system SHALL use the actual current system time
- **AND** format it appropriately for the metadata file

#### Scenario: Check actual disk space
- **WHEN** preparing for installation
- **THEN** the system SHALL check real available disk space
- **AND** warn if insufficient space is available

### Requirement: Progress Feedback with New Style
The system SHALL provide accurate, real-time progress feedback for download and extraction operations using the ASCII art progress bar style.

#### Scenario: Real download progress
- **WHEN** downloading a distribution file
- **THEN** the system SHALL track actual bytes downloaded
- **AND** calculate percentage based on total file size
- **AND** update the progress bar in real-time

#### Scenario: Real extraction progress
- **WHEN** extracting the downloaded archive
- **THEN** the system SHALL track files extracted or bytes processed
- **AND** update the progress bar based on actual progress

#### Scenario: Handle unknown file size
- **WHEN** downloading a file without content-length header
- **THEN** the system SHALL display indeterminate progress
- **AND** show bytes downloaded instead of percentage

#### Scenario: Progress bar completion
- **WHEN** an operation completes
- **THEN** the progress bar SHALL show 100%
- **AND** display a success message

### Requirement: Installation Completion Flow
The system SHALL prompt the user to start the installed system immediately after successful installation.

#### Scenario: Prompt user to start system
- **WHEN** installation completes successfully
- **THEN** the system SHALL display a prompt asking "是否立即启动系统？"
- **AND** provide Y/N options

#### Scenario: Start system immediately
- **WHEN** user confirms to start the system
- **THEN** the system SHALL execute the start script automatically
- **AND** log the user into the new system

#### Scenario: Decline immediate start
- **WHEN** user declines to start the system
- **THEN** the system SHALL display the manual start command
- **AND** return to the main menu

### Requirement: Distribution-Specific Initialization
The system SHALL execute distribution-specific init commands configured via {distro}-init entries during installation.

#### Scenario: Execute Ubuntu init commands
- **WHEN** installing Ubuntu with ubuntu-init configured
- **THEN** the system SHALL execute the init commands inside the chroot environment
- **AND** log the execution results

#### Scenario: Execute multi-line init script
- **WHEN** init commands contain multiple lines
- **THEN** the system SHALL execute them as a script within the installed system
- **AND** maintain proper execution order
