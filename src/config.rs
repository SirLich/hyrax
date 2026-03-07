#![allow(unused_braces)]

use anyhow::{bail, Context, Result};
use git2::Config;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use toml::Table;

use crate::cli::AddParams;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyraxDependency {
    pub url: String,
    pub version: Option<String>,
    pub destination: PathBuf,
    pub source: Option<PathBuf>,
}

impl HyraxDependency {
    pub fn from_params(params: AddParams) -> HyraxDependency {
        return HyraxDependency {
            url: params.url,
            ..Default::default()
        };
    }

    /// If true, we only wish to install a portion of the repo
    pub fn has_source_remap(&self) -> bool {
        return self.source.is_some();
    }

    pub fn validate(&self) -> Result<()> {
        match &self.source {
            Some(s) => {
                if s.is_absolute() {
                    bail!("Source directory may not be absolute.")
                }
            }
            None => (),
        }

        if self.destination.is_absolute() {
            bail!("Destination may not be absolute.")
        }

        return Ok(());
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyraxConfig {
    pub dependencies: Vec<HyraxDependency>,
}

pub fn get_config_path() -> PathBuf {
    PathBuf::from("hyrax.toml")
}

pub fn load_config() -> Result<HyraxConfig> {
    let path = get_config_path();

    if !path.exists() {
        return Ok(HyraxConfig::default());
    }

    // let mut file = File::open(getConfigPath()).context("Could not open hyrax.toml file")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    let contents = read_to_string(path).expect("Could not read config file.");
    let config: HyraxConfig =
        toml::from_str(&contents).context("Could not parse contents of hyrax.toml")?;

    Ok(config)
}

pub fn save_config(config: &HyraxConfig) -> Result<()> {
    let path = get_config_path();

    let mut file = File::create(path)?;
    let json_str = toml::to_string_pretty(config)?;

    file.write_all(json_str.as_bytes())?;
    Ok(())
}
