use crate::shell::{detect_shell, get_shell_config_path, prepend_hook_to_config};
use crate::{DotfilesError, Result, info, warning};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn install() -> Result<()> {
    info!("Installing dotfiles...");

    // Get dotfiles directory (fixed path)
    let home = env::var("HOME")
        .map_err(|_| DotfilesError::Shell("HOME environment variable not set".to_string()))?;
    let dotfiles_dir = PathBuf::from(format!("{}/.config/dotfiles", home));
    if !dotfiles_dir.exists() {
        return Err(DotfilesError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Dotfiles directory not found at {}. Please clone the repository first.",
                dotfiles_dir.display()
            ),
        )));
    }

    info!("Dotfiles directory: {}", dotfiles_dir.display());

    let shell = detect_shell()?;
    let config_path = get_shell_config_path(&shell)?;
    info!("Detected shell config: {}", config_path.display());
    prepend_hook_to_config(&config_path, &shell)?;
    create_config_symlinks(&dotfiles_dir)?;

    info!("Dotfiles installation complete!");
    info!(
        "Please restart your shell or run: source {}",
        config_path.display()
    );

    Ok(())
}

fn create_config_symlinks(dotfiles_dir: &Path) -> Result<()> {
    let config_source_dir = dotfiles_dir.join("home").join(".config");
    let home = env::var("HOME")
        .map_err(|_| DotfilesError::Shell("HOME environment variable not set".to_string()))?;
    let config_target_dir = Path::new(&home).join(".config");

    fs::create_dir_all(&config_target_dir)?;
    if !config_source_dir.exists() {
        warning!(
            "Config directory not found at {}",
            config_source_dir.display()
        );
        return Ok(());
    }

    for entry in fs::read_dir(&config_source_dir)? {
        let entry = entry?;
        let source = entry.path();
        let filename = entry.file_name();
        let target = config_target_dir.join(&filename);

        create_symlink(&source, &target)?;
    }

    Ok(())
}

fn create_symlink(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        if target.is_symlink() {
            let current_link = fs::read_link(target)?;
            if current_link == source {
                info!(
                    "Symlink {} already points to {}",
                    target.display(),
                    source.display()
                );
                return Ok(());
            } else {
                warning!("Removing existing symlink {}", target.display());
                if target.is_dir() {
                    fs::remove_dir_all(target)?;
                } else {
                    fs::remove_file(target)?;
                }
            }
        } else {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let backup = format!("{}.backup.{}", target.display(), timestamp);
            warning!("Backing up existing {} to {}", target.display(), backup);
            fs::rename(target, &backup)?;
        }
    }

    info!(
        "Creating symlink: {} -> {}",
        target.display(),
        source.display()
    );

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(source, target)?;
    }

    #[cfg(windows)]
    {
        if source.is_dir() {
            std::os::windows::fs::symlink_dir(source, target)?;
        } else {
            std::os::windows::fs::symlink_file(source, target)?;
        }
    }

    Ok(())
}
