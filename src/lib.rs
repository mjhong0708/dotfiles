pub mod commands;
pub mod shell;
pub use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::Path;

/// Helper to get environment variable with context
pub fn env_var(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("{} environment variable not set", name))
}

fn create_symlink(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        if target.is_symlink() {
            let current_link = fs::read_link(target)?;
            if current_link == source {
                info!("Symlink {} already points to {}", target.display(), source.display());
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

    info!("Creating symlink: {} -> {}", target.display(), source.display());

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

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("\x1b[32m[INFO]\x1b[0m {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        println!("\x1b[33m[WARN]\x1b[0m {}", format!($($arg)*));
    };
}
