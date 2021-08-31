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
        confirm(
            format!("Remove {} papers, including files and notes?", num_paper),
            false,
        )?;
    }

    // Remove files and notes.
    let mut errors = Vec::new();
    for &ind in paper_list.0.iter() {
        if let Some(filepath) = state.papers[ind].filepath(config) {
            if let Err(e) = std::fs::remove_file(&filepath) {
                errors.push(e);
            }
        }
        if let Some(notepath) = state.papers[ind].notepath(config, false)? {
            if let Err(e) = std::fs::remove_file(&notepath) {
                errors.push(e);
            }
        }
    }

    // Remove papers.
    paper_list.0.reverse();
    for ind in paper_list.0 {
        state.papers.remove(ind);
    }

    // Print errors.
    println!("Errors occured while deleting files and notes:");
    for e in errors {
        println!("{}", e);
    }

    Ok(CommandOutput::Message(format!(
        "Removed {} {} from the paperbase.",
        num_paper,
        if num_paper != 1 { "papers" } else { "paper" },
    )))
}
