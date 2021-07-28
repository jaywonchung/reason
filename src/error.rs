use thiserror::Error;

use crate::filter::PaperFilterPieceBuilderError;

/// A use of invalid or faulty reason.
#[derive(Error, Debug)]
pub enum Fallacy {
    // Critical errors
    #[error("Failed to load paper metadata from '{0}': '{1}'")]
    StateLoadFailed(String, std::io::Error),
    #[error("Failed to parse paper metadata loaded from '{0}': '{1}'")]
    StateDeserializeFailed(String, serde_yaml::Error),
    #[error("Failed to store paper metadata to '{0}': '{1}'")]
    StateStoreFailed(String, std::io::Error),
    #[error("Failed to serialize and store paper metadata to '{0}': '{1}'")]
    StateSerializeFailed(String, serde_yaml::Error),
    #[error("Failed to load from config: '{0}'")]
    ConfigLoadFailed(std::io::Error),

    // Non-critical errors
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),
    #[error("Invalid filter: '{0}'")]
    InvalidFilter(regex::Error),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Failed to build filter from regex: '{0}'")]
    FilterBuildFailed(#[from] PaperFilterPieceBuilderError),
    #[error("No matching regex for keyword '{0}'")]
    FilterKeywordNoMatch(String),
}
