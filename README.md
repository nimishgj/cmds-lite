# cmds-lite

A repository containing simple Linux commands replicated in Rust to learn how they function under the hood. This project currently includes:

- `cmd-ls`: A simplified implementation of the Unix `ls` command

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70.0 or later)
- Cargo (included with Rust)

## Installation

### Clone the Repository

```bash
git clone https://github.com/yourusername/cmds-lite.git
cd cmds-lite
```

### Install the Commands

```bash
cargo install --path .
```

This will install the binaries to your Cargo bin directory (typically `~/.cargo/bin/`), which should be in your PATH.

## Commands

### cmd-ls

A simplified implementation of the Unix `ls` command that lists directory contents.

#### Usage

```bash
# List files in the current directory
cmd-ls

# List files in a specific directory
cmd-ls /path/to/directory

# Show hidden files
cmd-ls -a

# Show detailed listing with permissions, size, and timestamps
cmd-ls -l

# Combine options
cmd-ls -la /path/to/directory
```

#### Options

- `-a`: Show all files, including hidden ones (those starting with '.')
- `-l`: Use long listing format with permissions, size, and timestamps

## Development

### Adding a New Command

1. Create a new Rust file in the `src` directory (e.g., `src/grep.rs`)
2. Implement your command functionality
3. Add a binary entry in `Cargo.toml`:

```toml
[[bin]]
name = "cmd-grep"
path = "src/grep.rs"
```

4. Reinstall with `cargo install --path . --force`

