use crate::cli::RunParams;
use crate::config;
use anyhow::{Context, Result, bail};

pub fn run(params: RunParams) -> Result<()> {
	let config = config::read()?;

	let _profile = config.get_profile(params.profile)?;

	println!("Hello World!");
	
	Ok(())
}