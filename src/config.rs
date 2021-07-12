use serde::{Deserialize, Serialize};

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
