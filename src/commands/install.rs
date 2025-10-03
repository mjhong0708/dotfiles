use crate::shell::{detect_shell, get_shell_config_path, prepend_hook_to_config};
use crate::{Context, Result, create_symlink, env_var, info, warning};
use anyhow::bail;
use std::fs;
use std::path::{Path, PathBuf};

pub fn install() -> Result<()> {
    info!("Installing dotfiles...");

    let dotfiles_dir = get_dotfiles_dir()?;
    info!("Dotfiles directory: {}", dotfiles_dir.display());

    let shell = detect_shell()?;
    let config_path = get_shell_config_path(&shell)?;
    info!("Detected shell config: {}", config_path.display());

    prepend_hook_to_config(&config_path, &shell)?;
    install_root_configs(&dotfiles_dir)?;
    install_dot_config_dir(&dotfiles_dir)?;

    info!("Dotfiles installation complete!");
    info!("Please restart your shell or run: source {}", config_path.display());

    Ok(())
}

fn get_dotfiles_dir() -> Result<PathBuf> {
    let home = env_var("HOME")?;
    let dotfiles_dir = PathBuf::from(format!("{}/.config/dotfiles", home));
    if !dotfiles_dir.exists() {
        bail!(
            "Dotfiles directory not found at {}. Please clone the repository first.",
            dotfiles_dir.display()
        );
    }

    Ok(dotfiles_dir)
}

fn install_root_configs(dotfiles_dir: &Path) -> Result<()> {
    let user_home = env_var("HOME")?;
    let source_dir = dotfiles_dir.join("home");
    let target_dir = Path::new(&user_home);

    for entry in fs::read_dir(&source_dir).context("Failed to read home directory")? {
        match entry {
            Ok(entry) => {
                let source = entry.path();
                if source.file_name().map_or(false, |name| name == ".config") {
                    continue; // Skip .config directory
                }
                let target = target_dir.join(&entry.file_name());
                create_symlink(&source, &target)?;
            }
            Err(e) => {
                warning!("Failed to read {} in home directory: Skipping...", e);
            }
        }
    }

    Ok(())
}

fn install_dot_config_dir(dotfiles_dir: &Path) -> Result<()> {
    let user_home = env_var("HOME")?;
    let source_dir = dotfiles_dir.join("home").join(".config");
    let target_dir = Path::new(&user_home).join(".config");

    fs::create_dir_all(&target_dir).context("Failed to create .config directory")?;

    for entry in fs::read_dir(&source_dir).context("Failed to read config directory")? {
        match entry {
            Ok(entry) => {
                let source = entry.path();
                let target = target_dir.join(&entry.file_name());
                create_symlink(&source, &target)?;
            }
            Err(e) => {
                warning!("Failed to read {} in config directory: Skipping...", e);
            }
        }
    }

    Ok(())
}
