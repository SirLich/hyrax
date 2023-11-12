use std::path::Path;
use walkdir::WalkDir;
use std::fs;
use anyhow::{Context, Result, bail};
use crate::global;

use std::path::PathBuf;

pub fn is_dir_empty(path: PathBuf) -> Result<bool> {
    let is_empty = path.read_dir().context("Failed to test whether the current directory is empty.")?.next().is_none();
    Ok(is_empty)
}

pub fn clean_dir(path: &Path) -> Result<()> {
	for entry in WalkDir::new(path) {
		let entry = entry.context("Failed to access a file")?;
		let path = entry.path();

		// Skip Directories
		if path.is_dir() {
			continue;
		}
		
		let metadata = fs::metadata(path).expect("No metadata found");

		if metadata.modified()? != global::magic_time() {
			bail!("File has incorrect edit time!")
		}

		println!("{}", entry.path().display());
	}

	Ok(())
}