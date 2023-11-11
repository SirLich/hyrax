#![allow(unused_braces)]

use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Read, Write};
use anyhow::{Result};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub foo: i64,

    // Ignore unknown fields
    #[serde(flatten)]
    unknown: serde_json::Map<String, serde_json::Value>,
}

pub fn read() -> Result<Root> {
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Root = serde_json::from_str(&contents)?;

    Ok(config)
}

pub fn write(config: &Root) -> Result<()> {
    let mut file = File::create("config.json")?;
    let json_str = serde_json::to_string_pretty(config)?;

    file.write_all(json_str.as_bytes())?;
    Ok(())
}