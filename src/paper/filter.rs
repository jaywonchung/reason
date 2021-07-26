use std::fmt;

use derive_builder::Builder;
use regex::Regex;

pub enum FilterInst {
    /// cd something
    /// Add a new filter joined with AND.
    Add(PaperFilterPiece),
    /// cd .
    /// Changes nothing, but `cd -` takes this into account.
    Keep,
    /// cd ..
    /// Remove the most recent non-empty filter.
    Parent,
    /// cd
    /// Clear the filter history vector.
    Reset,
    /// cd -
    /// Reset to previous filter state.
    Prev,
}

#[derive(Builder, Default, Clone)]
#[builder(setter(strip_option))]
pub struct PaperFilterPiece {
    pub title: Option<Regex>,
    pub nickname: Option<Regex>,
    pub author: Option<Regex>,
    pub first_author: Option<Regex>,
    pub venue: Option<Regex>,
    pub year: Option<Regex>,
}

impl PaperFilterPiece {
    pub fn merge(pieces: &[PaperFilterPiece]) -> PaperFilter {
        let mut merged = PaperFilter::default();
        for filter in pieces {
            let cloned = filter.clone();
            merged.title.extend(cloned.title);
            merged.nickname.extend(cloned.nickname);
            merged.author.extend(cloned.author);
            merged.first_author.extend(cloned.first_author);
            merged.venue.extend(cloned.venue);
            merged.year.extend(cloned.year);
        }
        merged
    }
}

#[derive(Default)]
pub struct PaperFilter {
    pub title: Vec<Regex>,
    pub nickname: Vec<Regex>,
    pub author: Vec<Regex>,
    pub first_author: Vec<Regex>,
    pub venue: Vec<Regex>,
    pub year: Vec<Regex>,
}

impl fmt::Display for PaperFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut segments = Vec::new();
        let displayer = |ret: &mut Vec<String>, filter: &Vec<Regex>, name: &str| {
            let joined = filter
                .iter()
                .map(|re| re.to_string())
                .reduce(|a, b| format!("({})|({})", a, b));
            if let Some(joined) = joined {
                ret.push(format!("{} matches '{}'", name, joined));
            }
        };

        displayer(&mut segments, &self.title, "title");
        displayer(&mut segments, &self.nickname, "nickname");
        displayer(&mut segments, &self.author, "author");
        displayer(&mut segments, &self.first_author, "first_author");
        displayer(&mut segments, &self.venue, "venue");
        displayer(&mut segments, &self.year, "year");

        writeln!(f, "{}", segments.join(", "))
    }
}
