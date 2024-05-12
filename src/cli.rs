use std::env;
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};

use crate::file_helper;

/// A lite re-implementation of Regolith in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Root {
    #[command(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    /// Defines the working directory for this command. Can be relative, or absolute.
    #[arg(short, long, default_value = ".", global = true)]
    pub dir: PathBuf,
}

impl GlobalOpts {
    pub fn working_dir(&self) -> Result<PathBuf> {
        if self.dir.exists() {
            Ok(file_helper::absolute_path(&self.dir.clone())?)
        } else {
            Ok(env::current_dir()?)
        }
    }

    pub fn cache(&self) -> Result<PathBuf> {
        Ok(self.working_dir()?.join(".hyrax/cache"))
    }
}

#[derive(Debug, Args)]
pub struct InitParams {
    /// Forcefully initializes the project
    #[arg(short, long)]
    pub force: bool,

    /// Define the author of the project
    #[arg(short, long, default_value = "Your name")]
    pub author: String,

    /// Define the name of the project
    #[arg(short, long, default_value = "Project name")]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct NewParams {}

#[derive(Debug, Args)]
pub struct RunParams {
    #[clap(default_value = "default")]
    pub profile: String,
}

#[derive(Debug, Args)]
pub struct InstallParams {
    /// Filter name or url
    #[arg()]
    pub filter: String,

    /// Forcefully initializes the project
    #[arg(short, long)]
    pub force: bool,
}

pub struct UrlDescriptor {
    /// The URL to the repository root, e.g., https://github.com/Bedrock-OSS/regolith-filters
    pub url: String,

    /// The author of the repo (e.g., SirLich)
    pub author_name: String,

    /// The name of the repo (e.g, funtils)
    pub repo_name: String,

    /// The filepath where the filter.json can be found. e.g, C:/projects/project/.hyrax/filters/sirlich/functils
    pub file_path: PathBuf,
}

impl UrlDescriptor {}

enum FilterIdentifierType {
    ShortName,
    WebUrl,
    RegolithUrl,
}

impl InstallParams {
    pub fn get_filter_identifier(&self, opts: &GlobalOpts) -> Result<UrlDescriptor> {
        match self.get_identifier_type() {
            FilterIdentifierType::ShortName => {
                bail!("Short-name URLs are not yet implemented.")
            }
            FilterIdentifierType::WebUrl => {
                let prefix = "https://github.com/";
                let parts = self
                    .filter
                    .as_str()
                    .strip_prefix(prefix)
                    .expect("Should be a github URL.")
                    .split("/")
                    .collect::<Vec<&str>>();

                println!("{:?}", parts);

                let name = parts[3].to_owned();

                return Ok(UrlDescriptor {
                    url: "https://".to_owned() + parts[0] + "/" + parts[1] + "/" + parts[2],
                    author_name: name.clone(),
                    repo_name: name.clone(),
                    file_path: opts.cache()?.join(name),
                });
            }
            FilterIdentifierType::RegolithUrl => {
                bail!("Regolith URLs are not supported.")
            }
        }
    }

    fn get_identifier_type(&self) -> FilterIdentifierType {
        if self.filter.contains(":") {
            return FilterIdentifierType::WebUrl;
        }

        if self.filter.contains("/") {
            return FilterIdentifierType::RegolithUrl;
        }

        return FilterIdentifierType::ShortName;
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initializes a new Hyrax project. Intended to be run in a configured godot project.
    Init(InitParams),

    /// Runs the specified profile
    Run(RunParams),

    /// Installs the specified filter
    Install(InstallParams),

    /// Creates a new project based on the template
    New(NewParams),

    /// Test harness, for quickly running arbitrary code.
    Test {},
}
