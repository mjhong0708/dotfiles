use anyhow::Result;
use clap::{Parser, Subcommand};
use dotfiles::commands::{all_tools, check_and_install_tools, hook, install};
use dotfiles::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "dotfiles")]
#[command(about = "A simple dotfiles manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install dotfiles and configure shell
    Install {
        /// Do not install development tools
        #[arg(long, default_value_t = false)]
        no_tools: bool,
    },
    /// Generate shell configuration
    Hook {
        /// Target shell (zsh, bash)
        #[arg(long)]
        shell: String,
    },
    /// Manage development tools
    Tools {
        #[command(subcommand)]
        action: ToolsAction,
    },
}

#[derive(Subcommand)]
enum ToolsAction {
    /// List which tools are available and installed
    List,
    /// Install a specific tool
    Install,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { no_tools } => {
            install()?;
            if !no_tools {
                info!("Installing development tools...");
                check_and_install_tools()?;
            }
        }
        Commands::Hook { shell } => {
            hook(&shell)?;
        }
        Commands::Tools { action } => match action {
            ToolsAction::List => {
                let tools = all_tools();
                for tool in tools {
                    let status = if tool.is_installed() {
                        "Installed"
                    } else {
                        "Not Installed"
                    };
                    println!("{}: {}", tool.name(), status);
                }
            }
            ToolsAction::Install => {
                info!("Installing development tools...");
                check_and_install_tools()?;
            }
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
