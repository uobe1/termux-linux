## Why
The current codebase has grown organically and needs structural improvements for maintainability, user experience, and extensibility. The monolithic file structure makes it difficult to add new features, and the UI lacks modern visual feedback mechanisms. Additionally, the project needs better internationalization support and more flexible configuration options.

## What Changes
- **BREAKING**: Restructure `/src/` from flat to multi-directory architecture (`/src/ui/`, `/src/installer/`, etc.)
- **BREAKING**: Rename executable from `termux-linux-install` to `insOs`
- **BREAKING**: Change installation directory from `$HOME/Ostermux/` to `$HOME/termos/`
- Split large monolithic files into focused utility modules
- Redesign progress bar with modern ASCII art style (0%, 10%, 50%, 100% with visual bars)
- Add ANSI color support for enhanced UI with `--no-color` flag option
- Add English language interface support alongside Chinese
- Add custom shell command configuration for login via config file
- Implement real user group and permission detection
- Optimize default system settings for better out-of-box experience
- Remove unused code and consolidate utility functions

## Impact
- **Affected specs**: `cli`, `ui`, `installer`, `config`
- **Affected code**: All source files in `src/`, build configuration, installation scripts
- **Migration required**: Existing installations in `$HOME/Ostermux/` need manual migration
- **User impact**: Users will need to update their commands from `termux-linux-install` to `insOs`
