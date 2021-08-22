use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Fallacy;
use crate::utils::{expand_tilde, expand_tilde_str};

pub static MAN: &str = include_str!("../man/config.md");

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub storage: StorageConfig,
    pub filter: FilterConfig,
    pub output: OutputConfig,
}

#[derive(Serialize, Deserialize)]
pub struct StorageConfig {
    pub paper_metadata: PathBuf,
    pub command_history: PathBuf,
    pub max_history_size: usize,
    pub file_dir: PathBuf,
    pub note_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct FilterConfig {
    pub case_insensitive_regex: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OutputConfig {
    pub table_columns: Vec<String>,
    pub viewer_command: Vec<String>,
    pub viewer_batch: bool,
    pub editor_command: Vec<String>,
    pub editor_batch: bool,
    pub browser_command: Vec<String>,
}

impl Config {
    pub fn validate(&mut self) -> Result<(), Fallacy> {
        self.storage.validate()?;
        self.filter.validate()?;
        self.output.validate()?;
        Ok(())
    }
}

impl StorageConfig {
    fn validate(&mut self) -> Result<(), Fallacy> {
        self.paper_metadata = expand_tilde(&self.paper_metadata)?;
        self.command_history = expand_tilde(&self.command_history)?;
        self.file_dir = expand_tilde(&self.file_dir)?;
        std::fs::create_dir_all(&self.file_dir)?;
        self.note_dir = expand_tilde(&self.note_dir)?;
        std::fs::create_dir_all(&self.note_dir)?;
        Ok(())
    }
}

impl FilterConfig {
    fn validate(&mut self) -> Result<(), Fallacy> {
        Ok(())
    }
}

impl OutputConfig {
    fn validate(&mut self) -> Result<(), Fallacy> {
        let allowed_columns = vec!["title", "authors", "first author", "venue", "year", "state"];

        // Convert everything to lowercase.
        for field in &mut self.table_columns {
            *field = field.to_lowercase();
        }

        // Check table columns.
        for col in self.table_columns.iter() {
            if !allowed_columns.contains(&&col[..]) {
                return Err(Fallacy::ConfigAuditError(format!(
                    "Table column name {} is not supported.",
                    col
                )));
            }
        }

        // Check viewer command and expand tilde.
        if self.viewer_command.is_empty() {
            return Err(Fallacy::ConfigAuditError(
                "Viewer command cannot be empty.".to_owned(),
            ));
        }
        for path in self.viewer_command.iter_mut() {
            *path = expand_tilde_str(path)?;
        }

        // Check editor command and expand tilde.
        if self.editor_command.is_empty() {
            return Err(Fallacy::ConfigAuditError(
                "Editor command cannot be empty.".to_owned(),
            ));
        }
        for path in self.editor_command.iter_mut() {
            *path = expand_tilde_str(path)?;
        }

        // Check browser command and expand tilde.
        if self.browser_command.is_empty() {
            return Err(Fallacy::ConfigAuditError(
                "Browser command cannot be emtpy.".to_owned(),
            ));
        }
        for path in self.browser_command.iter_mut() {
            *path = expand_tilde_str(path)?;
        }

        Ok(())
    }
}

impl Default for StorageConfig {
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
        let paper_metadata = {
            let mut path = data_dir.clone();
            path.push("metadata.yaml");
            path
        };
        let command_history = {
            let mut path = data_dir.clone();
            path.push("history.txt");
            path
        };

        let max_history_size = 1000;

        let file_base_dir = {
            let mut path = data_dir.clone();
            path.push("files");
            path
        };

        let note_dir = {
            let mut path = data_dir;
            path.push("notes");
            path
        };

        Self {
            paper_metadata,
            command_history,
            max_history_size,
            file_dir: file_base_dir,
            note_dir,
        }
    }
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            case_insensitive_regex: false,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        let table_columns = vec!["title", "first author", "venue", "year"];
        let table_columns = table_columns.into_iter().map(|s| s.to_string()).collect();
        let viewer_command = vec![String::from("zathura")];
        let viewer_batch = false;
        let editor_command = vec![String::from("vim"), String::from("-p")];
        let editor_batch = true;
        let browser_command = vec![String::from("google-chrome-stable")];

        Self {
            table_columns,
            viewer_command,
            viewer_batch,
            editor_command,
            editor_batch,
            browser_command,
        }
    }
}
