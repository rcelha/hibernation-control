use clap::{Parser, Subcommand};
use hibernation_control::commands::enable::{Options, SwapOptions};

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
    Enable {
        /// Size of the swap file in MiB
        #[arg(long)]
        swapfile_size: Option<u64>,

        /// Path to the swap file
        #[arg(short, long)]
        swapfile_path: Option<String>,
    },

    /// This command DOES NOT remove the current swap file
    #[command()]
    Disable,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    match Cli::parse().command {
        Commands::Enable {
            swapfile_size,
            swapfile_path,
        } => {
            let options = Options {
                swap: SwapOptions {
                    size: swapfile_size,
                    path: swapfile_path,
                },
            };
            hibernation_control::commands::enable::run(options)?;
        }
        Commands::Disable => eyre::bail!("TODO"),
    };
    Ok(())
}
