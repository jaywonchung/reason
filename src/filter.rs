use std::fmt;

use regex::{Regex, RegexBuilder};

use crate::error::Fallacy;
use crate::paper::Paper;

pub static MAN: &str = include_str!("../man/filter.md");

#[derive(Default, Debug, Clone)]
pub struct PaperFilter {
    pub title: Vec<Regex>,
    pub nickname: Vec<Regex>,
    pub author: Vec<Regex>,
    pub first_author: Vec<Regex>,
    pub venue: Vec<Regex>,
    pub year: Vec<Regex>,
    pub is_label: Vec<Regex>,
    pub not_label: Vec<Regex>,
}

impl PaperFilter {
    /// Accepts filter arguments given to commands and builds an
    /// instance of `PaperFilter`. Remove the command (first argument)
    /// and pass the rest to this function.
    pub fn from_args(args: &[String], case_insensitive: bool) -> Result<Self, Fallacy> {
        let mut filter = Self::default();
        let mut arg_iter = args.iter();
        while let Some(arg) = arg_iter.next() {
            let (mut place, item) = match arg.as_ref() {
                "as" => (&mut filter.nickname, arg_iter.next()),
                "by" => (&mut filter.author, arg_iter.next()),
                "by1" => (&mut filter.first_author, arg_iter.next()),
                "at" => (&mut filter.venue, arg_iter.next()),
                "in" => (&mut filter.year, arg_iter.next()),
                "is" => (&mut filter.is_label, arg_iter.next()),
                "not" => (&mut filter.not_label, arg_iter.next()),
                _ => (&mut filter.title, Some(arg)),
            };
            let item = match item {
                Some(string) => string,
                None => {
                    // If no matching regex is found, instead match title.
                    place = &mut filter.title;
                    arg
                }
            };
            match RegexBuilder::new(item)
                .case_insensitive(case_insensitive)
                .build()
            {
                Ok(regex) => place.push(regex),
                Err(e) => return Err(Fallacy::FilterBuildFailed(e)),
            }
        }
        Ok(filter)
    }

    /// Merges multiple filters into one.
    pub fn merge(filters: &[Self]) -> Self {
        let mut merged = Self::default();
        for filter in filters {
            merged.title.extend(filter.title.clone());
            merged.nickname.extend(filter.nickname.clone());
            merged.author.extend(filter.author.clone());
            merged.first_author.extend(filter.first_author.clone());
            merged.venue.extend(filter.venue.clone());
            merged.year.extend(filter.year.clone());
            merged.is_label.extend(filter.is_label.clone());
            merged.not_label.extend(filter.not_label.clone());
        }
        merged
    }

    /// Check if the filter matches the given paper.
    pub fn matches(&self, paper: &Paper) -> bool {
        macro_rules! checker {
            // A field value should match all regexes in the filter.
            ($regex_field:ident) => {
                if !self
                    .$regex_field
                    .iter()
                    .all(|regex| regex.is_match(paper.$regex_field.as_ref()))
                {
                    return false;
                }
            };
            // Same as above, but with a custom field getter.
            ($regex_field:ident, getter => $field_getter:expr) => {
                if !self
                    .$regex_field
                    .iter()
                    .all(|regex| regex.is_match($field_getter))
                {
                    return false;
                }
            };
            // At least one element in the vector field should match all regexes in the filter.
            ($regex_field:ident, vector => $vec_field:ident) => {
                if !self
                    .$regex_field
                    .iter()
                    .all(|regex| paper.$vec_field.iter().any(|field| regex.is_match(field)))
                {
                    return false;
                }
            };
            // None of the elements in the vector field should match any regex in the filter.
            ($regex_field:ident, vector =!> $vec_field:ident) => {
                if !self
                    .$regex_field
                    .iter()
                    .all(|regex| paper.$vec_field.iter().all(|field| !regex.is_match(field)))
                {
                    return false;
                }
            };
        }

        checker!(title);
        checker!(nickname, getter => paper.nickname.as_ref().unwrap_or(&"".to_string()));
        checker!(author, vector => authors);
        checker!(first_author, getter => paper.authors.first().unwrap());
        checker!(venue);
        checker!(year);
        checker!(is_label, vector => labels);
        checker!(not_label, vector =!> labels);

        true
    }

    /// Check if this filter is empty.
    pub fn is_empty(&self) -> bool {
        macro_rules! checker {
            ($field:ident) => {
                if !self.$field.is_empty() {
                    return false;
                }
            };
        }

        checker!(title);
        checker!(nickname);
        checker!(author);
        checker!(first_author);
        checker!(venue);
        checker!(year);
        checker!(is_label);
        checker!(not_label);

        true
    }
}

impl fmt::Display for PaperFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut segments = Vec::new();
        let displayer = |ret: &mut Vec<String>, filter: &Vec<Regex>, name: &str, matches: bool| {
            let joined = filter
                .iter()
                .map(|re| re.to_string())
                .reduce(|a, b| format!("{}' & '{}", a, b));
            if let Some(joined) = joined {
                ret.push(format!(
                    "{} {} '{}'",
                    name,
                    if matches { "matches" } else { "does not match" },
                    joined
                ));
            }
        };

        displayer(&mut segments, &self.title, "title", true);
        displayer(&mut segments, &self.nickname, "nickname", true);
        displayer(&mut segments, &self.author, "author", true);
        displayer(&mut segments, &self.first_author, "first_author", true);
        displayer(&mut segments, &self.venue, "venue", true);
        displayer(&mut segments, &self.year, "year", true);
        displayer(&mut segments, &self.is_label, "label", true);
        displayer(&mut segments, &self.not_label, "label", false);

        if segments.is_empty() {
            writeln!(f, "No filters are active.")
        } else {
            writeln!(f, "{}", segments.join(", "))
        }
    }
}
