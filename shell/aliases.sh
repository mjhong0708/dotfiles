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
