use std::path::PathBuf;

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub state_path: PathBuf,
    pub history_path: PathBuf,
    pub max_history_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        // Generate default paths.
        // Try to follow the XDG base directory specification, but fall back to the
        // current working directory if not possible.
        // These paths are not created yet.
        let data_dir = match ProjectDirs::from("rs", "reason", "reason") {
            Some(project_dir) => project_dir.data_dir().to_path_buf(),
            None => {
                eprintln!("User home directory not found. Storing state to the current working directory.");
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
