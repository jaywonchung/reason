use std::collections::HashMap;
use std::fmt;

use chrono::prelude::*;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::Fallacy;
use crate::state::State;

pub struct PaperList {
    pub selected: Vec<usize>,
}

impl PaperList {
    pub fn into_string(self, state: &State, config: &Config) -> String {
        let mut table = Table::new();

        // Content width is dynamically arranged.
        table.set_content_arrangement(ContentArrangement::Dynamic);

        // Header line.
        let header = config.display.table_columns.iter().map(|s| {
            Cell::new(s)
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
        });
        table.set_header(header);

        // One row per paper.
        for ind in self.selected {
            let p = &state.papers[ind];
            let mut row = Vec::new();
            for col in config.display.table_columns.iter() {
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

    /// Where the paper was published, excluding the year. This field is queryable.
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

        Ok(Paper {
            title,
            nickname,
            authors,
            venue,
            year,
            state,
            filepath,
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
            "state" => self.state.first().unwrap().to_string(),
            _ => "".to_string(),
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
