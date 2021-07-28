use serde::{Deserialize, Serialize};

use crate::error::Fallacy;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub state_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            state_path: String::from("metadata.yaml"),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Fallacy> {
        match confy::load("reason") {
            Ok(c) => Ok(c),
            Err(e) => return Err(Fallacy::ConfigLoadFailed(e)),
        }
    }
}
