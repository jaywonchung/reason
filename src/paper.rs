use std::fmt;

use chrono::prelude::*;
use prettytable::{cell, row, Table};
use serde::{Deserialize, Serialize};

use crate::state::State;

pub struct PaperList {
    pub selected: Vec<usize>,
}

impl PaperList {
    pub fn new(length: usize) -> Self {
        let selected = (0..length).collect();
        Self { selected }
    }
}

impl PaperList {
    pub fn into_string(self, state: &State) -> String {
        let mut table = Table::new();

        // First row
        table.add_row(row![
            bc->"Title",
            bc->"First Author",
            bc->"Venue",
            bc->"Year",
            bc->"State"
        ]);

        // One row per paper
        for ind in self.selected {
            let p = &state.papers[ind];
            table.add_row(row![
                p.title,
                p.authors.first().unwrap_or(&"".to_string()),
                p.venue,
                p.year.to_string(),
                p.state.to_string(),
            ]);
        }

        table.to_string()
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
