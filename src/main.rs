mod config;
mod cli;

use std::env;
use std::io::Write;
use std::path;

use anyhow::{Context, Result, bail};

use cli::Command;
use clap::Parser;

use std::path::Path;

use std::fs;
use std::fs::File;

use serde_json::json;

fn is_directory_empty(path: path::PathBuf) -> Result<bool> {
    let is_empty = path.read_dir().context("Failed to test whether the current directory is empty.")?.next().is_none();
    Ok(is_empty)
}

fn write_gitignore(force: bool) -> Result<()> {
    let path = Path::new(".gitignore");

    if force && path.exists() {
        println!("Initialization forced: deleting .gitignore");
        fs::remove_file(".gitignore")?;
    }

    let mut file = File::create(path)?;
    file.write("/build".as_bytes())?;
    file.write("/.regolith".as_bytes())?;
    
    Ok(())
}

fn write_config(params: cli::InitParams) -> Result<()> {
    let path: &Path = Path::new("config.json");

    if params.force && path.exists() {
        println!("Initialization forced: deleting config.json");
        fs::remove_file(path)?;
    }

    let mut data = json!({
        "author": "",
        "name": "",
        "packs": {
            "behaviorPack": "./project/BP",
            "resourcePack": "./project/RP"
        },
        "regolith": {
            "dataPath": "./project/data",
            "filterDefinitions": {},
            "profiles": {
                "default": {
                    "export": {
                        "readOnly": false,
                        "target": "development"
                    },
                    "filters": []
                }
            }
        }
    });
    
    data["author"] = serde_json::Value::String(params.author);
    data["name"] = serde_json::Value::String(params.name);

    let mut file = File::create(path)?;
    file.write(serde_json::to_string_pretty(&data).unwrap().as_bytes())?;

    Ok(())
}


fn initialize_project(params: cli::InitParams) -> Result<()> {
    let force = params.force;
    let current_dir: path::PathBuf = env::current_dir()?;

    println!("Initializing project in '{}' (force={})", current_dir.as_path().display(), force);


    if !force && !is_directory_empty(current_dir)? {
        bail!("Directory is not empty. Consider running with --force")
    }


    write_gitignore(force)?;
    write_config(params)?;

    fs::create_dir_all(".regolith/cache/venvs")?;
    fs::create_dir_all("project/BP")?;
    fs::create_dir_all("project/RP")?;
    fs::create_dir_all("project/data")?;


    Ok(())
}

fn main() {
    let cli = cli::Root::parse();

    match cli.command {
        Command::Init(params) => {
            initialize_project(params).expect("Could not initialize project.")
        },
        Command::Run { } => {
            println!("Regolith Run!")
        }
        Command::Test {} => {
            let mut config = config::read().expect("Could not read config");
            config.foo = 200;

            config::write(&config).expect("Could not write config.");
        }
    }
}
