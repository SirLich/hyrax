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

    /// Defines the working directory for this command.
    #[arg(short, long, default_value="")]
    pub dir: PathBuf,
}

impl InitParams {
    pub fn working_dir(&self) -> Result<PathBuf> {
        if self.dir.exists() {
            Ok(file_helper::absolute_path(&self.dir.clone())?)
        } else {
            Ok(env::current_dir()?)
        }
    }
}

#[derive(Debug, Args)]
pub struct RunParams {
    #[clap(default_value = "default")]
    pub profile: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initializes a new Regolite project
    Init(InitParams),

    /// Runs the specified profile
    Run(RunParams),

    /// Test
    Test {

    }
}

