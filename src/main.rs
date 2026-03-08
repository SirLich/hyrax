mod cli;
mod config;
mod installer;

use clap::Parser;
use cli::Command;

fn main() {
    let cli = cli::Root::parse();

    match cli.command {
        Command::Test {} => {}
        Command::Add(params) => {
            installer::add(params).unwrap();
        }
        Command::Sync(params) => {
            installer::sync(&params).unwrap();
        }
        Command::Check(params) => {
            installer::check(&params).unwrap();
        }
    }
}
