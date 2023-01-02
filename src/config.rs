use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

/// Configuration for jotoba login server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub webserver: Webserver,
    pub resources: Resources,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            webserver: Webserver::default(),
            resources: Resources::default(),
        }
    }
}

/// Webserver configurations
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Webserver {
    pub bind_address: String,
}

impl Default for Webserver {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:8080".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resources {
    pub directories: Vec<Directory>,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            directories: vec![Directory {
                name: "dir1".to_string(),
                path: "/mnt/directory1".to_string(),
            }],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Directory {
    pub name: String,
    pub path: String,
}

impl Config {
    pub fn load_file(file: &str) -> Result<Config, Box<dyn Error>> {
        log::info!("Trying to load config file from {}", file);
        let read = BufReader::new(File::open(file)?);
        Ok(serde_yaml::from_reader(read)?)
    }

    pub fn load(file_path: &str, file_name: &str) -> Result<Config, Box<dyn Error>> {
        let path = Path::new(file_path).join(file_name);

        if !path.exists() {
            log::info!("Creating new config");

            let config = Config::default();

            if !Path::new(file_path).exists() {
                std::fs::create_dir_all(file_path)?;
            }

            let writer = BufWriter::new(File::create(path)?);
            serde_yaml::to_writer(writer, &config)?;
            return Ok(config);
        }

        log::info!("Loading config from {}", file_path);
        let read = BufReader::new(File::open(path)?);
        Ok(serde_yaml::from_reader(read)?)
    }
}
