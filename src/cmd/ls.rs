use crate::cmd::prelude::*;
use crate::paper::PaperList;
use crate::state::FilterInst;

pub fn execute(
    input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Convert arguments to a filter
    let filter_inst = FilterInst::from_args(&input.args[1..], false)?;

    // Filter state + argument filter (without modifying the filter state).
    let filter = state.filters.observe(filter_inst);

    // Filter papers.
    let mut selected = Vec::new();
    for (ind, paper) in state.papers.iter().enumerate() {
        if filter.matches(paper) {
            selected.push(ind);
        }
    }

    Ok(CommandOutput::Papers(PaperList { selected }))
}
