use std::path::PathBuf;

use crate::cmd::prelude::*;
use crate::paper::{Paper, PaperList};

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
