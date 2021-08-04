use crate::cmd::prelude::*;
use crate::utils::confirm;

pub static MAN: &'static str = "Usage:
1) alone: rm [filter]
2) pipe:  [paper list] | rm

Remove papers from the paperbase.

When a paper list is given to `rm` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `rm` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | rm` is equivalent to just `rm`.
";

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
    let num_paper = paper_list.selected.len();
    if num_paper > 1 {
        confirm(format!("Remove {} papers?", num_paper), false)?;
    }

    // Remove papers.
    paper_list.selected.reverse();
    for ind in paper_list.selected {
        state.papers.remove(ind);
    }

    Ok(CommandOutput::Message(format!("Removed {} papers.", num_paper)))
}
