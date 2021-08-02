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
    #[error("Failed to load reason config: '{0}'")]
    ConfigLoadFailed(#[from] confy::ConfyError),
    #[error("Failed to read config: '{0}'")]
    ConfigAuditError(String),

    // Non-critical errors
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),
    #[error("Invalid filter: '{0}'")]
    InvalidFilter(#[from] regex::Error),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Failed to build filter from regex:\n{0}")]
    FilterBuildFailed(regex::Error),
    #[error("No matching regex for keyword '{0}'")]
    FilterKeywordNoMatch(String),
    #[error("Duplicate field keyword specified: '{0}'")]
    PaperDuplicateField(String),
    #[error("Required field keywords not given: {0}")]
    PaperMissingFields(String),
    #[error("Specified file path does not exist: '{0}'")]
    PaperPathDoesNotExist(String),
    #[error("Exit reason")]
    ExitReason,
}
