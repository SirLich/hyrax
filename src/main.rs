use clap::{Parser, Subcommand};
use std::{env, path};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub foo: i64,

    // Ignore unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

fn read_config() -> Result<Root> {
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Root = serde_json::from_str(&contents)?;

    Ok(config)
}

fn write_config(config: &Root) -> Result<()> {
    let mut file = File::create("config.json")?;
    let json_str = serde_json::to_string_pretty(config)?;

    file.write_all(json_str.as_bytes())?;
    Ok(())
}


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
    Test {

    }
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
        Command::Test {} => {
            let mut config = read_config().expect("Could not read config");
            config.foo = 200;

            write_config(&config).expect("Could not write config.");
        }
    }
}
