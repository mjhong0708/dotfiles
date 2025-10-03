use std::fs;
use std::path::{Path, PathBuf};
use crate::{Result, env_var, info};
use anyhow::bail;

#[derive(Debug, Clone)]
pub enum Shell {
    Zsh,
    Bash,
}

impl Shell {
    pub fn as_str(&self) -> &str {
        match self {
            Shell::Zsh => "zsh",
            Shell::Bash => "bash",
        }
    }
}

pub fn detect_shell() -> Result<Shell> {
    let shell_path = env_var("SHELL")?;

    if shell_path.contains("zsh") {
        Ok(Shell::Zsh)
    } else if shell_path.contains("bash") {
        Ok(Shell::Bash)
    } else {
        bail!("Unsupported shell: {}", shell_path)
    }
}

pub fn get_shell_config_path(shell: &Shell) -> Result<PathBuf> {
    let home = env_var("HOME")?;
    let home_path = Path::new(&home);

    let candidates = match shell {
        Shell::Zsh => vec![".zshrc"],
        Shell::Bash => vec![".bashrc", ".bash_profile", ".profile"],
    };

    // Check for existing files first
    for candidate in &candidates {
        let path = home_path.join(candidate);
        if path.exists() {
            return Ok(path);
        }
    }

    // Default to primary config file for the shell
    let default = match shell {
        Shell::Zsh => ".zshrc",
        Shell::Bash => ".bashrc",
    };

    Ok(home_path.join(default))
}

pub fn is_dotfiles_configured(config_path: &Path) -> Result<bool> {
    if !config_path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(config_path)?;
    Ok(content.contains("eval \"$(dotfiles hook"))
}

pub fn prepend_hook_to_config(config_path: &Path, shell: &Shell) -> Result<()> {
    let hook_line = format!("eval \"$(dotfiles hook --shell {})\"", shell.as_str());

    if is_dotfiles_configured(config_path)? {
        info!("Dotfiles hook already configured in {}", config_path.display());
        return Ok(());
    }

    let existing_content = if config_path.exists() {
        fs::read_to_string(config_path)?
    } else {
        String::new()
    };

    let new_content = if existing_content.is_empty() {
        format!("{}\n", hook_line)
    } else {
        format!("{}\n\n{}", hook_line, existing_content)
    };

    info!("Adding dotfiles hook to {}", config_path.display());
    fs::write(config_path, new_content)?;

    Ok(())
}