use std::fmt;

use regex::Regex;

use crate::error::Fallacy;

#[derive(Default, Debug)]
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
    /// instance of `PaperFilter`.
    pub fn from_args(args: &[String]) -> Result<Self, Fallacy> {
        let mut filter = Self::default();
        let mut arg_iter = args.iter().peekable();
        while let Some(arg) = arg_iter.next() {
            let (place, item) = match arg.as_ref() {
                "as" => (&mut filter.nickname, arg_iter.next()),
                "by" => (&mut filter.author, arg_iter.next()),
                "by1" => (&mut filter.first_author, arg_iter.next()),
                "at" | "on" => (&mut filter.venue, arg_iter.next()),
                "in" => (&mut filter.year, arg_iter.next()),
                _ => (&mut filter.title, Some(arg)),
            };
            let item = match item {
                Some(string) => string,
                None => return Err(Fallacy::FilterKeywordNoMatch(arg.to_string())),
            };
            place.push(Regex::new(item)?);
        }
        Ok(filter)
    }

    /// Merges multiple filters into one.
    pub fn merge(filters: &[Self]) -> Self {
        let mut merged = PaperFilter::default();
        for filter in filters {
            let cloned = filter.clone();
            merged.title.extend(filter.title.clone());
            merged.nickname.extend(cloned.nickname.clone());
            merged.author.extend(cloned.author.clone());
            merged.first_author.extend(cloned.first_author.clone());
            merged.venue.extend(cloned.venue.clone());
            merged.year.extend(cloned.year.clone());
        }
        merged
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

        writeln!(f, "{}", segments.join(", "))
    }
}
