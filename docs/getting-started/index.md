# Getting Started

## Installation

See the [Installation section in the README](https://github.com/sinelaw/fresh#installation) for all available installation methods, including Homebrew, AUR, .deb/.rpm packages, npm, crates.io, and building from source.





## Running Fresh

To run Fresh, you can either open it without a file, or specify a file to open:

```bash
# Open an empty buffer
fresh

# Open a file
fresh src/main.rs

# Open a file at a specific line
fresh src/main.rs:42

# Open a file at a specific line and column
fresh src/main.rs:42:10

# Open multiple files (with optional line:col)
fresh Cargo.toml src/lib.rs:100:5

# Open a remote file via SSH (experimental)
fresh user@host:/path/to/file.txt

# Open a remote directory via SSH
fresh user@host:~/projects
```

The `file:line:col` syntax is useful for jumping directly to compiler errors or search results.

### CLI Tools

Fresh includes command-line tools for package development:

```bash
# Create a new plugin, theme, or language pack project
fresh --init

# Validate a theme file (requires: pip install jsonschema)
./scripts/validate-theme.sh path/to/theme.json
```

## Core Concepts

*   **The Command Palette:** Press `Ctrl+P` to open the command palette - your central hub for navigating files, running commands, switching buffers, and jumping to lines. Use prefix characters to switch modes (see [Command Palette](../features/index.md#command-palette)).
*   **Buffers:** Each open file is represented as a buffer. You can have multiple buffers open at once and switch between them.
*   **Splits:** You can split your editor view horizontally or vertically to view multiple buffers at once.
*   **The Status Bar:** The status bar at the bottom of the screen displays information about the current buffer, including the file name, cursor position, and Git branch. Click on status messages to view the full message history.
