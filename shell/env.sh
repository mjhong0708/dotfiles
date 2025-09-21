#!/bin/sh
# Load utils
. "$DOTFILES_DIR/shell/utils.sh"

# Set custom PATH
BIN_DIRS=(
    "$HOME/opt/bin"
    "$HOME/.local/bin"
    "$HOME/.cargo/bin"
    "$HOME/go/bin"
)
for dir in "${BIN_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        maybe_prepend_env_var "PATH" "$dir"
    fi
done

# Set custom LD_LIBRARY_PATH
LIB_DIRS=(
    "$HOME/opt/lib"
    "$HOME/.local/lib"

)
for dir in "${LIB_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        maybe_prepend_env_var "LD_LIBRARY_PATH" "$dir"
    fi
done


export EXA_COLORS="uu=34:gu=34:un=37:gn=37"
export EDITOR="nvim"
