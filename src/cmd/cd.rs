use crate::cmd::prelude::*;
use crate::filter::PaperFilter;

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Input args to paper filter
    let args = input.args;

    PaperFilter::from_args
}
