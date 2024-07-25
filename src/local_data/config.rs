use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Config {
    token: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            token: "your_token_here".to_string(),
        }
    }

    pub fn from_toml(path: &str) -> Self {
        let patht = Path::new(path);
        if patht.exists() {
            toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
        } else {
            let config = Config::new();
            config.save_to_toml(path);
            config
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn save_to_toml(&self, path: &str) {
        let path = Path::new(path);
        if !path.exists() {
            std::fs::File::create(path).unwrap();
        }
        std::fs::write(path, toml::to_string(&self).unwrap()).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
