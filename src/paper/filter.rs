use std::fmt;

use derive_builder::Builder;
use regex::Regex;

pub enum FilterInst {
    /// cd something
    /// Add a new filter joined with AND.
    ADD(PaperFilter),
    /// cd .
    /// Changes nothing, but `cd -` takes this into account.
    KEEP,
    /// cd ..
    /// Remove the most recent non-empty filter.
    PARENT,
    /// cd
    /// Clear the filter history vector.
    RESET,
    /// cd -
    /// Reset to previous filter state.
    PREV,
}

#[derive(Builder, Default, Clone)]
#[builder(setter(strip_option))]
pub struct PaperFilter {
    title: Option<Regex>,
    nickname: Option<Regex>,
    author: Option<Regex>,
    first_author: Option<Regex>,
    venue: Option<Regex>,
    year: Option<Regex>,
}

impl fmt::Display for PaperFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = Vec::new();
        if let Some(regex) = &self.title {
            ret.push(format!("title matches '{}'", regex));
        }
        if let Some(regex) = &self.nickname {
            ret.push(format!("nickname matches '{}'", regex));
        }
        if let Some(regex) = &self.author {
            ret.push(format!("author matches '{}'", regex));
        }
        if let Some(regex) = &self.first_author {
            ret.push(format!("first author matches '{}'", regex));
        }
        if let Some(regex) = &self.venue {
            ret.push(format!("venue matches '{}'", regex));
        }
        if let Some(regex) = &self.year {
            ret.push(format!("year matches '{}'", regex));
        }

        write!(f, "{}\n", ret.join(", "))
    }
}
