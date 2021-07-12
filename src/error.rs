use thiserror::Error;

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

    // Non-critical errors
    #[error("Invalid filter: '{0}'")]
    InvalidFilter(regex::Error),
    #[error("No more than two commands can be chained.")]
    ChainTooLongError,
}
