use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cmd::prelude::*;
use crate::paper::PaperList;
use crate::utils::{confirm, expand_tilde_string};

pub static MAN: &'static str = "Usage:
1) alone: open [filter]
2) pipe:  [paper list] | open

Open papers with a viewer program and outputs
successfully opened papers in the usual table format.
You may configure the viewer to use by setting the
`output.viewer_command` entry in your config file.

When a paper list is given to `open` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `open` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | open` is equivalent to just `open`.

The following might come in handy:
```
ls as Reason | open | read
```
";

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Build paper list from input.
    let selected = match input.papers {
        // Papers are given through pipe.
        Some(list) => list.0,
        // Papers are specified as filter.
        None => {
            match crate::cmd::ls::execute(input, state, config)? {
                CommandOutput::Papers(paper_list) => paper_list.0,
                // `ls` always returns CommandOutput::Papers.
                _ => panic!(),
            }
        }
    };

    // Build a vector of file paths.
    let num_papers = selected.len();
    let selected: Vec<_> = selected
        .into_iter()
        .filter(|&i| state.papers[i].filepath.is_some())
        .collect();
    let mut files = Vec::new();
    for &ind in selected.iter() {
        if let Some(path) = state.papers[ind].abs_filepath(config)? {
            files.push((ind, path));
        }
    }

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
    let mut selected = Vec::new();
    if config.output.viewer_batch {
        // let command = build_viewer_command(files, config);
        // spawn(command, // TODO: handle batched.
    } else {
        for (i, file) in files.into_iter() {
            let command = build_viewer_command(&[file], config);
            spawn(command, i, &mut selected);
        }
    }

    Ok(CommandOutput::Papers(PaperList(selected)))
}

fn spawn(mut command: Command, i: usize, selected: &mut Vec<usize>) {
    match command.spawn() {
        Ok(_) => selected.push(i),
        Err(e) => {
            if matches!(e.kind(), std::io::ErrorKind::NotFound) {
                println!("Invalid editor command: '{:?}'", e);
            } else {
                println!("Failed to spawn subprocess: '{:?}'", e);
            }
        }
    }
}

fn build_viewer_command(files: &[PathBuf], config: &Config) -> Command {
    let command = &config.output.viewer_command;
    let mut ret = Command::new(&command[0]);
    ret.args(&command[1..]).args(files);
    ret.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    ret
}
