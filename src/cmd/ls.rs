use crate::cmd::prelude::*;
use crate::paper::Papers;
use crate::state::FilterInst;

pub fn execute<'p>(
    input: CommandInput,
    state: &'p mut State,
    config: &Config,
) -> Result<CommandOutput<'p>, Fallacy> {
    // Convert arguments to a filter
    let filter_inst = FilterInst::from_args(&input.args[1..])?;

    // Filter state + argument filter (without modifying the filter state).
    let filter = state.filters.observe(filter_inst);

    // Filter papers.
    let papers = match input.papers {
        Some(p) => p,
        None => Papers(state.papers.iter().collect()),
    };

    let result = Vec::new();
    for paper in papers.0 {
        if filter.matches(paper) {
            result.push(paper);
        }
    }

    Err(Fallacy::InvalidCommand("no".to_owned()))
}
