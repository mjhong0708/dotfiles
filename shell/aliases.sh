if command -v eza >/dev/null 2>&1; then
    alias l='eza --icons'
    alias ls='eza --icons'
    alias ll='eza -ghl --icons --git'
else
    alias l='ls'
    alias ll='ls -lh'
fi

if command -v lazygit >/dev/null 2>&1; then
    alias lg='lazygit'
fi

if command -v pnpm >/dev/null 2>&1; then
    alias pn='pnpm'
    alias px='pnpx'
fi


# Directories and files to exclude from rsync
RSYNC_EXCLUDES=(
    # Python
    '*.pyc'
    '*.pyo'
    '.venv'
    '__pycache__'
    # npm
    'node_modules'
    # Rust
    'target'
    # macOS
    '.DS_Store'
    # Git
    '.git'
)

rsync_exclude_args() {
    local args=()
    for exclude in "${RSYNC_EXCLUDES[@]}"; do
        args+=("--exclude='$exclude'")
    done
    echo "${args[@]}"
}
alias sync_dir="rsync -avh --delete --progress $(rsync_exclude_args)"
