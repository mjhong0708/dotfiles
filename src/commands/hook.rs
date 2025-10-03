use crate::{Result, env_var};
use anyhow::bail;
use std::env;

pub fn hook(shell: &str) -> Result<()> {
    match shell {
        "zsh" | "bash" => generate_shell_config(shell),
        _ => bail!("Unsupported shell: {}", shell),
    }
}

fn generate_shell_config(_shell: &str) -> Result<()> {
    let dotfiles_dir = env::var("DOTFILES_DIR").unwrap_or_else(|_| {
        env_var("HOME")
            .map(|home| format!("{}/.config/dotfiles", home))
            .unwrap_or_else(|_| "~/.config/dotfiles".to_string())
    });

    println!("export DOTFILES_DIR=\"{}\"", dotfiles_dir);
    println!("source \"$DOTFILES_DIR/shell/hook.sh\"");

    Ok(())
}
