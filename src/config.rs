use fspp::*;
use serde::{Deserialize, Serialize};

use crate::repos::Repo;

const DEFAULT_CONFIG: &str = r#"{
    "repos": [
        {
            "url": "github:ALT-F4-LLC/kickstart.nix",
            "priority": 255,
        }
    ]
}
"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) repos: Vec<Repo>,
}

fn config_dir() -> Path {
    location::config().unwrap().add_str("kix")
}

fn config_file() -> Path {
    config_dir().add_str("config.json")
}

pub fn read_config() -> String {
    let config_dir: Path = config_dir();
    let config_file: Path = config_file();
    if !config_file.exists() {
        if !config_dir.exists() {
            directory::create(&config_dir).unwrap();
            file::write(DEFAULT_CONFIG, &config_file).unwrap();
        } else {
            file::write(DEFAULT_CONFIG, &config_file).unwrap();
        }
    }
    let contents = file::read(&config_file).unwrap();
    return contents;
}
