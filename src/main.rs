mod cli;
mod config;
mod installer;

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

    match cli.command {
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
