use std::fmt;

use chrono::prelude::*;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use serde::{Deserialize, Serialize};

use crate::state::State;

pub struct PaperList {
    pub selected: Vec<usize>,
}

impl PaperList {
    pub fn into_string(self, state: &State) -> String {
        let mut table = Table::new();

        // Content width is dynamically arranged.
        table.set_content_arrangement(ContentArrangement::Dynamic);

        // First row
        table.set_header(vec![
            Cell::new("Title").set_alignment(CellAlignment::Center).add_attribute(Attribute::Bold),
            Cell::new("First Author").set_alignment(CellAlignment::Center).add_attribute(Attribute::Bold),
            Cell::new("Venue").set_alignment(CellAlignment::Center).add_attribute(Attribute::Bold),
            Cell::new("Year").set_alignment(CellAlignment::Center).add_attribute(Attribute::Bold),
            Cell::new("State").set_alignment(CellAlignment::Center).add_attribute(Attribute::Bold),
        ]);

        // One row per paper
        for ind in self.selected {
            let p = &state.papers[ind];
            table.add_row(vec![
                p.title.to_string(),
                p.authors.first().unwrap_or(&"".to_string()).to_string(),
                p.venue.to_string(),
                p.year.to_string(),
                p.state.to_string()
            ]);
        }

        table.to_string() + "\n"
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paper {
    pub title: String,
    pub nickname: String,
    pub authors: Vec<String>,
    pub venue: String,
    pub year: String,
    pub state: PaperStatus,
    // tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PaperStatus {
    Added(String),
    Read(String),
}

impl PaperStatus {
    fn read(&mut self) {
        *self = Self::Read(Local::today().to_string());
    }
}

impl Default for PaperStatus {
    fn default() -> Self {
        Self::Added(Local::now().to_string())
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
