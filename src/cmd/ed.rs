use std::path::PathBuf;
use std::process::Command;

use crate::cmd::prelude::*;
use crate::utils::confirm;

pub static MAN: &str = include_str!("../../man/ed.md");

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
                _ => panic!("internal ls invocation returned output variant"),
            }
        }
    };

    // Build a vector of note paths.
    let num_papers = selected.len();
    let mut notes = Vec::new();
    for i in selected {
        notes.push(state.papers[i].notepath(config, true)?.unwrap());
    }

    // Ask for confirmation.
    if num_papers > 1 {
        confirm(format!("Open notes for {} paper?", num_papers), true)?;
    }

    // Open notes.
    if config.output.editor_batch {
        spawn(build_editor_command(notes.as_ref(), config), true);
    } else {
        for note in notes {
            spawn(build_editor_command(&[note], config), false);
        }
    }

    Ok(CommandOutput::None)
}

fn spawn(mut command: Command, block: bool) {
    match command.spawn() {
        Ok(mut handle) => {
            if !block {
                return;
            }
            if let Err(e) = handle.wait() {
                println!("Failed to wait subprocess: {}", e);
            }
        }
        Err(e) => {
            if matches!(e.kind(), std::io::ErrorKind::NotFound) {
                println!("Invalid editor command: '{:?}'", e);
            } else {
                println!("Failed to spawn subprocess: '{:?}'", e);
            }
        }
    }
}

fn build_editor_command(notes: &[PathBuf], config: &Config) -> Command {
    let command = &config.output.editor_command;
    let mut ret = Command::new(&command[0]);
    ret.args(&command[1..]).args(notes);
    ret
}
