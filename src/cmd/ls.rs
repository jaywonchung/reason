use crate::cmd::prelude::*;
use crate::paper::PaperList;
use crate::state::FilterInst;

pub fn execute<'p>(
    input: CommandInput<'p>,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput<'p>, Fallacy> {
    // Convert arguments to a filter
    let filter_inst = FilterInst::from_args(&input.args[1..])?;

    // Filter state + argument filter (without modifying the filter state).
    let filter = state.filters.observe(filter_inst);

    // Filter papers.

    Err(Fallacy::InvalidCommand("no".to_owned()))
}
