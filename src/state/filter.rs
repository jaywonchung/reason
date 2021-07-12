use std::fmt;

use regex::Regex;

pub struct FilterState {
    history: Vec<PaperFilter>,
    current: usize,
    previous: usize,
}

impl FilterState {
    pub fn apply(&mut self, inst: FilterInst) {
        match inst {
            FilterInst::ADD(filter) => {
                self.previous = self.current;
                self.current += 1;
                if self.current == self.history.len() {
                    self.history.push(filter);
                } else {
                    self.history[self.current] = filter;
                }
            }
            FilterInst::KEEP => {
                self.previous = self.current;
                self.history.push(PaperFilter::default());
            }
            FilterInst::PARENT => {
                self.previous = self.current;
                if self.current > 0 {
                    self.current -= 1;
                }
            }
            FilterInst::RESET => {
                self.previous = self.current;
                self.current = 0;
            }
            FilterInst::PREV => {
                std::mem::swap(&mut self.previous, &mut self.current);
            }
        }
    }
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            history: vec![PaperFilter::default()],
            current: 0,
            previous: 0,
        }
    }
}

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

#[derive(Default, Clone)]
pub struct PaperFilter {
    title: Option<Regex>,
    by: Option<Regex>,
    at: Option<Regex>,
    on: Option<Regex>,
}

impl fmt::Display for PaperFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = Vec::new();
        if let Some(regex) = &self.title {
            ret.push(format!("title matches '{}'", regex));
        }
        if let Some(regex) = &self.by {
            ret.push(format!("author matches '{}'", regex));
        }
        if let Some(regex) = &self.at {
            ret.push(format!("venue matches '{}'", regex));
        }
        if let Some(regex) = &self.on {
            ret.push(format!("year matches '{}'", regex));
        }

        write!(f, "{}\n", ret.join(", "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref FILTERS: Vec<PaperFilter> = vec![
            PaperFilter {
                title: Some(Regex::new(r"a").unwrap()),
                by: None,
                at: None,
                on: None
            },
            PaperFilter {
                title: Some(Regex::new(r"b").unwrap()),
                by: None,
                at: None,
                on: None
            },
            PaperFilter {
                title: None,
                by: Some(Regex::new(r"Chung").unwrap()),
                at: None,
                on: None
            },
            PaperFilter {
                title: None,
                by: None,
                at: Some(Regex::new(r"OSDI").unwrap()),
                on: None
            },
        ];
    }

    #[test]
    fn add_and_add() {
        let mut state = FilterState::default();
        state.apply(FilterInst::ADD(FILTERS[0].clone()));
        state.apply(FilterInst::ADD(FILTERS[1].clone()));
        assert_eq!(state.history[1].to_string(), FILTERS[1].to_string());
        assert_eq!(state.history[2].to_string(), FILTERS[2].to_string());
        assert_eq!(state.current, 2);
        assert_eq!(state.previous, 1);
    }
}
