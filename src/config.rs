use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Fallacy;
use crate::utils::{expand_tilde, expand_tilde_string};

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
   (Not specified by default.)
- note_dir: The directory where markdown notes are stored.
   (default: ~/.local/share/reason/notes)

## Filter

- case_insensitive_regex: Whether filter regexes match
  in a case-insensitive manner.
   (default: false)

## Output

- table_columns: Which paper attributes `ls` shows.
  Allowed values are 'title', 'authors', 'first author',
  'venue', 'year', and 'state'.
   (default: ['title', 'first author', 'venue', 'year'])
- viewer_command: Command to use for the viewer to open
  papers. It is assumed that the viewer program is a
  non-command line program.
   (default: ['zathura'])
- viewer_batch: Whether to open multiple papers with a
  single invocation of the viewer command. If true, the
  command ran is: `viewer_command file1 file2 ...`.
  Otherwise, the viewer command is invoked once for each
  paper.
   (default: false)
- editor_command: Command to use for the editor to edit
  notes. It is assumed that the editor is a command line
  program.
   (default: ['vim', '-p'])
- editor_batch: Whether to open multiple notes with a
  single invocation of the editor command. If true, the
  command ran is: `viewer_command file1 file2 ...`.
  Otherwise, the editor command is invoked once for each
  paper.
   (default: true)
";

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
    pub file_base_dir: Option<PathBuf>,
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
        if self.viewer_command.len() == 0 {
            return Err(Fallacy::ConfigAuditError("Viewer command cannot be empty.".to_owned()));
        }
        for path in self.viewer_command.iter_mut() {
            *path = expand_tilde_string(path)?;
        }

        // Check editor command and expand tilde.
        if self.editor_command.len() == 0 {
            return Err(Fallacy::ConfigAuditError("Editor command cannot be empty.".to_owned()));
        }
        for path in self.editor_command.iter_mut() {
            *path = expand_tilde_string(path)?;
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

        let file_base_dir = None;

        let note_dir = {
            let mut path = data_dir;
            path.push("notes");
            path
        };

        Self {
            paper_metadata,
            command_history,
            max_history_size,
            file_base_dir,
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
        // TODO: See if Tabbed + Zathura works.
        let viewer_command = vec![String::from("zathura")];
        let viewer_batch = false;
        let editor_command = vec![String::from("vim"), String::from("-p")];
        let editor_batch = true;

        Self {
            table_columns,
            viewer_command,
            viewer_batch,
            editor_command,
            editor_batch,
        }
    }
}
