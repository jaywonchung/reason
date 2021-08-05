use std::process::Command;

use crate::cmd::prelude::*;
use crate::utils::confirm;

pub static MAN: &'static str = "Usage:
1) alone: read [filter]
2) pipe:  [paper list] | read

Open paper notes with a text editor and outputs
the papers of successfully notes in the usual table format.
You may configure the editor to use by setting the
`output.editor_binary_path` entry in your config file.

When a paper list is given to `read` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `read` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | read` is equivalent to just `read`.

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

    // Build a vector of note paths.
    let num_papers = selected.len();
    let notes: Vec<_> = selected
        .iter()
        .map(|&i| state.papers[i].note_path())
        .collect();

    // Ask for confirmation.
    if num_papers > 1 {
        confirm(format!("Open notes for {} paper?", num_papers), true)?;
    }

    // Open notes.
    let mut handle = Command::new(&config.output.editor_binary_path).arg("-p").args(&notes[..]).spawn().unwrap();
    println!("{:?}", handle.wait());

    Ok(CommandOutput::None)
}
