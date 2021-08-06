use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::path::PathBuf;

use chrono::prelude::*;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::Fallacy;
use crate::state::State;
use crate::utils::expand_tilde_string;

pub static MAN: &'static str = "Paper metadata.

Reason keeps metadata for each paper in its paperpase.
- title: The title of the paper, in full.
- nickname: The nickname of the paper. For instance,
   the name of the system.
- authors: A list of authors, in order.
- venue: Where the paper was published, excluding year.
- year: The year when the paper was published.
- filepath: The path to the PDF file of the paper.
- state: The management state history of the paper.
   Two states are supported: ADDED and READ.
- notepath: 
";

pub struct PaperList(pub Vec<usize>);

impl PaperList {
    pub fn into_string(self, state: &State, config: &Config) -> String {
        let mut table = Table::new();

        // Content width is dynamically arranged.
        table.set_content_arrangement(ContentArrangement::Dynamic);

        // Header line.
        let header = config.output.table_columns.iter().map(|s| {
            Cell::new(s)
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
        });
        table.set_header(header);

        // One row per paper.
        for ind in self.0 {
            let p = &state.papers[ind];
            let mut row = Vec::new();
            for col in config.output.table_columns.iter() {
                row.push(p.field_as_string(col));
            }
            table.add_row(row);
        }

        table.to_string() + "\n"
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Paper {
    /// The title of the paper, in full. This field is queryable.
    /// Keyword: None. An argument without a keyword is considered a title.
    pub title: String,

    /// The nickname of the paper. For instance, the name of the system.
    /// This field is queryable.
    /// Keyword: 'as'
    pub nickname: Option<String>,

    /// A list of authors, in order. This field is queryable.
    /// Keyword: 'by', 'by1' (first author)
    pub authors: Vec<String>,

    /// Where the paper was published, excluding year. This field is queryable.
    /// Keyword: 'at'
    pub venue: String,

    /// The year when the paper was published. This field is queryable.
    /// Keyword: 'in'
    pub year: String,

    /// The path to the PDF file of the paper. The user may choose not to specify one.
    /// Keyword: '@'
    pub filepath: Option<String>,

    /// The management state history of the paper.
    pub state: Vec<PaperStatus>,

    /// The path to the markdown note of the paper. File names are created with the
    /// title of the paper. If collisions are detected, an integer will be appended
    /// to the file name.
    pub notepath: Option<String>,
}

impl Paper {
    /// Accepts arguments given to commands and builds an instance
    /// of `Paper`. Remove the command (first argument) and pass the
    /// rest to this function.
    pub fn from_args(args: Vec<String>) -> Result<Self, Fallacy> {
        // Collect a mapping of keyword -> Option<argument>.
        let mut map = HashMap::new();
        let mut arg_iter = args.into_iter();
        while let Some(arg) = arg_iter.next() {
            match arg.as_ref() {
                "as" | "by" | "at" | "in" | "@" => {
                    if map.contains_key(arg.as_str()) {
                        return Err(Fallacy::PaperDuplicateField(arg));
                    }
                    map.insert(arg, arg_iter.next());
                }
                _ => {
                    if map.contains_key("title") {
                        return Err(Fallacy::PaperDuplicateField("title".to_owned()));
                    }
                    map.insert("_".to_owned(), Some(arg));
                }
            }
        }

        // Collect a mapping of field -> argument, along with missing required fields.
        let mut missing = Vec::new();
        let mut fields: HashMap<&str, String> = HashMap::new();
        for (keyword, field_name, required) in [
            ("by", "authors", true),
            ("at", "venue", true),
            ("in", "year", true),
            ("_", "title", true),
            ("as", "nickname", false),
            ("@", "filepath", false),
        ] {
            match map.remove(keyword) {
                Some(Some(string)) => {
                    fields.insert(field_name, string);
                }
                _ => {
                    if required {
                        missing.push(format!("{}({})", field_name, keyword));
                    }
                }
            }
        }

        // Report error if there are missing fields.
        if missing.len() != 0 {
            return Err(Fallacy::PaperMissingFields(missing.join(", ")));
        }

        // Make paper fields.
        let title = fields.remove("title").unwrap();
        let nickname = fields.remove("nickname");
        let authors = fields
            .remove("authors")
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let venue = fields.remove("venue").unwrap();
        let year = fields.remove("year").unwrap();
        let state = vec![PaperStatus::new()];
        let filepath = fields.remove("filepath");
        let notepath = None;

        Ok(Paper {
            title,
            nickname,
            authors,
            venue,
            year,
            state,
            filepath,
            notepath,
        })
    }

    pub fn field_as_string(&self, field: &str) -> String {
        match field {
            "title" => self.title.clone(),
            "nickname" => self.nickname.clone().unwrap_or_default(),
            "authors" => self.authors.join(", "),
            "first author" => self.authors.first().unwrap().clone(),
            "venue" => self.venue.clone(),
            "year" => self.year.clone(),
            "state" => self.state.last().unwrap().to_string(),
            _ => "".to_string(),
        }
    }

    pub fn note_path(&mut self, note_dir: &PathBuf) -> Result<String, Fallacy> {
        // No notes for this paper. Create one!
        if self.notepath.is_none() {
            // Generate a unique filepath for this paper.
            // By default the nickname of the paper + .md.
            // If the paper doesn't have a nickname, the title is used.
            let file = match self.nickname.clone() {
                Some(string) => string,
                None => self
                    .title
                    .clone()
                    .replace(|c: char| c.is_whitespace(), "-")
                    .replace(|c: char| c != '-' && !c.is_ascii_alphanumeric(), ""),
            };

            // Find a filename that doesn't exist.
            let mut attempt = 0usize;
            let path = loop {
                let mut path = note_dir.clone();
                let mut filename = file.clone();
                if attempt == 0 {
                    filename.push_str(".md");
                } else {
                    let formatted = format!("-{}.md", attempt);
                    filename.push_str(&formatted);
                }
                path.push(filename);
                if !path.exists() {
                    break path;
                } else {
                    attempt += 1;
                }
            };

            // Create the note file and initialize it with some metadata.
            if let Err(e) = std::fs::create_dir_all(&path.parent().unwrap()) {
                return Err(e.into());
            }
            println!("{:?}", path);
            match std::fs::File::create(&path) {
                Ok(mut file) => {
                    if let Err(e) = write!(
                        file,
                        "- {}\n- {}\n- {} {}\n\n",
                        self.title,
                        self.authors.join(", "),
                        self.venue,
                        self.year
                    ) {
                        return Err(e.into());
                    }
                    drop(file);
                }
                Err(e) => return Err(e.into()),
            };

            match path.to_str() {
                Some(path) => self.notepath.replace(path.to_owned()),
                None => return Err(Fallacy::PathInvalidUTF8(path)),
            };
        }
        Ok(self.notepath.clone().unwrap())
    }

    /// Create an absolute path to the paper file.
    /// Returns Ok(None) if the paper does not have a filepath.
    pub fn abs_filepath(&self, config: &Config) -> Result<Option<PathBuf>, Fallacy> {
        if let Some(filepath) = self.filepath.as_ref() {
            let path = PathBuf::from(expand_tilde_string(filepath)?);
            // Path is already absolute.
            if path.is_absolute() {
                return Ok(Some(path));
            }
            // Relative path.
            if let Some(base) = config.storage.file_base_dir.as_ref() {
                let mut base = base.clone();
                base.push(path);
                Ok(Some(base))
            } else {
                Err(Fallacy::PathRelativeWithoutBase(path))
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PaperStatus {
    Added(String),
    Read(String),
}

impl PaperStatus {
    pub fn new() -> Self {
        Self::Added(Local::now().format("%F %r").to_string())
    }

    pub fn read(&mut self) {
        *self = Self::Read(Local::now().format("%F %r").to_string());
    }
}

impl fmt::Display for PaperStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaperStatus::Added(datetime) => write!(f, "ADDED {}", datetime),
            PaperStatus::Read(datetime) => write!(f, "READ  {}", datetime),
        }
    }
}
