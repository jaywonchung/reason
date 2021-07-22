use crate::paper::{FilterInst, PaperFilter, PaperFilterPiece};

pub struct FilterState {
    history: Vec<PaperFilterPiece>,
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
            FilterInst::Keep => {
                self.previous = self.current;
                self.current += 1;
                let filter = PaperFilterPiece::default();
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
        let mut current = PaperFilter::default();
        for filter in &self.history[..self.current + 1] {
            let cloned = filter.clone();
            current.title.extend(cloned.title);
            current.nickname.extend(cloned.nickname);
            current.author.extend(cloned.author);
            current.first_author.extend(cloned.first_author);
            current.venue.extend(cloned.venue);
            current.year.extend(cloned.year);
        }
        current
    }
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            history: vec![PaperFilterPiece::default()],
            current: 0,
            previous: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::paper::PaperFilterPieceBuilder;
    use regex::Regex;

    #[test]
    fn empty_filter() {
        let fstate = FilterState::default();
        assert_eq!(fstate.history.len(), 0);
        assert_eq!(fstate.current, 0);
        assert_eq!(fstate.previous, 0);
    }

    #[test]
    fn one_filter() -> Result<(), Box<dyn std::error::Error>> {
        let mut fstate = FilterState::default();
        fstate.record(FilterInst::Add(
            PaperFilterPieceBuilder::default()
                .author(Regex::new("Chung")?)
                .build()?,
        ));
        assert_eq!(fstate.history.len(), 1);
        assert_eq!(fstate.current, 1);
        assert_eq!(fstate.previous, 0);
        Ok(())
    }

    #[test]
    fn two_filters() -> Result<(), Box<dyn std::error::Error>> {
        let mut fstate = FilterState::default();
        fstate.record(FilterInst::Add(
            PaperFilterPieceBuilder::default()
                .author(Regex::new("Chung")?)
                .build()?,
        ));
        fstate.record(FilterInst::Add(
            PaperFilterPieceBuilder::default()
                .nickname(Regex::new("ShadowTutor")?)
                .build()?,
        ));
        assert_eq!(fstate.history.len(), 2);
        assert_eq!(fstate.current, 2);
        assert_eq!(fstate.previous, 1);
        Ok(())
    }

    #[test]
    fn two_then_parent() -> Result<(), Box<dyn std::error::Error>> {
        let mut fstate = FilterState::default();
        fstate.record(FilterInst::Add(
            PaperFilterPieceBuilder::default()
                .author(Regex::new("Chung")?)
                .build()?,
        ));
        fstate.record(FilterInst::Add(
            PaperFilterPieceBuilder::default()
                .nickname(Regex::new("ShadowTutor")?)
                .build()?,
        ));
        fstate.record(FilterInst::Prev);
        assert_eq!(fstate.history.len(), 2);
        assert_eq!(fstate.current, 1);
        assert_eq!(fstate.previous, 2);
        Ok(())
    }
}
