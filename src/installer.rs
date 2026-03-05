use anyhow;
use anyhow::{bail, Error, Result};
use git2::Repository;
use std::{env, fs};
use tempfile::{self, TempDir};

use crate::{
    cli::{AddParams, GlobalOpts, InstallParams, NewParams, UrlDescriptor},
    config::{load_config, save_config, HyraxDependency},
};

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

pub fn add(params: AddParams) -> Result<()> {
    let mut config = load_config().expect("Could not load config.");

    for dependency in &config.dependencies {
        if dependency.url == params.url {
            bail!("This dependency already exists.");
        }
    }

    config
        .dependencies
        .push(HyraxDependency::from_params(params));

    save_config(&config).expect("Could not save config.");

    return Ok(());
}

pub fn sync() -> Result<()> {
    let config = load_config().expect("Could not load config.");

    for dependency in &config.dependencies {
        sync_dependency(dependency).expect("Failed to sync dependency")
    }

    return Ok(());
}

pub fn sync_dependency(dependency: &HyraxDependency) -> Result<()> {
    let dir = TempDir::new()?.keep();

    println!("{}", dir.display());

    println!("{}", dependency.source.display());

    let move_from = std::path::Path::join(&dir, &dependency.source);
    let move_to = std::path::Path::join(&std::env::current_dir()?, &dependency.destination);

    println!("{}", move_from.display());
    println!("{}", move_to.display());

    let clone_result = Repository::clone(&dependency.url, dir);
    match clone_result {
        Ok(_repo) => {
            println!("Repo installed correctly.")
        }
        Err(e) => {
            println!("Repo could not be installed");
            println!("{}", e.to_string());
        }
    }

    std::fs::rename(move_from, move_to);

    return Ok(());
}
