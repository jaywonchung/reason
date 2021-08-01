use std::fmt;

use regex::Regex;

use crate::error::Fallacy;
use crate::paper::Paper;

#[derive(Default, Debug, Clone)]
pub struct PaperFilter {
    pub title: Vec<Regex>,
    pub nickname: Vec<Regex>,
    pub author: Vec<Regex>,
    pub first_author: Vec<Regex>,
    pub venue: Vec<Regex>,
    pub year: Vec<Regex>,
    // TODO: tags or labels
}

impl PaperFilter {
    /// Accepts filter arguments given to commands and builds an
    /// instance of `PaperFilter`. Remove the command (first argument)
    /// and pass the rest to this function.
    pub fn from_args(args: &[String]) -> Result<Self, Fallacy> {
        let mut filter = Self::default();
        let mut arg_iter = args.iter();
        while let Some(arg) = arg_iter.next() {
            let (place, item) = match arg.as_ref() {
                "as" => (&mut filter.nickname, arg_iter.next()),
                "by" => (&mut filter.author, arg_iter.next()),
                "by1" => (&mut filter.first_author, arg_iter.next()),
                "at" => (&mut filter.venue, arg_iter.next()),
                "in" => (&mut filter.year, arg_iter.next()),
                _ => (&mut filter.title, Some(arg)),
            };
            let item = match item {
                Some(string) => string,
                None => return Err(Fallacy::FilterKeywordNoMatch(arg.to_string())),
            };
            match Regex::new(item) {
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
        }
        merged
    }

    /// Check if the filter matches the given paper.
    pub fn matches(&self, paper: &Paper) -> bool {
        macro_rules! checker {
            ($regex_field:ident) => {
                if !self
                    .$regex_field
                    .iter()
                    .all(|regex| regex.is_match(paper.$regex_field.as_ref()))
                {
                    return false;
                }
            };
            ($regex_field:ident, vector => $vec_field:ident) => {
                if !self
                    .$regex_field
                    .iter()
                    .all(|regex| paper.$vec_field.iter().any(|field| regex.is_match(field)))
                {
                    return false;
                }
            };
            ($regex_field:ident, getter => $field_getter:expr) => {
                if !self
                    .$regex_field
                    .iter()
                    .all(|regex| regex.is_match($field_getter))
                {
                    return false;
                }
            };
        }

        checker!(title);
        checker!(nickname, getter => paper.nickname.as_ref().unwrap_or(&"".to_string()));
        checker!(author, vector => authors);
        checker!(first_author, getter => paper.authors.first().unwrap_or(&"".to_string()));
        checker!(venue);
        checker!(year);

        true
    }

    /// Check if this filter is empty.
    pub fn is_empty(&self) -> bool {
        macro_rules! checker {
            ($field:ident) => {
                if self.$field.len() != 0 {
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

        true
    }
}

impl fmt::Display for PaperFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut segments = Vec::new();
        let displayer = |ret: &mut Vec<String>, filter: &Vec<Regex>, name: &str| {
            let joined = filter
                .iter()
                .map(|re| re.to_string())
                .reduce(|a, b| format!("{}, {}", a, b));
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

        if segments.len() == 0 {
            writeln!(f, "No filters are active.")
        } else {
            writeln!(f, "{}", segments.join(", "))
        }
    }
}
