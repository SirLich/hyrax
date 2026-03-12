use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// A lite re-implementation of Regolith in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Root {
    #[command(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    /// Whether to enable verbose logging.
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(Debug, Args)]
pub struct NewParams {}

#[derive(Debug, Args)]
pub struct AddParams {
    /// Friendly name for the dependency. Just used for bookkeeping.
    #[arg()]
    pub name: String,

    /// The git URL of the dependency you wish to install.
    #[arg()]
    pub url: String,

    /// The path within the project where the dependency will be installed. Should usually point to a blank directory.
    #[arg()]
    pub destination: PathBuf,

    /// The path within the dependency that you want. If left blank, the entire dependency will be installed.
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// The version of the dependency you want to install. Can be a branch name, tag, or commit SHA.
    /// If left blank, the main branch will be used.
    #[arg(short, long)]
    pub version: Option<String>,
}

#[derive(Debug, Args)]
pub struct SyncParams {
    /// Updates dependencies to the latest version.
    #[arg(short, long)]
    pub update: bool,

    /// Skips dialogues
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct CheckParams {}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new dependency to the project.
    Add(AddParams),

    /// Downloads the listed dependencies into your project.
    Sync(SyncParams),

    /// Checks whether your dependencies are up to date.
    Check(CheckParams),
}
