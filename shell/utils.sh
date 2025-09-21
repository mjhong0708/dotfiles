#!/bin/sh

cprint() {
    local color="$1"
    shift
    case "$color" in
        red) echo "\033[0;31m$*\033[0m" ;;
        green) echo "\033[0;32m$*\033[0m" ;;
        yellow) echo "\033[0;33m$*\033[0m" ;;
        blue) echo "\033[0;34m$*\033[0m" ;;
        magenta) echo "\033[0;35m$*\033[0m" ;;
        cyan) echo "\033[0;36m$*\033[0m" ;;
        *) echo "$*" ;;  # Default to no color
    esac
}

printlog() {
    local level="$1"
    shift
    case "$level" in
        info) cprint green "[INFO] $*" ;;
        warn) cprint yellow "[WARN] $*" ;;
        error) cprint red "[ERROR] $*" ;;
        *) echo "$*" ;;  # Default to no log level
    esac
}

detect_os() {
    os_type="$(uname)"

    if [ "$os_type" = "Darwin" ]; then
        echo "macos"
    elif [ "$os_type" = "Linux" ]; then
        echo "linux"
    else
        echo "other"
    fi
}


# Prepend a given value to an environment variable if it is not already present.
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

