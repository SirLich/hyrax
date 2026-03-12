use crate::cli::{CheckParams, SyncParams};
use crate::{
    cli::AddParams,
    config::{load_config, save_config, HyraxDependency},
};
use anyhow::{self};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use git2::{Direction, ObjectType, Oid, Repository};
use inquire::Confirm;
use log::info;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use tempfile::{self, TempDir};

#[derive(PartialEq)]
enum VersionType {
    SHA,
    Branch,
    Tag,
    Unknown,
}

pub fn add(params: AddParams) -> Result<()> {
    let mut config = load_config().expect("Could not load config.");

    for dependency in &config.dependencies {
        if dependency.name == params.name {
            bail!("This dependency already exists.");
        }
    }

    let mut dependency: HyraxDependency = params.into();
    if dependency.version.is_empty() {
        dependency.version =
            get_default_branch(&dependency.url).expect("Could not resolve default branch.");
    }
    config.dependencies.push(dependency);

    save_config(&config).expect("Could not save config.");

    return Ok(());
}

fn get_default_branch(repo_url: &str) -> Result<String, git2::Error> {
    let install_dir: PathBuf = TempDir::new().unwrap().path().to_path_buf();
    let repo = Repository::init_bare(install_dir)?;
    let mut remote = repo.remote_anonymous(repo_url)?;
    remote.connect(Direction::Fetch)?;
    let buf = remote.default_branch()?;
    let branch = buf.as_str().unwrap_or("").to_string();

    Ok(branch)
}

pub fn check(params: &CheckParams) -> Result<()> {
    let config = load_config().expect("Could not load config.");

    for dependency in config.dependencies {
        check_dependency(dependency, params);
    }
    Ok(())
}

fn is_tag(repository: &Repository, version: &str) -> bool {
    let refname = format!("refs/tags/{}", version);
    repository.find_reference(&refname).is_ok()
}

fn is_branch(repository: &Repository, version: &str) -> bool {
    let refname = format!("refs/heads/{}", version);
    repository.find_reference(&refname).is_ok()
}

fn is_commit_sha(repository: &Repository, version: &str) -> bool {
    if let Ok(oid) = Oid::from_str(version) {
        if let Ok(obj) = repository.find_object(oid, None) {
            return obj.kind() == Some(ObjectType::Commit);
        }
    }
    false
}

fn get_version_type(repository: &Repository, version: &str) -> VersionType {
    if is_tag(repository, &version) {
        return VersionType::Tag;
    } else if is_branch(repository, &version) {
        return VersionType::Branch;
    } else if is_commit_sha(repository, version) {
        return VersionType::SHA;
    }

    return VersionType::Unknown;
}

fn get_commit_time(repository: &Repository, sha: &str) -> String {
    let oid = Oid::from_str(sha).unwrap();
    let commit = repository.find_commit(oid).unwrap();
    let time = commit.time();
    let timestamp = time.seconds();
    let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();

    datetime.to_rfc3339()
}

pub fn check_dependency(dependency: HyraxDependency, _params: &CheckParams) {
    let install_dir: PathBuf = TempDir::new().unwrap().path().to_path_buf();
    let repository =
        install_repository(&dependency, &install_dir).expect("Could not install repository");

    if dependency.version_lock.is_none() {
        println!(
            "{} is likely not installed {}",
            dependency.name.yellow().bold(),
            "(no version_lock)".italic()
        );
        return;
    }

    let installed_version = dependency.version_lock.unwrap();
    let installed_type = get_version_type(&repository, &installed_version);

    let desired_version = dependency.version;
    let desired_type = get_version_type(&repository, &desired_version);

    if installed_type == VersionType::SHA {
        println!(
            "{} is likely not installed {}",
            dependency.name.yellow().bold(),
            "(no version_lock)".italic()
        );
        return;
    };

    let available_version =
        get_repo_sha(&repository).expect("Could not evaluate available version");

    if installed_version == available_version {
        println!(
            "{} is up to date {}",
            dependency.name.green().bold(),
            "(version matches version_lock)".italic()
        )
    } else {
        println!(
            "{} is stale.\n- {}: {} ({})\n- {}: {} ({})",
            dependency.name.red().bold(),
            "Installed".bold(),
            installed_version,
            get_commit_time(&repository, &installed_version).italic(),
            "Available".bold(),
            &available_version,
            get_commit_time(&repository, &available_version).italic()
        )
    }
}

pub fn sync(params: &SyncParams) -> Result<()> {
    let mut config = load_config().expect("Could not load config.");

    for dependency in &mut config.dependencies {
        sync_dependency(dependency, &params).expect("Failed to sync dependency")
    }

    save_config(&config).expect("Failed to save config");

    return Ok(());
}

fn get_user_confirmtation(
    dependency: &HyraxDependency,
    params: &SyncParams,
    install_dir: &PathBuf,
) -> bool {
    if params.force {
        return true;
    }
    println!(
        "- {} will be installed into {}",
        dependency.name.yellow(),
        install_dir.display().italic()
    );

    let answer: bool = Confirm::new("Confirm?")
        .with_default(false)
        .with_help_message("Warning: Destination folder should be empty.")
        .prompt()
        .expect("Failed to parse user response.");

    if !answer {
        return false;
    }

    return true;
}

pub fn sync_dependency(dependency: &mut HyraxDependency, params: &SyncParams) -> Result<()> {
    dependency.validate().expect("Dependency is invalid");

    if params.update {
        if dependency.version_lock.is_some() {
            println!("Updating {}...", dependency.name.green());
        } else {
            println!("Installing {}...", dependency.name.green());
        }
    } else {
        if dependency.version_lock.is_some() {
            println!(
                "Re-installing {} ({})",
                dependency.name.yellow(),
                "using version_lock".italic()
            );
        } else {
            println!("Installing {}...", dependency.name.green());
        }
    }
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

    let user_response = get_user_confirmtation(dependency, params, &install_dir);
    if !user_response {
        bail!("User rejected the confirmation.")
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
    info!("Installing repository: {}", dependency.url);
    let repository = Repository::clone(&dependency.url, &install_dir).expect("Failed to clone.");

    info!("Installing version: {}", dependency.version);

    {
        let (object, reference) = repository
            .revparse_ext(&dependency.version)
            .expect("Object not found");

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

    let user_response = get_user_confirmtation(dependency, params, &move_to);
    if !user_response {
        bail!("User rejected the confirmation.")
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
