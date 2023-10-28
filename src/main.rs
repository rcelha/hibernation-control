use clap::{Parser, Subcommand};

/// Manages Hibernation on Linux
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(author, version, about, long_about = None)]
enum Commands {
    /// Create swap file, update fstab and grub, and enable hibernation on systemd
    #[command()]
    Enable,

    /// This command DOES NOT remove the current swap file
    #[command()]
    Disable,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    match Cli::parse().command {
        Commands::Enable => {
            hibernation_control::commands::enable::run()?;
        }
        Commands::Disable => eyre::bail!("TODO"),
    };
    Ok(())
}
