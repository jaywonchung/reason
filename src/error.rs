use pdf::error::PdfError;
use std::path::PathBuf;

use thiserror::Error;

/// A use of invalid or faulty reason.
#[derive(Error, Debug)]
pub enum Fallacy {
    // Critical errors that terminate the program.
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

    // Non-critical errors that are caught by the main loop.
    // general
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("I/O error: '{0}'")]
    IOError(#[from] std::io::Error),
    #[error("{0}")]
    FailedUserInteraction(String),
    // filter
    #[error("Failed to build filter from regex:\n{0}")]
    FilterBuildFailed(regex::Error),
    // paper
    #[error("Duplicate paper field keyword specified: '{0}'")]
    PaperDuplicateField(String),
    #[error("Required paper fields not given: {0}")]
    PaperMissingFields(String),
    // path
    #[error("Specified file path does not exist: '{0}'")]
    PathDoesNotExist(PathBuf),
    #[error("Invalid UTF-8 character in path: '{0}'")]
    PathInvalidUTF8(PathBuf),
    #[error("Failed to find home directory for user.")]
    Homeless,
    // exit
    #[error("Exit reason")]
    ExitReason,
    // man command
    #[error("`man` accepts exactly one argument.")]
    ManInvalidArgument,
    #[error("Unknown subject: '{0}'")]
    ManUnknownSubject(String),
    // curl command
    #[error("`curl` accepts exactly one argument as source.")]
    CurlNoSource,
    #[error("Unknown source: '{0}'")]
    CurlUnknownSource(String),
    #[error("Invalid source: '{0}'. Refer to `man curl`.")]
    CurlInvalidSourceUrl(String),
    #[error("Failed to parse source url: '{0}'")]
    CurlURLParseError(#[from] url::ParseError),
    #[error("Failed to fetch from url: '{0}'")]
    CurlGetFailed(#[from] reqwest::Error),
    #[error("Failed to parse title. {0}")]
    CurlCannotFindTitle(String),
    #[error("Failed to parse author list. {0}")]
    CurlCannotFindAuthor(String),
    #[error("Failed to parse information from PDF File. {0}")]
    CurlPdfParsingError(#[from] PdfError),
    // printf command
    #[error("Failed to build book: '{0}'")]
    PrintfBuildError(#[from] mdbook::errors::Error),
    // set command
    #[error("No papers given through pipe.")]
    SetNoPapers,
}
