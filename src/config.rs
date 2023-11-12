#![allow(unused_braces)]

use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Read, Write};
use anyhow::{Result, bail, Context};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub author: String,
    pub name: String,
    pub regolith: Regolith,

    // Capture unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterDefinition {
    /// The version of the filter (for remote)
    version: String,

    /// The URL of the filter (for remote)
    url: String,

    /// The execution type for the filter (for local)
    run_with: String,

    /// The path of the script to execute (for local)
    script: String,

    /// Additional arguments which should be passed into the filter execution
    arguments: Vec<String>,

    /// Settings object, which will be passed to the filter as json
    settings: serde_json::Value,

    // Capture unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunFilter {
    /// The filter to run, sourced from filterDefinitions.json
    filter: String, 

    /// Settings object, which will be passed to the filter as json
    settings: serde_json::Value,

    /// Whether the filter is disabled
    #[serde(default)]
    disabled: bool, // Default false

    // Capture unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportData {
    /// The export type (e.g., exact)
    target: String,

    /// The export location, used with 'exact'
    path: String

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {

    /// The list of filters to execute, when this profile is run
    filters: Vec<RunFilter>,

    /// The export data, which defines where this profile will be exported
    export: ExportData,

    // Capture unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regolith {
    pub data_path: String,
    pub filter_definitions: HashMap<String, FilterDefinition>,
    pub profiles: HashMap<String, Profile>,

    // Ignore unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

pub fn read() -> Result<Config> {
    let mut file = File::open("config.json").context("Could not open config.json file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents).context("Could not parse contents of config.json")?;

    Ok(config)
}

pub fn write(config: &Config) -> Result<()> {
    let mut file = File::create("config.json")?;
    let json_str = serde_json::to_string_pretty(config)?;

    file.write_all(json_str.as_bytes())?;
    Ok(())
}

impl Config {

    /// Fetches a profile by name, or error if not exists.
    pub fn get_profile(&self, profile_name : String) -> Result<Profile> {
        match self.regolith.profiles.get(&profile_name) {
            Some(p) => Ok(p.clone()),
            None => bail!("Profile '{}' does not exist", profile_name)
        }
    }
}