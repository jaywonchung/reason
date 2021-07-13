use crate::paper::{FilterInst, PaperFilter};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_filter() {
        let filter = FilterState::default();
        assert!(filter.history.len() == 0);
        assert_eq!(filter.current, 0);
        assert_eq!(filter.previous, 0);
    }

    #[test]
    fn one_filter() {
        let mut filter = FilterState::default();
        filter.apply(FilterInst::ADD(PaperFilterBuilder::new().build()?));
    }
}
