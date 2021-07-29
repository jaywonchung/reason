use std::fs::File;
use std::path::PathBuf;

mod filter;
pub use crate::state::filter::{FilterInst, FilterState};

use crate::error::Fallacy;
use crate::paper::Paper;

#[derive(Default, Debug)]
pub struct State {
    pub papers: Vec<Paper>,
    pub filters: FilterState,
}

impl State {
    pub fn load(state_path: &PathBuf) -> Result<Self, Fallacy> {
        if state_path.exists() {
            // Open file for reading.
            let file = match File::open(state_path) {
                Ok(f) => f,
                Err(e) => return Err(Fallacy::StateLoadFailed(state_path.to_owned(), e)),
            };

            // Load state from the file.
            match serde_yaml::from_reader(file) {
                Ok(papers) => Ok(Self {
                    papers,
                    filters: FilterState::default(),
                }),
                Err(e) => Err(Fallacy::StateDeserializeFailed(state_path.to_owned(), e)),
            }
        } else {
            // Try creating the file to see if we have access.
            if let Some(dir) = state_path.parent() {
                if let Err(e) = std::fs::create_dir_all(dir) {
                    return Err(Fallacy::StateStoreFailed(dir.to_owned(), e));
                }
            }
            match File::create(state_path) {
                Ok(f) => f,
                Err(e) => return Err(Fallacy::StateStoreFailed(state_path.to_owned(), e)),
            };

            // Return default empty state.
            Ok(Self::default())
        }
    }

    pub fn store(&self, state_path: &PathBuf) -> Result<(), Fallacy> {
        let emergency_button = |state: &State| {
            eprintln!("Could not save state. Dumping to stderr!");
            eprintln!("== Debug string ==\n{:#?}\n", state);
            match serde_yaml::to_string(&state.papers) {
                Ok(s) => eprintln!("== Serialized string ==\n{}", s),
                Err(e) => eprintln!("== Serialization error ==\n{}", e),
            }
        };

        // Create/truncate the file first.
        if let Some(dir) = state_path.parent() {
            if let Err(e) = std::fs::create_dir_all(dir) {
                return Err(Fallacy::StateStoreFailed(dir.to_owned(), e));
            }
        }
        let file = match File::create(state_path) {
            Ok(f) => f,
            Err(e) => {
                emergency_button(&self);
                return Err(Fallacy::StateStoreFailed(state_path.to_owned(), e));
            }
        };

        // Store state into the file.
        match serde_yaml::to_writer(file, &self.papers) {
            Ok(()) => Ok(()),
            Err(e) => {
                emergency_button(&self);
                return Err(Fallacy::StateSerializeFailed(state_path.to_owned(), e));
            }
        }
    }
}
