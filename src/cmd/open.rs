use std::process::Command;

use crate::cmd::prelude::*;
use crate::utils::{confirm, expand_tilde_string};

pub static MAN: &'static str = "Usage:
1) alone: open [filter]
2) pipe:  ls [filter] | open, touch [paper] | open

The `open` command opens papers with a viewer program.
You may configure the viewer to use by setting the
`display.viewer_binary_path` entry in your config file.

When a paper list is given to `open` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `open` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | open` is equivalent to just `open`.
";

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

    // Build a vector of file paths.
    let files: Vec<_> = paper_list
        .selected
        .into_iter()
        .filter_map(|i| state.papers[i].filepath.as_ref())
        .collect();

    // Ask for confirmation.
    if files.len() > 1 {
        confirm(format!("Open {} papers?", files.len()), true)?;
    }

    // Open papers.
    for file in files {
        let file = expand_tilde_string(&file)?;
        Command::new(&config.display.viewer_binary_path)
            .arg(file)
            .spawn();
    }

    Ok(CommandOutput::None)
}
