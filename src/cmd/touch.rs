use crate::cmd::prelude::*;
use crate::paper::{Paper, PaperList};

pub static MAN: &str = include_str!("../../man/touch.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Parse input to paper metadata.
    let paper = Paper::from_args(input.args)?;

    // Verify file path.
    if let Some(filepath) = paper.filepath(config) {
        if !filepath.exists() {
            return Err(Fallacy::PathDoesNotExist(filepath));
        }
    }

    // Add paper to state.
    state.papers.push(paper);

    Ok(CommandOutput::Papers(PaperList(vec![
        state.papers.len() - 1,
    ])))
}
