use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spm")]
#[command(about = "Standalone Store Package Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a package into /store/ and active profile
    Install {
        name: String,
        version: String,

        /// Do not link binaries to active profile
        #[arg(short, long)]
        disabled: bool,
    },

    /// Unlink a package from active profile
    Remove {
        name: String,

        /// Also delete the package directory from /store/
        #[arg(short, long)]
        purge: bool,
    },

    /// List active and installed packages
    List,

    /// Remove unlinked/orphaned package directories from /store/
    Gc,

    /// Display system profile and store path information
    Info,
}