use std::path::Path;
use walkdir::WalkDir;

use anyhow::{Context, Result, bail};

use std::path::PathBuf;

pub fn is_dir_empty(path: PathBuf) -> Result<bool> {
    let is_empty = path.read_dir().context("Failed to test whether the current directory is empty.")?.next().is_none();
    Ok(is_empty)
}

pub fn clean_dir(path: &Path) {
	for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
		println!("{}", entry.path().display());
	}
}