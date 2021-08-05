use std::path::PathBuf;

use crate::cmd::prelude::*;
use crate::paper::{Paper, PaperList};
use crate::utils::expand_tilde_string;

pub static MAN: &'static str = "Usage: touch [paper]

Adds a new paper to the paperbase. Papers can be
specified like filters. Differences are:
- The path to the paper file can be specified with the
   keyword '@'.
- Authors should be specified with a single-quoted
   comma-separated string.

Required fields are 'title', 'authors(by)', 'venue(at)',
and 'year(in)'. Just like filters, they don't have to
be in order.

For instance:
```
>> touch 'Reason: A Cool New System' by 'Jae-Won
Chung, Chaehyun Jeong' at OSDI at 2022 as Reason
@ ~/workspace/papers/reason.pdf
```
";

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Parse input to paper metadata.
    let paper = Paper::from_args(input.args)?;

    // Verify file path.
    if let Some(filepath) = &paper.filepath {
        if let Some(base) = &config.storage.file_base_dir {

        }
        let path = expand_tilde_string(&filepath)?;
        if !PathBuf::from(path).exists() {
            return Err(Fallacy::PathDoesNotExist(filepath.to_owned()));
        }
    }

    // Add paper to state.
    state.papers.push(paper);

    Ok(CommandOutput::Papers(PaperList(vec![
        state.papers.len() - 1,
    ])))
}
