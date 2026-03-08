use anyhow::{self};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use git2::{Oid, Repository};
use inquire::Confirm;
use std::path::PathBuf;
use tempfile::{self, TempDir};

use crate::cli::{CheckParams, SyncParams};
use crate::{
    cli::AddParams,
    config::{load_config, save_config, HyraxDependency},
};

pub fn add(params: AddParams) -> Result<()> {
    let mut config = load_config().expect("Could not load config.");

    // TODO: Arguably if you need multiple deps from the same URL...
    for dependency in &config.dependencies {
        if dependency.url == params.url {
            bail!("This dependency already exists.");
        }
    }

    config.dependencies.push(params.into());

    save_config(&config).expect("Could not save config.");

    return Ok(());
}

pub fn check(params: &CheckParams) -> Result<()> {
    let config = load_config().expect("Could not load config.");

    for dependency in &config.dependencies {
        check_dependency(dependency, params)?;
    }
    Ok(())
}

fn get_commit_time(repository: &Repository, sha: &str) -> String {
    let oid = Oid::from_str(sha).unwrap();
    let commit = repository.find_commit(oid).unwrap();
    let time = commit.time();
    let timestamp = time.seconds();
    let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();

    datetime.to_rfc3339()
}

pub fn check_dependency(dependency: &HyraxDependency, _params: &CheckParams) -> Result<()> {
    let install_dir: PathBuf = TempDir::new()?.path().to_path_buf();
    let repository = install_repository(dependency, &install_dir)?;
    let current_sha = get_repo_sha(&repository)?;

    if let Some(version_lock) = &dependency.version_lock {
        if version_lock == &current_sha {
            println!(
                "Dependency {} is up to date (version matches version_lock)",
                dependency.name
            )
        } else {
            println!(
                "Dependency {} is stale.\n- Installed: {} (at {})\n- Available: {} (at {})",
                dependency.name,
                version_lock,
                get_commit_time(&repository, &version_lock),
                &current_sha,
                get_commit_time(&repository, &current_sha)
            )
        }
    } else {
        println!(
            "Dependency {} is likely not installed (no version_lock)",
            dependency.name
        )
    }

    Ok(())
}

pub fn sync(params: &SyncParams) -> Result<()> {
    let mut config = load_config().expect("Could not load config.");

    for dependency in &mut config.dependencies {
        sync_dependency(dependency, &params).expect("Failed to sync dependency")
    }

    save_config(&config).expect("Failed to save config");

    return Ok(());
}

pub fn sync_dependency(dependency: &mut HyraxDependency, params: &SyncParams) -> Result<()> {
    dependency.validate().expect("Dependency is invalid");

    let installed_version = if dependency.has_source_remap() {
        sync_dependency_with_source_remap(dependency, params).expect("Could not install dependency")
    } else {
        sync_dependency_full(dependency, params).expect("Could not install dependency")
    };

    dependency.version_lock.replace(installed_version);

    return Ok(());
}

/// Syncs a dependency directly into the users project.
/// Attempts to delete the .git folder of the synced dep.
pub fn sync_dependency_full(dependency: &HyraxDependency, params: &SyncParams) -> Result<String> {
    let project_dir = std::env::current_dir()?;
    let install_dir = project_dir.join(&dependency.destination);

    // Provider a user dialogue
    if params.force == false {
        println!("---");
        println!("Installing Dependency: {}", dependency.url);
        println!(
            " - Files will be installed directly into: {} (No",
            install_dir.display()
        );
        println!(
            " - This directory will be cleared before cloning: {}",
            install_dir.display()
        );

        let answer: bool = Confirm::new("Confirm?")
            .with_default(false)
            .prompt()
            .expect("Failed to parse user response.");

        if !answer {
            bail!("User rejected the operation.")
        }
    }

    if install_dir.exists() {
        std::fs::remove_dir_all(&install_dir).expect("Failed to clear installation dir.");
    }

    let repository =
        install_repository(&dependency, &install_dir).expect("Failed to install repository.");
    let installed_version = get_repo_sha(&repository)?;
    drop(repository); // So we can remove git dir.

    let remove_dir = std::path::Path::join(&install_dir, ".git");
    println!("Deleting .git dir: {}", remove_dir.display());
    std::fs::remove_dir_all(remove_dir)
        .expect("Failed to delete .git folder from installed dependency.");

    return Ok(installed_version);
}

pub fn get_repo_sha(repository: &Repository) -> Result<String> {
    return Ok(repository.head()?.peel_to_commit()?.id().to_string());
}
pub fn install_repository(
    dependency: &HyraxDependency,
    install_dir: &PathBuf,
) -> Result<Repository> {
    let repository = Repository::clone(&dependency.url, &install_dir).expect("Failed to clone.");

    // Switch to correct ref
    match &dependency.version {
        Some(version) => {
            let (object, reference) = repository.revparse_ext(&version).expect("Object not found");

            repository
                .checkout_tree(&object, None)
                .expect("Failed to checkout");

            match reference {
                // gref is an actual reference like branches or tags
                Some(gref) => repository.set_head(gref.name().unwrap()),
                // this is a commit, not a reference
                None => repository.set_head_detached(object.id()),
            }
            .expect("Failed to set HEAD");
        }
        None => (),
    }

    Ok(repository)
}

/// Syncs a dependency into a temp dir, and then copies it into the users project.
pub fn sync_dependency_with_source_remap(
    dependency: &HyraxDependency,
    params: &SyncParams,
) -> Result<String> {
    let project_dir = std::env::current_dir()?;

    let install_dir: PathBuf = TempDir::new()?.path().to_path_buf();
    let source_path = &dependency.source.as_ref().unwrap();
    let move_from = std::path::Path::join(&install_dir, &source_path);
    let move_to = std::path::Path::join(&project_dir, &dependency.destination);

    if params.force == false {
        println!("---");
        println!("Dependency: {}", dependency.url);
        println!("- Source: {}", move_from.display());
        println!("- Destination: {}", move_to.display());

        let answer: bool = Confirm::new("The entire destination folder is deleted. Confirm?")
            .with_default(false)
            .with_help_message("Hint: Destination folder should usually be empty.")
            .prompt()
            .expect("Failed to parse user response.");

        if !answer {
            bail!("User rejected the operation.")
        }
    }

    let repository =
        install_repository(&dependency, &install_dir).expect("Failed to install repository.");
    let installed_version = get_repo_sha(&repository)?;

    // Clear the output directory.
    if move_to.exists() {
        std::fs::remove_dir_all(&move_to)?;
    }

    std::fs::rename(move_from, move_to).expect("Failed to move from source to destination");

    return Ok(installed_version);
}
