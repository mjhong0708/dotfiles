if command -v eza >/dev/null 2>&1; then
    alias l='eza --icons'
    alias ls='eza --icons'
    alias ll='eza -ghl --icons --git'
else
    alias l='ls'
    alias ll='ls -lh'
fi

if command -v lazygit >/dev/null 2>&1; then
    alias lg='LANG=en_US.UTF-8 lazygit'
fi

if command -v pnpm >/dev/null 2>&1; then
    alias pn='pnpm'
    alias px='pnpx'
fi