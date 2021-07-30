use std::fmt;

use chrono::prelude::*;
use prettytable::{cell, row, Table};
use serde::{Deserialize, Serialize};

pub struct PaperList<'p> {
    papers: &'p Vec<Paper>,
    pub selected: Vec<usize>,
}

impl<'p> PaperList<'p> {
    pub fn new(papers: &'p Vec<Paper>) -> Self {
        let selected = (0..papers.len()).collect();
        Self { papers, selected }
    }
}

impl<'p> fmt::Display for PaperList<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        for &ind in self.selected.iter() {
            let p = &self.papers[ind];
            table.add_row(row![
                p.title,
                p.authors.first().unwrap_or(&"".to_string()),
                p.venue,
                p.year.to_string(),
                p.state.to_string(),
            ]);
        }

        write!(f, "{}", table)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paper {
    title: String,
    nickname: String,
    authors: Vec<String>,
    venue: String,
    year: u32,
    state: PaperStatus,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PaperStatus {
    Added(String),
    Read(String),
}

impl PaperStatus {
    fn read(&mut self) {
        *self = Self::Read(Local::now().to_string());
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
