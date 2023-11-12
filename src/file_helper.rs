use std::path::Path;
use walkdir::WalkDir;

use anyhow::{Context, Result, bail};

use std::path::PathBuf;

pub fn is_dir_empty(path: PathBuf) -> Result<bool> {
    let is_empty = path.read_dir().context("Failed to test whether the current directory is empty.")?.next().is_none();
    Ok(is_empty)
}

pub fn clean_dir(path: &Path) -> Result<()> {
	for entry in WalkDir::new(path) {
		let entry = entry.context("Failed to access a file")?;

		// Skip Directories
		if entry.path().is_dir() {
			continue;
		}

		println!("{}", entry.path().display());
	}

	Ok(())
}