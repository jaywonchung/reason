use crate::cmd::prelude::*;

pub static MAN: &str = include_str!("../../man/wc.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    let paper_list = match input.papers {
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

    Ok(CommandOutput::Message(format!("{} papers.\n", paper_list.0.len())))
}
