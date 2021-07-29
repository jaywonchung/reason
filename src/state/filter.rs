use crate::filter::PaperFilter;

pub enum FilterInst {
    /// cd something
    /// Add a new filter joined with AND.
    Add(PaperFilter),
    /// cd .
    /// Changes nothing, but `cd -` takes this into account.
    Here,
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

#[derive(Debug)]
pub struct FilterState {
    history: Vec<PaperFilter>,
    current: usize,
    previous: usize,
}

impl FilterState {
    pub fn record(&mut self, inst: FilterInst) {
        match inst {
            FilterInst::Add(filter) => {
                self.previous = self.current;
                self.current += 1;
                if self.current == self.history.len() {
                    self.history.push(filter);
                } else {
                    self.history[self.current] = filter;
                }
            }
            FilterInst::Here => {
                self.previous = self.current;
                self.current += 1;
                let filter = PaperFilter::default();
                if self.current == self.history.len() {
                    self.history.push(filter);
                } else {
                    self.history[self.current] = filter;
                }
            }
            FilterInst::Parent => {
                self.previous = self.current;
                if self.current > 0 {
                    self.current -= 1;
                }
            }
            FilterInst::Reset => {
                self.previous = self.current;
                self.current = 0;
            }
            FilterInst::Prev => {
                std::mem::swap(&mut self.previous, &mut self.current);
            }
        }
    }

    pub fn current(&self) -> PaperFilter {
        PaperFilter::merge(&self.history[..self.current + 1])
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::filter::PaperFilterBuilder;
    use regex::Regex;

    #[test]
    fn empty_filter() {
        let fstate = FilterState::default();
        assert_eq!(fstate.history.len(), 1);
        assert_eq!(fstate.current, 0);
        assert_eq!(fstate.previous, 0);
    }

    #[test]
    fn one_filter() {
        let mut fstate = FilterState::default();
        fstate.record(FilterInst::Add(
            PaperFilterBuilder::default()
                .author(vec![Regex::new("Chung").unwrap()])
                .build()
                .unwrap(),
        ));
        assert_eq!(fstate.history.len(), 2);
        assert_eq!(fstate.current, 1);
        assert_eq!(fstate.previous, 0);
    }

    #[test]
    fn two_filters() {
        let mut fstate = FilterState::default();
        fstate.record(FilterInst::Add(
            PaperFilterBuilder::default()
                .author(vec![Regex::new("Chung").unwrap()])
                .build()
                .unwrap(),
        ));
        fstate.record(FilterInst::Add(
            PaperFilterBuilder::default()
                .nickname(vec![Regex::new("ShadowTutor").unwrap()])
                .build()
                .unwrap(),
        ));
        assert_eq!(fstate.history.len(), 3);
        assert_eq!(fstate.current, 2);
        assert_eq!(fstate.previous, 1);
    }

    #[test]
    fn two_then_parent() {
        let mut fstate = FilterState::default();
        fstate.record(FilterInst::Add(
            PaperFilterBuilder::default()
                .author(vec![Regex::new("Chung").unwrap()])
                .build()
                .unwrap(),
        ));
        fstate.record(FilterInst::Add(
            PaperFilterBuilder::default()
                .nickname(vec![Regex::new("ShadowTutor").unwrap()])
                .build()
                .unwrap(),
        ));
        fstate.record(FilterInst::Prev);
        assert_eq!(fstate.history.len(), 3);
        assert_eq!(fstate.current, 1);
        assert_eq!(fstate.previous, 2);
    }
}
