use anyhow::Result;

use crate::cli::{InstallParams, GlobalOpts};

pub fn install(params: InstallParams, opts: GlobalOpts) -> Result<()> {
	println!("Hello World!");
	Ok(())
}