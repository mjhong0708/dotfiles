BIN_DIRS=(
    "$HOME/.cargo/bin"
    "$HOME/go/bin"
    "$HOME/opt/bin"
    "$HOME/.local/bin"
)
LIB_DIRS=(
    "$HOME/opt/lib"
    "$HOME/.local/lib"

)

maybe_prepend_env_var() {
    local var_name="$1"
    local new_value="$2"
    local old_value="$(eval echo \$$var_name)"

    case ":$old_value:" in
        *:"$new_value":*)
            ;;
        *)
            export "$var_name=$new_value:$old_value"
            ;;
    esac
}

for dir in "${BIN_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        maybe_prepend_env_var "PATH" "$dir"
    fi
done

for dir in "${LIB_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        maybe_prepend_env_var "LD_LIBRARY_PATH" "$dir"
    fi
done

if command -v nvim >/dev/null 2>&1; then
    export EDITOR="nvim"
else
    export EDITOR="vim"
fi

export EXA_COLORS="uu=34:gu=34:un=37:gn=37"
export BAT_THEME="OneHalfDark"
