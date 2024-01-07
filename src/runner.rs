use crate::cli::GlobalOpts;
use crate::cli::RunParams;
use crate::config;
use crate::file_helper;

use anyhow::{Context, Result, bail};
use std::path::Path;

fn clean_export(profile: config::Profile) ->Result<()> {
	let path = Path::new(&profile.export.path);
	file_helper::clean_dir(path).context("Could not clean export directory.")
}


pub fn run(params: RunParams, opts: GlobalOpts) -> Result<()> {
	let config = config::read()?;
	let profile = config.get_profile(params.profile)?;

	clean_export(profile)?;

	println!("Hello World!");

	Ok(())
}