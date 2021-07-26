use crate::cmd::prelude::*;
use crate::paper::{PaperFilter, PaperFilterPieceBuilder};

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Input to filter
    let args = input
        .args
        .expect("Failed to unwrap args from CommandInput.");

    Err(Fallacy::InvalidCommand(String::from("no")))
}

fn input_to_filter(input: &CommandInput) -> Result<PaperFilter, Fallacy> {
    let args = match input.args {
        None => return Ok(PaperFilter::default()),
        Some(args) => args,
    };

    // Parse arguments into a PaperFilter.
    // The basic structure is `(keyword filter)*`. Filters for titles are without keywords.
    // The same keyword may appear multiple times. Those filters are and'ed.
    let keyword: String;
    let found_keyword = false;
    for &arg in args {
        // The current argument should be a regex.
        if found_keyword {
        } else {
        }
    }

    Ok(PaperFilter::default())
}
