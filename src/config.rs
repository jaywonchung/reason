use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub state_path: PathBuf,
    pub history_path: PathBuf,
    pub max_history_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = match home::home_dir() {
            Some(mut p) => {
                p.push(".local/share/reason");
                p
            }
            None => {
                eprintln!("Failed to find your home directory. Using the current directory to save state and history.");
                PathBuf::from(".")
            }
        };
        let state_path = {
            let mut path = data_dir.clone();
            path.push("metadata.yaml");
            path
        };
        let history_path = {
            let mut path = data_dir.clone();
            path.push("history.txt");
            path
        };

        Self {
            state_path,
            history_path,
            max_history_size: 1000,
        }
    }
}
