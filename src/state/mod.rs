use std::fs::File;
use std::path::Path;

mod filter;
pub use crate::state::filter::FilterState;

use crate::error::Fallacy;
use crate::paper::Paper;

#[derive(Default)]
pub struct State {
    papers: Vec<Paper>,
    filters: FilterState,
}

impl State {
    pub fn load(state_path: &str) -> Result<Self, Fallacy> {
        let filepath = Path::new(state_path);
        if filepath.exists() {
            let file = match File::open(&filepath) {
                Ok(f) => f,
                Err(e) => return Err(Fallacy::StateLoadFailed(state_path.to_owned(), e)),
            };
            match serde_yaml::from_reader(file) {
                Ok(papers) => Ok(Self {
                    papers,
                    filters: FilterState::default(),
                }),
                Err(e) => Err(Fallacy::StateDeserializeFailed(state_path.to_owned(), e)),
            }
        } else {
            Ok(Self {
                papers: Vec::new(),
                filters: FilterState::default(),
            })
        }
    }

    pub fn store(&self, state_path: &str) -> Result<(), Fallacy> {
        let filepath = Path::new(state_path);
        let file = match File::open(&filepath) {
            Ok(f) => f,
            Err(e) => return Err(Fallacy::StateStoreFailed(state_path.to_owned(), e)),
        };
        match serde_yaml::to_writer(file, &self.papers) {
            Ok(()) => Ok(()),
            Err(e) => Err(Fallacy::StateSerializeFailed(state_path.to_owned(), e)),
        }
    }
}
