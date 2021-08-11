use crate::cmd::prelude::*;
use crate::utils::confirm;

pub static MAN: &str = include_str!("../../man/rm.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    let mut paper_list = match input.papers {
        // Papers are given through pipe.
        Some(list) => list,
        // Papers are specified as filter.
        None => {
            match crate::cmd::ls::execute(input, state, config)? {
                CommandOutput::Papers(paper_list) => paper_list,
                // `ls` always returns CommandOutput::Papers.
                _ => panic!(),
            }
        }
    };

    // Ask for confirmation.
    let num_paper = paper_list.0.len();
    if num_paper > 1 {
        confirm(format!("Remove {} papers?", num_paper), false)?;
    }

    // Remove papers.
    paper_list.0.reverse();
    for ind in paper_list.0 {
        state.papers.remove(ind);
    }

    Ok(CommandOutput::Message(format!(
        "Removed {} {}.",
        num_paper,
        if num_paper != 1 { "papers" } else { "paper" },
    )))
}
