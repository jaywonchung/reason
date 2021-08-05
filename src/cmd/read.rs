use crate::cmd::prelude::*;

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
    _state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    Ok(CommandOutput::None)
}
