use clap::{Parser, Subcommand};
use std::{fs, io, env, path};
use anyhow::{Context, Result, bail};

/// A lite re-implementation of Regolith in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initializes a new Regolite project
    Init {
        /// Forcefully initializes the project
        #[arg(short, long)]
        force: bool
    },
    Run {

    },
}

fn is_directory_empty(path: path::PathBuf) -> Result<bool> {
    let is_empty = path.read_dir().context("Failed to test whether the current directory is empty.")?.next().is_none();
    Ok(is_empty)
}

fn initialize_project(force: bool) -> Result<()> {
    println!("Initializing project (force={})", force);

    let current_dir: path::PathBuf = env::current_dir()?;

    if !force && !is_directory_empty(current_dir)? {
        bail!("Directory is not empty. Consider running with --force")
    }

    fs::create_dir(".regolith")?;
    fs::create_dir("project")?;
    Ok(())
}

fn main() {
    let cli: Args = Args::parse();

    match cli.command {
        Command::Init { force } => {
            initialize_project(force).expect("Could not initialize project.")
        },
        Command::Run { } => {
            println!("Regolith Run!")
        }
    }
}
