mod cli;
mod config;
mod installer;

use anyhow::Context;
use clap::Parser;
use cli::Command;
use log::{Level, LevelFilter, Metadata, Record};

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
static LOGGER: Logger = Logger;

fn main() {
    let cli = cli::Root::parse();

    let mut log_level = LevelFilter::Error;
    if cli.global_opts.debug {
        log_level = LevelFilter::Debug;
    }

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log_level))
        .expect("Failed to initialize logger.");

    if let Err(e) = run(cli) {
        println!("{}", e);

        for cause in e.chain().skip(1) {
            println!(" - {}", cause);
        }

        std::process::exit(1);
    }
}

pub fn run(cli: cli::Root) -> anyhow::Result<()> {
    match cli.command {
        Command::Add(params) => installer::add(params).context("Failed to add dependency"),
        Command::Sync(params) => installer::sync(&params).context("Failed to sync dependencies"),
        Command::Check(params) => installer::check(&params).context("Failed to check dependencies"),
    }?;

    Ok(())
}
