use std::path::PathBuf;
use std::env;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand, Args};

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
    #[arg(short, long, default_value="", global=true)]
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
}


#[derive(Debug, Args)]
pub struct InitParams {
    /// Forcefully initializes the project
    #[arg(short, long)]
    pub force: bool,

    /// Define the author of the project
    #[arg(short, long, default_value="Your name")]
    pub author: String,

    /// Define the name of the project
    #[arg(short, long, default_value="Project name")]
    pub name: String,
}


#[derive(Debug, Args)]
pub struct RunParams {
    #[clap(default_value = "default")]
    pub profile: String,
}

#[derive(Debug, Args)]
pub struct InstallParams {
    /// Forcefully initializes the project
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initializes a new Regolite project
    Init(InitParams),

    /// Runs the specified profile
    Run(RunParams),

    /// Installs the specified filter
    Install(InstallParams),

    /// Test harness, for quickly running arbitrary code.
    Test {

    }
}

