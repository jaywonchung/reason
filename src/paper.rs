use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};

use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::Fallacy;
use crate::state::State;
use crate::utils::{as_filename, confirm};

pub static MAN: &str = include_str!("../man/paper.md");

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
    pub filepath: Option<PathBuf>,

    /// Labels assigned to this paper.
    /// Keyword: 'is', 'not'
    pub labels: HashSet<String>,

    /// The path to the markdown note of the paper. File names are created with the
    /// title of the paper. If collisions are detected, an integer will be appended
    /// to the file name.
    pub notepath: Option<PathBuf>,
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
                "as" | "by" | "at" | "in" | "@" | "is" => {
                    if map.contains_key(arg.as_str()) {
                        return Err(Fallacy::PaperDuplicateField(arg));
                    }
                    map.insert(arg, arg_iter.next());
                }
                _ => {
                    if map.contains_key("_") {
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
            ("is", "labels", false),
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
        if !missing.is_empty() {
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
        let labels = fields
            .remove("labels")
            .map(|l| l.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        let filepath = fields.remove("filepath").map(PathBuf::from);
        let notepath = None;

        Ok(Paper {
            title,
            nickname,
            authors,
            venue,
            year,
            labels,
            filepath,
            notepath,
        })
    }

    pub fn apply_from_args(&mut self, args: &[String]) -> Result<(), Fallacy> {
        // Collect a mapping of keyword -> Option<argument>.
        let mut map = HashMap::new();
        let mut arg_iter = args.iter().cloned();
        while let Some(arg) = arg_iter.next() {
            match arg.as_ref() {
                "as" | "by" | "at" | "in" | "is" | "not" => {
                    if map.contains_key(arg.as_str()) {
                        return Err(Fallacy::PaperDuplicateField(arg));
                    }
                    if let Some(field) = arg_iter.next() {
                        map.insert(arg, field);
                    } else {
                        map.insert("_".to_owned(), arg);
                    }
                }
                _ => {
                    if map.contains_key("_") {
                        return Err(Fallacy::PaperDuplicateField("title".to_owned()));
                    }
                    map.insert("_".to_owned(), arg);
                }
            }
        }

        // Apply changes.
        if let Some(title) = map.remove("_") {
            self.title = title;
        }
        if let Some(nickname) = map.remove("as") {
            self.nickname = Some(nickname);
        }
        if let Some(authors) = map.remove("by") {
            self.authors = authors.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Some(venue) = map.remove("at") {
            self.venue = venue;
        }
        if let Some(year) = map.remove("in") {
            self.year = year;
        }
        if let Some(labels) = map.remove("is") {
            for label in labels.split(',') {
                self.labels.insert(label.trim().to_string());
            }
        }
        if let Some(labels) = map.remove("not") {
            for label in labels.split(',') {
                self.labels.remove(&label.trim().to_string());
            }
        }

        Ok(())
    }

    pub fn field_as_string(&self, field: &str) -> String {
        match field {
            "title" => self.title.clone(),
            "nickname" => self.nickname.clone().unwrap_or_default(),
            "authors" => self.authors.join(", "),
            "first author" => self.authors.first().unwrap().clone(),
            "venue" => self.venue.clone(),
            "year" => self.year.clone(),
            _ => "".to_string(),
        }
    }

    pub fn note_path(&mut self, note_dir: &Path) -> Result<PathBuf, Fallacy> {
        // No notes for this paper. Create one!
        if self.notepath.is_none() {
            // Generate a filename for this paper.
            let file = match self.nickname.clone() {
                Some(string) => as_filename(&string),
                None => as_filename(&self.title),
            };

            // Find a filename that doesn't exist.
            let mut attempt = 0usize;
            let path = loop {
                let mut path = note_dir.to_path_buf();
                let mut filename = file.clone();
                if attempt == 0 {
                    filename.push_str(".md");
                } else {
                    let formatted = format!("-{}.md", attempt);
                    filename.push_str(&formatted);
                }
                path.push(&filename);
                if !path.exists() {
                    // Record in state.
                    self.notepath.replace(PathBuf::from(filename));
                    break path;
                } else {
                    attempt += 1;
                }
            };

            self.create_note(&path)?;
        }

        // Unwrap will never fail.
        let relative = self.notepath.as_ref().unwrap();
        let mut path = note_dir.to_path_buf();
        path.push(relative);

        // Notepath exists, but the file doesn't actually exist.
        if !path.exists() {
            confirm(
                format!(
                    "Note file {:?} does not exist. Create a new one here?",
                    path
                ),
                true,
            )?;
            self.create_note(&path)?;
        }

        Ok(path)
    }

    fn create_note(&self, path: &Path) -> Result<(), Fallacy> {
        // Parent directory was already created during config validation.
        match std::fs::File::create(&path) {
            Ok(mut file) => {
                if let Err(e) = write!(
                    file,
                    "# {}\n\n- {}\n- {} {}\n\n",
                    self.title,
                    self.authors.join(", "),
                    self.venue,
                    self.year
                ) {
                    return Err(e.into());
                }
            }
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    /// Return the absolute path to the paper file.
    /// Returns None if the paper does not have a filepath.
    pub fn filepath(&self, config: &Config) -> Option<PathBuf> {
        self.filepath.as_ref().map(|filepath| {
            let mut base = config.storage.file_dir.clone();
            base.push(filepath);
            base
        })
    }
}
