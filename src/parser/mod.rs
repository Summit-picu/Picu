use clap::{Parser, Subcommand};

pub mod parserout;

#[derive(Parser)]
#[command(
    version,
    about = "Source-based package manager made on rust programing language, for summit linux"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// install packages
    Add {
        packages: Vec<String>,
        #[arg(short, long)]
        yes: bool,
        #[arg(short = 'u', long)]
        allow_unsafe: bool,
        #[arg(short, long)]
        local_package: bool,
    },
    /// build packages from source
    Cook {
        packages: Vec<String>,
        #[arg(long)]
        clean_sources: bool,
        #[arg(short, long)]
        clean: bool,
        #[arg(long)]
        no_install: bool,
    },
    /// delete package
    Cut(Vec<String>),
    /// find packages
    Find(String),
    /// sync mirrors
    Sync,
    /// remove old mirrors and install new
    Rebase,
    /// list packages
    List {
        #[arg(short, long)]
        show_uninstalled: bool,
    },
    /// show info about package or mirror
    Info {
        // name on <Package> or <Mirror> (if you wana get info aboout mirror set `-m` flag or you can get error)
        name: String,
        // info about mirror REQUERED FLAG TO GET INFO ABOUT MIRROR
        #[arg(short, long)]
        mirror: bool,
    },
    /// update old packages if updates avaible
    Update {
        /// sync mirrors before update
        #[arg(short, long)]
        sync_mirrors: bool,
    },
    /// update trust of mirrors
    Trust(TrustCommands),
}

#[derive(Subcommand)]
pub enum TrustCommands {
    /// list mirrors and they trust level
    List,
    /// add trust level to mirror
    Add {
        /// name of mirror
        name: String,
        #[arg(long)]
        user_local_setting: bool,
    },
    /// remove mirror from trust level
    Revoke(String),
}

pub fn parser() -> Commands {
    Cli::parse().command
}
