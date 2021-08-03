use std::path::PathBuf;

use crate::cmd::prelude::*;
use crate::paper::{Paper, PaperList};

pub static MAN: &'static str = "Usage: touch [paper]

Adds a new paper to the paperbase. Papers can be
specified like filters, except that the path to the
paper file can be specified with the keyword '@'.

Required fields are 'title', 'authors(by)', 'venue(at)',
and 'year(in)'.
";

pub fn execute(
    input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Parse input to paper metadata.
    let paper = Paper::from_args(input.args)?;

    // Verify file path.
    if let Some(filepath) = &paper.filepath {
        if !PathBuf::from(filepath).exists() {
            return Err(Fallacy::PaperPathDoesNotExist(filepath.to_owned()));
        }
    }

    // Add paper to state.
    state.papers.push(paper);

    Ok(CommandOutput::Papers(PaperList {
        selected: vec![state.papers.len() - 1],
    }))
}
