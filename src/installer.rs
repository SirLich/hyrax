use anyhow::Result;
use git2::Repository;
use std::fs;

use crate::cli::{GlobalOpts, InstallParams, NewParams, UrlDescriptor};

/// Returns whether a source currently exists or not
pub fn does_source_exist(identifier: &UrlDescriptor) -> bool {
    return identifier.file_path.exists();
}

pub fn download_repo(params: InstallParams, opts: GlobalOpts) -> Result<Repository> {
    let working_dir = opts.working_dir()?;

    let identifier: UrlDescriptor = params.get_filter_identifier(&opts)?;
    let url = &identifier.url;
    let install_location = working_dir
        .join(".hyrax/cache/sources/")
        .join(&identifier.author_name);

    fs::create_dir_all(&install_location)?;

    println!(
        "Attempting to install filter '{}' in '{}'",
        url,
        install_location.display()
    );
    match Repository::clone(url, install_location) {
        Ok(repo) => Ok(repo),
        Err(e) => panic!("failed to clone: {}", e),
    }
}

pub fn install(params: InstallParams, opts: GlobalOpts) -> Result<()> {
    println!("Installing filter '{}'", params.filter);

    download_repo(params, opts)?;
    Ok(())
}
