# Dotfiles

A simple Rust-based dotfiles manager that uses the eval hook pattern for clean shell configuration.

## Installation

```shell
git clone https://github.com/mjhong/.config/dotfiles ~/.config/dotfiles
cd ~/.config/dotfiles
cargo install --path .
dotfiles install
```

The install command will:
- Auto-detect your shell (zsh/bash) and prepend eval hook to your shell config
- Create symlinks for all configurations in .config/ directory
- Safely handle existing configurations (with backups if needed)
- Can be run multiple times safely

After installation, your shell config will contain a single line:
```shell
eval $(dotfiles hook --shell zsh)
```

## Usage

- `dotfiles install` - One-time setup and configuration
- `dotfiles hook --shell <shell>` - Generate shell configuration (used by eval hook)
- `dotfiles tools check` - Check which development tools are installed
- `dotfiles tools install <tool>` - Install development tools

## Benefits

- **Clean shell config** - Only one eval line instead of multiple source commands
- **Dynamic configuration** - Shell config is generated based on available tools
- **Easy updates** - Update the binary without touching shell config
- **Tool management** - Built-in checking and installation of development tools
- **Cross-platform** - Works on macOS and Linux

