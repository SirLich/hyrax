#![allow(unused_braces)]

use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Read, Write};
use anyhow::{Result};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub author: String,
    pub name: String,

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

    // Capture unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {

    // The list of filters to execute, when this profile is run
    filters: Vec<RunFilter>,

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
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}

pub fn write(config: &Config) -> Result<()> {
    let mut file = File::create("config.json")?;
    let json_str = serde_json::to_string_pretty(config)?;

    file.write_all(json_str.as_bytes())?;
    Ok(())
}