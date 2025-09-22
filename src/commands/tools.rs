use crate::{DotfilesError, Result, error, info, warning};
use std::process::Command;

pub trait ToolInstaller {
    fn name(&self) -> &str;
    fn is_installed(&self) -> bool {
        is_cmd_available(self.name())
    }
    fn install(&self) -> Result<()>;
}

pub struct Eza;
impl ToolInstaller for Eza {
    fn name(&self) -> &str {
        "eza"
    }

    fn install(&self) -> Result<()> {
        install_with_cargo(self.name())
    }
}

pub struct Ripgrep;
impl ToolInstaller for Ripgrep {
    fn name(&self) -> &str {
        "rg"
    }
    fn install(&self) -> Result<()> {
        install_with_cargo("ripgrep")
    }
}

pub struct Fd;
impl ToolInstaller for Fd {
    fn name(&self) -> &str {
        "fd"
    }
    fn install(&self) -> Result<()> {
        install_with_cargo("fd-find")
    }
}

pub struct Neovim;
impl ToolInstaller for Neovim {
    fn name(&self) -> &str {
        "nvim"
    }

    fn install(&self) -> Result<()> {
        let status = install_with_brew(self.name());
        match status {
            Err(_) => {
                warning!("Failed to install nvim with brew. Install it manually.");
                status
            }
            Ok(()) => Ok(()),
        }
    }
}

pub fn all_tools() -> Vec<Box<dyn ToolInstaller>> {
    vec![
        Box::new(Eza),
        Box::new(Ripgrep),
        Box::new(Fd),
        Box::new(Neovim),
        // Add more tools here as needed
    ]
}

pub fn check_and_install_tools() -> Result<()> {
    for tool in all_tools() {
        if tool.is_installed() {
            info!("{} is already installed.", tool.name());
        } else {
            info!("{} is not installed. Installing...", tool.name());
            tool.install()?;
        }
    }

    Ok(())
}

// Utils
fn is_cmd_available(tool: &str) -> bool {
    Command::new("which")
        .arg(tool)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_with_cargo(tool: &str) -> Result<()> {
    info!("Installing {} with cargo...", tool);

    if !is_cmd_available("cargo") {
        return Err(DotfilesError::Tool(
            "Cargo not found. Please install Rust first.".to_string(),
        ));
    }

    let output = Command::new("cargo").args(["install", tool]).output()?;

    if output.status.success() {
        info!("Successfully installed {}", tool);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Failed to install {}: {}", tool, stderr);
        return Err(DotfilesError::Tool(format!("Failed to install {}", tool)));
    }

    Ok(())
}

fn install_with_brew(tool: &str) -> Result<()> {
    info!("Installing {} with brew...", tool);
    if !is_cmd_available("brew") {
        return Err(DotfilesError::Tool(
            "Homebrew not found. Please install Homebrew first.".to_string(),
        ));
    }

    let output = Command::new("brew").args(["install", tool]).output()?;

    if output.status.success() {
        info!("Successfully installed {}", tool);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Failed to install {}: {}", tool, stderr);
        return Err(DotfilesError::Tool(format!("Failed to install {}", tool)));
    }

    Ok(())
}
