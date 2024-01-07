mod config;
mod cli;
mod file_helper;
mod global; 
mod runner;
mod installer;

use std::env;
use std::io::Write;
use std::path;

use anyhow::{Result, bail};
use path_clean::{clean, PathClean};

use cli::Command;
use clap::Parser;

use std::path::{Path, PathBuf};

use std::fs;
use std::fs::File;

use serde_json::json;


fn write_gitignore(dir: &PathBuf, force: bool) -> Result<()> {
    let path: &Path = &dir.join(".gitignore");

    if force && path.exists() {
        println!("Initialization forced: deleting .gitignore");
        fs::remove_file(path)?;
    }

    let mut file = File::create(path)?;
    file.write("/build".as_bytes())?;
    file.write("/.regolith".as_bytes())?;
    
    Ok(())
}

fn write_config(dir: &PathBuf, params: cli::InitParams) -> Result<()> {
    let path: &Path = &dir.join("config.json");

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


fn initialize_project(params: cli::InitParams, opts: cli::GlobalOpts) -> Result<()> {
    let force = params.force;
    let working_dir: path::PathBuf = opts.working_dir()?;

    println!("Initializing project in '{}' (force={})", working_dir.as_path().display(), force);

    if !force && !file_helper::is_dir_empty(&working_dir)? {
        bail!("Directory is not empty. Consider running with --force")
    }

    write_gitignore(&working_dir, force)?;
    write_config(&working_dir, params)?;

    fs::create_dir_all(working_dir.join(".regolith/cache/venvs"))?;
    fs::create_dir_all(working_dir.join("project/BP"))?;
    fs::create_dir_all(working_dir.join("project/RP"))?;
    fs::create_dir_all(working_dir.join("project/data"))?;


    Ok(())
}

fn main() {
    let cli = cli::Root::parse();

    match cli.command {
        Command::Init(params) => {
            initialize_project(params, cli.global_opts).expect("Could not initialize project")
        },
        Command::Run(params) => {
            runner::run(params, cli.global_opts).expect("Could not run")
        },
        Command::Install(params) => {
            installer::install(params, cli.global_opts).expect("Could not install")
        },
        Command::Test {} => {
            println!("Hello World!");
        }
    }
}
