use crate::cmd::prelude::*;
use crate::state::FilterInst;

pub fn execute(
    input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Convert arguments to a filter.
    let filter_inst = FilterInst::from_args(&input.args[1..], true)?;

    // Record the filter instruction.
    state.filters.record(filter_inst);

    Ok(CommandOutput::None)
}