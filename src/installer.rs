use anyhow;
use anyhow::{bail, Error, Result};
use git2::build::RepoBuilder;
use git2::Repository;
use std::path::{Path, PathBuf};
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
    match &dependency.source {
        Some(s) => {
            if s.is_absolute() {
                bail!("Source directory may not be absolute.")
            }
        }
        None => (),
    }

    if dependency.destination.is_absolute() {
        bail!("Destination may not be absolute.")
    }

    let project_dir = std::env::current_dir()?;

    // Where we will clone the repo. This is either a temporary location, or the destination within the project.
    let install_dir: PathBuf = if dependency.has_source_remap() {
        TempDir::new()?.path().to_path_buf()
    } else {
        project_dir.join(&dependency.destination)
    };

    if install_dir.exists() {
        std::fs::remove_dir_all(&install_dir).expect("Failed to clear installation dir.");
    }

    let repo = Repository::clone(&dependency.url, &install_dir).expect("Failed to clone.");

    match &dependency.version {
        Some(version) => {
            let (object, reference) = repo.revparse_ext(&version).expect("Object not found");

            repo.checkout_tree(&object, None)
                .expect("Failed to checkout");

            match reference {
                // gref is an actual reference like branches or tags
                Some(gref) => repo.set_head(gref.name().unwrap()),
                // this is a commit, not a reference
                None => repo.set_head_detached(object.id()),
            }
            .expect("Failed to set HEAD");
        }
        None => (),
    }

    if dependency.has_source_remap() {
        let source_path = &dependency.source.as_ref().unwrap();
        let move_from = std::path::Path::join(&install_dir, &source_path);
        let move_to = std::path::Path::join(&project_dir, &dependency.destination);

        // Clear the output directory.
        if move_to.exists() {
            std::fs::remove_dir_all(&move_to)?;
        }

        println!(
            "Moving from {} to {}",
            move_from.display(),
            move_to.display()
        );
        std::fs::rename(move_from, move_to).expect("Failed to move from source to destination");
    } else {
        let remove_dir = std::path::Path::join(&install_dir, ".git");
        println!("Deleting .git dir: {}", remove_dir.display());
        std::fs::remove_dir_all(remove_dir)
            .expect("Failed to delete .git folder from installed dependency.");
    }

    return Ok(());
}
