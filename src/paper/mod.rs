use std::fmt;

use chrono::prelude::*;
use prettytable::{cell, row, Table};
use serde::{Deserialize, Serialize};

mod filter;
pub use crate::paper::filter::{
    FilterInst, PaperFilter, PaperFilterPiece, PaperFilterPieceBuilder,
    PaperFilterPieceBuilderError,
};

#[derive(Default)]
pub struct Papers(Vec<Paper>);

impl fmt::Display for Papers {
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
        for p in self.0.iter() {
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

#[derive(Serialize, Deserialize)]
pub struct Paper {
    title: String,
    nickname: String,
    authors: Vec<String>,
    venue: String,
    year: u32,
    state: PaperStatus,
}

#[derive(Serialize, Deserialize)]
pub enum PaperStatus {
    Added(String),
    Read(String),
}

impl PaperStatus {
    fn read(mut self) {
        self = Self::Read(Local::now().to_string());
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
