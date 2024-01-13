use std::path::PathBuf;
use std::env;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand, Args};

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
    #[arg(short, long, default_value=".", global=true)]
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
        Ok(self.working_dir()?.join(".regolith/cache"))
    }
}


#[derive(Debug, Args)]
pub struct InitParams {
    /// Forcefully initializes the project
    #[arg(short, long)]
    pub force: bool,

    /// Define the author of the project
    #[arg(short, long, default_value="Your name")]
    pub author: String,

    /// Define the name of the project
    #[arg(short, long, default_value="Project name")]
    pub name: String,
}


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

pub struct FilterIdentifier {
    /// The 'regolith' style URL, e.g, github.com/Bedrock-OSS/regolith-filters/name_ninja
    pub regolith_url: String,

    /// The URL, e.g., https://github.com/Bedrock-OSS/regolith-filters
    pub url: String,

    /// The identifier, e.g, name_ninja
    pub filter_name: String,

    /// The filepath where the filter.json can be found. e.g, C:/projects/project/.regolith/filters/name_ninja [/filter.json]
    pub file_path: PathBuf
}

impl FilterIdentifier {

}

enum FilterIdentifierType {
    ShortName,
    WebUrl,
    RegolithUrl
}


impl InstallParams {
    pub fn get_filter_identifier(&self, opts: &GlobalOpts) -> Result<FilterIdentifier>{
        match self.get_identifier_type() {
            FilterIdentifierType::ShortName => {
                bail!("Short-name URLs are not yet implemented.")
            },
            FilterIdentifierType::WebUrl => {
                bail!("Urls are not supported. Write it as a regolith-style url")
            },
            FilterIdentifierType::RegolithUrl => {
                let parts = self.filter.as_str().split("/").collect::<Vec<&str>>();

                match parts.len() {
                    4 => {
                        let name = parts[3].to_owned();

                        return Ok(FilterIdentifier {
                            regolith_url: self.filter.clone(),
                            url: "https://".to_owned() + parts[0] + "/" + parts[1] + "/" + parts[2],
                            filter_name: name.clone(),
                            file_path: opts.cache()?.join(name)
                        })
                    },
                    3 => {
                        bail!("Top level filters are not implemented")
                    },
                    _ => {
                        bail!("Is this URL malformed?")
                    } 
                }
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
    /// Initializes a new Regolite project
    Init(InitParams),

    /// Runs the specified profile
    Run(RunParams),

    /// Installs the specified filter
    Install(InstallParams),

    /// Test harness, for quickly running arbitrary code.
    Test {

    }
}

