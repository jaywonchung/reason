use std::path::PathBuf;

use thiserror::Error;

/// A use of invalid or faulty reason.
#[derive(Error, Debug)]
pub enum Fallacy {
    // Critical errors
    #[error("Failed to load paper metadata from '{0}': '{1}'")]
    StateLoadFailed(PathBuf, std::io::Error),
    #[error("Failed to parse paper metadata loaded from '{0}': '{1}'")]
    StateDeserializeFailed(PathBuf, serde_yaml::Error),
    #[error("Failed to store paper metadata to '{0}': '{1}'")]
    StateStoreFailed(PathBuf, std::io::Error),
    #[error("Failed to serialize and store paper metadata to '{0}': '{1}'")]
    StateSerializeFailed(PathBuf, serde_yaml::Error),
    #[error("Failed to store command history to '{0}': '{1}'")]
    HistoryStoreFailed(PathBuf, std::io::Error),
    #[error("Failed to store command history to '{0}': '{1}'")]
    RLHistoryStoreFailed(PathBuf, rustyline::error::ReadlineError),

    // Non-critical errors
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),
    #[error("Invalid filter: '{0}'")]
    InvalidFilter(#[from] regex::Error),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Failed to build filter from regex: '{0}'")]
    FilterBuildFailed(regex::Error),
    #[error("No matching regex for keyword '{0}'")]
    FilterKeywordNoMatch(String),
    #[error("Exit reason")]
    ExitReason,
}
