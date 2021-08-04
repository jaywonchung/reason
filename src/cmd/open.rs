use std::process::Command;

use crate::cmd::prelude::*;
use crate::utils::{confirm, expand_tilde_string};

pub static MAN: &'static str = "Usage:
1) alone: open [filter]
2) pipe:  [paper list] | open

The `open` command opens papers with a viewer program.
You may configure the viewer to use by setting the
`display.viewer_binary_path` entry in your config file.

When a paper list is given to `open` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `open` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | open` is equivalent to just `open`.

See `man command` for more on which commands output
paper lists.
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
    let num_papers = paper_list.selected.len();
    let files: Vec<_> = paper_list
        .selected
        .into_iter()
        .filter_map(|i| state.papers[i].filepath.as_ref())
        .collect();

    // Some reports.
    let num_open = files.len();
    println!(
        "Skipping papers without filepaths ({} out of {}).",
        num_papers - num_open,
        num_papers
    );

    // Ask for confirmation.
    if num_open > 1 {
        confirm(format!("Open {} papers?", num_open), true)?;
    }

    // Open papers.
    for file in files {
        let file = expand_tilde_string(&file)?;
        if let Err(e) = Command::new(&config.display.viewer_binary_path)
            .arg(&file)
            .spawn()
        {
            println!("Failed to open {}: {}", file, e);
        }
    }

    Ok(CommandOutput::None)
}
