use crate::{DotfilesError, Result};
use std::env;

pub fn hook(shell: &str) -> Result<()> {
    match shell {
        "zsh" | "bash" => {
            generate_shell_config(shell)?;
        }
        _ => {
            return Err(DotfilesError::Shell(format!(
                "Unsupported shell: {}",
                shell
            )));
        }
    }

    Ok(())
}

fn generate_shell_config(_shell: &str) -> Result<()> {
    let dotfiles_dir = env::var("DOTFILES_DIR").unwrap_or_else(|_| {
        env::var("HOME")
            .map(|home| format!("{}/.config/dotfiles", home))
            .unwrap_or_else(|_| "~/.config/dotfiles".to_string())
    });
    println!("export DOTFILES_DIR=\"{}\"", dotfiles_dir);
    println!("source \"$DOTFILES_DIR/shell/hook.sh\"");

    Ok(())
}
