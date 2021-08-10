use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cmd::prelude::*;
use crate::paper::PaperList;
use crate::utils::confirm;

pub static MAN: &str = include_str!("../../man/open.md");

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
        if let Some(path) = state.papers[ind].filepath(config) {
            files.push((ind, path));
        }
    }

    // Some reports.
    let num_open = files.len();
    if num_papers - num_open > 0 {
        println!(
            "{} {} selected. Skipping {} without file paths.",
            num_papers,
            if num_papers > 1 { "papers" } else { "paper" },
            num_papers - num_open,
        );
    }

    // Ask for confirmation.
    if num_open > 1 {
        confirm(format!("Open {} papers?", num_open), true)?;
    }

    // Open papers.
    if config.output.viewer_batch {
        let (selected, files): (Vec<usize>, Vec<PathBuf>) = files.into_iter().unzip();
        if spawn(build_viewer_command(files.as_ref(), config)) {
            Ok(CommandOutput::Papers(PaperList(selected)))
        } else {
            Ok(CommandOutput::Papers(PaperList(Vec::new())))
        }
    } else {
        let mut selected = Vec::new();
        for (i, file) in files.into_iter() {
            if spawn(build_viewer_command(&[file], config)) {
                selected.push(i);
            }
        }
        Ok(CommandOutput::Papers(PaperList(selected)))
    }
}

fn spawn(mut command: Command) -> bool {
    match command.spawn() {
        Ok(_) => true,
        Err(e) => {
            if matches!(e.kind(), std::io::ErrorKind::NotFound) {
                println!("Invalid editor command: '{:?}'", e);
            } else {
                println!("Failed to spawn subprocess: '{:?}'", e);
            }
            false
        }
    }
}

fn build_viewer_command(files: &[PathBuf], config: &Config) -> Command {
    let mut ret = Command::new(&config.output.viewer_command[0]);
    let mut curly = false;
    for command in &config.output.viewer_command[1..] {
        if command == "{}" {
            ret.args(files);
            curly = true;
        } else {
            ret.arg(command);
        }
    }
    if !curly {
        ret.args(files);
    }
    ret.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    ret
}
