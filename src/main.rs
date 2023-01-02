pub mod config;
pub mod webserver;

use std::{collections::HashSet, fs::read_dir, path::Path};

use actix_web::rt::System;
use config::Config;
use env_logger::Env;
use once_cell::sync::OnceCell;

/// Configuration path
pub const CONFIG_PATH: &str = "./data";
/// Configuration filename
pub const CONFIG_FILE: &str = "config.yaml";

/// Global configuration file
pub static CONFIG: OnceCell<Config> = OnceCell::new();

fn main() {
    load_logger();

    let config = load_config();

    if let Err(err) = check_dirs(&config) {
        log::error!("Failed to load all resource dirs: {err}");
        return;
    }

    if config.resources.directories.is_empty() {
        log::warn!("No resource directory configured!");
    }

    print_dir_info(&config);

    CONFIG.set(config).unwrap();

    System::new().block_on(async {
        webserver::start().await.unwrap();
    });
}

fn load_logger() {
    env_logger::init_from_env(Env::new().default_filter_or("info"));
}

/// Load the configuration file
fn load_config() -> Config {
    if let Ok(config_path) = std::env::var("CONFIG_PATH") {
        Config::load_file(&config_path).expect("Failed to load config")
    } else {
        Config::load(CONFIG_PATH, CONFIG_FILE).expect("Failed to load config")
    }
}

fn check_dirs(config: &Config) -> Result<(), String> {
    let dir_names: HashSet<&str> = HashSet::new();
    for dir in &config.resources.directories {
        let name = &dir.name;
        if dir_names.contains(name.as_str()) {
            return Err(format!("Directory {name} defined twice!"));
        }

        let path = Path::new(&dir.path);
        if !path.exists() {
            return Err(format!(
                "Path {:?} in dir {name:?} was configured but does not exist in filesystem!",
                dir.path,
            ));
        }
    }

    Ok(())
}

fn print_dir_info(config: &Config) {
    for dir in &config.resources.directories {
        let mut files = 0;
        let mut dirs = 0;
        for dir in read_dir(&dir.path).unwrap() {
            let dir = dir.unwrap();
            let metadata = dir.metadata().unwrap();

            if metadata.is_dir() {
                dirs += 1;
            } else if metadata.is_file() {
                files += 1;
            }
        }

        if files + dirs == 0 {
            log::warn!("Empty: {:?} located at {:?}", dir.name, dir.path);
        } else {
            log::debug!("Resources for {:?}", dir.name);
            log::debug!("\t{files} files.");
            log::debug!("\t{dirs} directories.");
        }
    }
}

#[inline]
pub fn config() -> &'static Config {
    CONFIG.get().unwrap()
}
