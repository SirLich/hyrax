use std::env;
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};

use crate::file_helper;

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
    /// Defines the working directory for this command. Can be relative, or absolute.
    #[arg(short, long, default_value = ".", global = true)]
    pub dir: PathBuf,
}

impl GlobalOpts {
    pub fn working_dir(&self) -> Result<PathBuf> {
        if self.dir.exists() {
            Ok(file_helper::absolute_path(&self.dir.clone())?)
        } else {
            Ok(env::current_dir()?)
        }
    }

    pub fn cache(&self) -> Result<PathBuf> {
        Ok(self.working_dir()?.join(".hyrax/cache"))
    }
}

#[derive(Debug, Args)]
pub struct InitParams {
    /// Forcefully initializes the project
    #[arg(short, long)]
    pub force: bool,

    /// Define the author of the project
    #[arg(short, long, default_value = "Your name")]
    pub author: String,

    /// Define the name of the project
    #[arg(short, long, default_value = "Project name")]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct NewParams {}

#[derive(Debug, Args)]
pub struct RunParams {
    #[clap(default_value = "default")]
    pub profile: String,
}

// #[derive(Debug, Args)]
// pub struct InstallParams {
//     /// Filter name or url
//     #[arg()]
//     pub filter: String,

//     /// Forcefully initializes the project
//     #[arg(short, long)]
//     pub force: bool,
// }

#[derive(Debug, Args)]
pub struct AddParams {
    #[arg()]
    pub url: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Test harness, for quickly running arbitrary code.
    Test {},

    /// Adds a dependency
    Add(AddParams),

    /// Sync
    Sync {},
}
