## Context
The TermuxForLinux project has evolved from a simple installation script to a full-featured Linux distribution manager for Termux. The current flat file structure in `/src/` has become difficult to maintain as features have been added. The UI lacks modern visual feedback, and the configuration system needs more flexibility. This change addresses these technical debt issues while adding user-requested features like internationalization and custom shell support.

**Stakeholders**:
- End users: Termux users on Android devices
- Developers: Contributors to the TermuxForLinux project
- Maintainers: Project maintainers responsible for code review and releases

**Constraints**:
- Must maintain compatibility with Termux environment
- Must work on devices with limited resources
- Must support both interactive and CLI modes
- Must handle various Android versions and architectures

## Goals / Non-Goals

**Goals**:
- Improve code maintainability through modular architecture
- Enhance user experience with modern UI elements
- Add internationalization support for broader adoption
- Increase configuration flexibility for power users
- Optimize default settings for better first-run experience
- Implement proper system integration (user/permission detection)

**Non-Goals**:
- Adding new Linux distributions (beyond current support)
- Changing the core installation technology (proot)
- Adding network installation or cloud features
- Creating a GUI or web interface
- Supporting non-Termux environments

## Decisions

### Decision 1: Multi-Directory Module Structure
**What**: Restructure `/src/` from flat files to hierarchical modules: `/src/ui/`, `/src/installer/`, `/src/config/`, `/src/utils/`

**Why**: 
- Better code organization and separation of concerns
- Easier navigation and code discovery
- Reduced merge conflicts with parallel development
- Follows Rust best practices for larger projects

**Implementation**:
```
src/
├── main.rs              # Entry point only
├── cli/                 # CLI parsing and interactive menu
│   ├── mod.rs
│   ├── args.rs         # Command-line argument parsing
│   └── interactive.rs  # Interactive menu logic
├── ui/                  # User interface components
│   ├── mod.rs
│   ├── colors.rs       # Color theming
│   ├── progress.rs     # Progress bar implementation
│   └── display.rs      # Text formatting and layout
├── installer/           # Installation logic
│   ├── mod.rs
│   ├── download.rs     # Download management
│   ├── extract.rs      # Archive extraction
│   └── configure.rs    # System configuration
├── config/              # Configuration management
│   ├── mod.rs
│   ├── parser.rs       # Config file parsing
│   └── defaults.rs     # Default settings
├── distro/              # Distribution definitions
│   ├── mod.rs
│   ├── ubuntu.rs
│   ├── debian.rs
│   └── ...
├── system/              # System management
│   ├── mod.rs
│   ├── permissions.rs  # User/group detection
│   └── manage.rs       # Installation/uninstallation
└── utils/               # Shared utilities
    ├── mod.rs
    ├── fs.rs           # File system operations
    ├── cmd.rs          # Command execution
    └── net.rs          # Network utilities
```

**Trade-offs**: 
- Pro: Better organization, easier maintenance
- Con: More files to navigate, steeper learning curve for new contributors

### Decision 2: i18n System Design
**What**: Implement internationalization with English and Chinese support, extensible for more languages

**Why**:
- Broader user base and adoption
- Better accessibility for non-Chinese speakers
- Foundation for future language additions

**Implementation**:
```rust
// src/i18n/mod.rs
pub enum Language {
    Chinese,
    English,
}

pub struct I18n {
    lang: Language,
    strings: HashMap<String, String>,
}

impl I18n {
    pub fn t(&self, key: &str) -> &str {
        self.strings.get(key).unwrap_or(key)
    }
}

// Usage:
let i18n = I18n::new(Language::English);
println!("{}", i18n.t("welcome_message"));
```

**Language Files**: Store translations in TOML files within `src/i18n/locales/`

**Trade-offs**:
- Pro: Better user experience for international users
- Con: Increased maintenance burden for translations

### Decision 3: ANSI Color Theming System
**What**: Add color support with `--no-color` flag for accessibility

**Why**:
- Modern, visually appealing interface
- Better information hierarchy
- Accessibility compliance (users can disable colors)

**Implementation**:
```rust
// src/ui/colors.rs
pub struct Theme {
    pub success: String,
    pub error: String,
    pub warning: String,
    pub info: String,
    pub progress: String,
}

impl Theme {
    pub fn colorize(&self, text: &str, color: &str) -> String {
        if self.no_color {
            text.to_string()
        } else {
            format!("\x1b[{}m{}\x1b[0m", color, text)
        }
    }
}
```

**Color Palette**:
- Success: Green
- Error: Red  
- Warning: Yellow
- Info: Blue
- Progress: Cyan

**Trade-offs**:
- Pro: Better visual feedback, modern appearance
- Con: May not render correctly on all terminals

### Decision 4: Progress Bar Implementation
**What**: Replace current progress bar with ASCII art style showing percentages

**Why**:
- More intuitive visual feedback
- Better for small screens
- Clear indication of completion percentage

**Implementation**:
```rust
// src/ui/progress.rs
pub struct ProgressBar {
    width: usize,
    current: f32,
    total: f32,
}

impl ProgressBar {
    pub fn render(&self) -> String {
        let percentage = (self.current / self.total) * 100.0;
        let filled = ((percentage / 100.0) * self.width as f32) as usize;
        let empty = self.width - filled;
        
        format!(
            "{:>3.0}% [{}{}]",
            percentage,
            "=".repeat(filled),
            " ".repeat(empty)
        )
    }
}
```

**Display Format**:
```
0%   [                    ]
10%  [==                  ]
50%  [==========          ]
100% [==================]
```

**Trade-offs**:
- Pro: Clear visual progress, works on all terminals
- Con: Takes more horizontal space than minimal progress indicators

### Decision 5: Custom Shell Configuration
**What**: Allow users to define custom shell commands for login via config file

**Why**:
- Flexibility for power users
- Support for custom environments
- Better integration with user workflows

**Implementation**:
```toml
# Config format in $HOME/termos/config
shell = """
# Custom shell commands run at login
echo "Welcome to Termux Linux!"
export CUSTOM_VAR=value
exec bash
"""
```

```rust
// src/config/parser.rs
pub struct Config {
    pub shell: Option<String>,
    // ... other fields
}

impl Config {
    pub fn get_shell_commands(&self) -> Vec<String> {
        self.shell.as_ref()
            .map(|s| s.lines().filter(|l| !l.trim().is_empty()).map(String::from).collect())
            .unwrap_or_default()
    }
}
```

**Trade-offs**:
- Pro: High flexibility for advanced users
- Con: Potential security issues if config file is compromised

### Decision 6: User Group and Permission Detection
**What**: Implement real detection instead of hardcoded assumptions

**Why**:
- Better security and correctness
- Support for custom Termux installations
- Proper handling of permission errors

**Implementation**:
```rust
// src/system/permissions.rs
use std::process::Command;

pub fn get_current_user() -> Result<String, String> {
    Command::new("whoami")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .map_err(|e| e.to_string())
}

pub fn get_user_groups() -> Result<Vec<String>, String> {
    Command::new("groups")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .split_whitespace()
                .map(String::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

pub fn check_write_permission(path: &Path) -> bool {
    path.metadata()
        .map(|metadata| {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            (mode & 0o222) != 0  // Check write bit
        })
        .unwrap_or(false)
}
```

**Trade-offs**:
- Pro: More robust and secure
- Con: Additional system calls may slightly impact performance

### Decision 7: Renaming (insOs and termos)
**What**: Rename executable to `insOs` and directory to `$HOME/termos/`

**Why**:
- Shorter, more memorable name
- Better branding and identity
- Easier to type on mobile keyboards

**Implementation**:
- Update `Cargo.toml`: `name = "insOs"`
- Replace all string literals: `"Ostermux"` → `"termos"`
- Update documentation and help text
- Provide migration script for existing users

**Trade-offs**:
- Pro: Better user experience, stronger brand identity
- Con: Breaking change requiring user migration

## Risks / Trade-offs

### Risk 1: Breaking Changes for Existing Users
**Risk**: Users with existing installations will need to manually migrate
**Mitigation**: 
- Provide clear migration documentation
- Create migration script to move data from `$HOME/Ostermux/` to `$HOME/termos/`
- Maintain backward compatibility for config file format where possible

### Risk 2: Translation Maintenance Burden  
**Risk**: Translations may become outdated as features are added
**Mitigation**:
- Start with English and Chinese only
- Use simple translation keys that are easy to update
- Document translation process for contributors
- Consider automated translation validation in CI

### Risk 3: Terminal Compatibility
**Risk**: ANSI colors may not work on all terminals
**Mitigation**:
- Implement `--no-color` flag
- Auto-detect terminal capabilities
- Provide fallback to plain text
- Test on common Termux terminal emulators

### Risk 4: Performance Impact
**Risk**: Additional system calls for permission detection may slow down operations
**Mitigation**:
- Cache permission checks where appropriate
- Only detect user/group once per session
- Profile performance before and after changes
- Optimize critical paths

## Migration Plan

### For Existing Installations
1. **Manual Migration Required**: Users must move data from `$HOME/Ostermux/` to `$HOME/termos/`
2. **Config Migration**: Existing config files will be read from old location if new location doesn't exist
3. **Command Update**: Users need to update scripts from `termux-linux-install` to `insOs`

### Migration Script (Optional)
```bash
#!/bin/bash
# migrate-to-termos.sh
if [ -d "$HOME/Ostermux" ]; then
    echo "Migrating from Ostermux to termos..."
    mv "$HOME/Ostermux" "$HOME/termos"
    echo "Migration complete!"
else
    echo "No Ostermux directory found."
fi
```

### Rollback Plan
- Keep old executable name as symlink for one release cycle (if feasible)
- Document manual rollback steps
- Ensure config format is backward compatible

## Open Questions

1. **Translation Strategy**: Should we use a translation framework like `gettext` or keep it simple with HashMaps?
2. **Color Palette**: What specific ANSI color codes work best across all Termux terminals?
3. **Shell Configuration**: Should we support multiple shell profiles or just one global configuration?
4. **Performance**: Should we make permission detection optional for performance-critical operations?
5. **Backward Compatibility**: Should we maintain support for reading old config locations indefinitely?

## Decision Log

- **2025-11-21**: Initial design document created
- **Decisions to be made during implementation**:
  - Final color palette selection
  - Translation framework choice
  - Specific module boundaries and file organization
  - Performance optimization strategies
