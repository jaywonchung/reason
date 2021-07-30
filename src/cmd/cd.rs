use crate::cmd::prelude::*;
use crate::state::FilterInst;

pub fn execute<'p>(
    input: CommandInput,
    state: &'p mut State,
    _config: &Config,
) -> Result<CommandOutput<'p>, Fallacy> {
    // Convert arguments to a filter.
    let filter_inst = FilterInst::from_args(&input.args[1..])?;

    // Record the filter instruction.
    state.filters.record(filter_inst);

    Ok(CommandOutput::None)
}
