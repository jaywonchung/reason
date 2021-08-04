use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Fallacy;
use crate::utils::expand_tilde;

pub static MAN: &'static str = "Reason configuration.

Config file location: '~/.config/reason/config.toml'.
If nothing is there, reason will create one populated
with default settings.

## Storage

- paper_metadata: Path to store paper metadata.
   (default: ~/.local/share/reason/metadata.yaml)
- command_history: Path to store command history.
   (default: ~/.local/share/reason/history.txt)
- max_history_size: How many commands to keep in history.
   (default: 1000)
- file_base_dir: The base directory where paper files
  are stored. If set, you can just specify the file path
  relative to this directory with 'touch @'.
   (default: None)

## Filter

- case_insensitive_regex: Whether filter regexes match
  in a case-insensitive manner.
   (default: false)

## Display

- table_columns: Which paper attributes `ls` shows.
  Allowed values are 'title', 'authors', 'first author',
  'venue', 'year', and 'state'.
   (default: ['title', 'first author', 'venue', 'year'])
- viewer_binary_path: The path to the viewer that will
  be used to open papers.
   (default: /usr/bin/zathura)
";

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub storage: StorageConfig,
    pub filter: FilterConfig,
    pub display: DisplayConfig,
}

#[derive(Serialize, Deserialize)]
pub struct StorageConfig {
    pub paper_metadata: PathBuf,
    pub command_history: PathBuf,
    pub max_history_size: usize,
    pub file_base_dir: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct FilterConfig {
    pub case_insensitive_regex: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DisplayConfig {
    pub table_columns: Vec<String>,
    pub viewer_binary_path: PathBuf,
}

impl Config {
    pub fn validate(&mut self) -> Result<(), Fallacy> {
        self.storage.validate()?;
        self.filter.validate()?;
        self.display.validate()?;
        Ok(())
    }
}

impl StorageConfig {
    fn validate(&mut self) -> Result<(), Fallacy> {
        self.paper_metadata = expand_tilde(&self.paper_metadata)?;
        self.command_history = expand_tilde(&self.command_history)?;
        if let Some(dir) = &self.file_base_dir {
            self.file_base_dir = Some(expand_tilde(dir)?);
        }
        Ok(())
    }
}

impl FilterConfig {
    fn validate(&mut self) -> Result<(), Fallacy> {
        Ok(())
    }
}

impl DisplayConfig {
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

        // Expand viewer binary path.
        expand_tilde(&mut self.viewer_binary_path)?;
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
            let mut path = data_dir;
            path.push("history.txt");
            path
        };

        let max_history_size = 1000;

        let file_base_dir = None;

        Self {
            paper_metadata,
            command_history,
            max_history_size,
            file_base_dir,
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

impl Default for DisplayConfig {
    fn default() -> Self {
        let table_columns = vec!["title", "first author", "venue", "year"];
        let table_columns = table_columns.into_iter().map(|s| s.to_string()).collect();
        let viewer_binary_path = PathBuf::from("/usr/bin/zathura");

        Self {
            table_columns,
            viewer_binary_path,
        }
    }
}
