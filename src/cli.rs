use clap::{Parser, Subcommand, Args};

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
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initializes a new Regolite project
    Init(InitParams),
    Run {

    },
    Test {

    }
}

