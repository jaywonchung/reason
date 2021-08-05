use crate::cmd::prelude::*;
use crate::paper::PaperList;
use crate::state::FilterInst;

pub static MAN: &'static str = "Usage: ls [filter]

Filter papers in the paperbase and print them in a
pretty table.

See `man filter` for more on filters.
";

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Convert arguments to a filter
    let filter_inst = FilterInst::from_args(
        &input.args[1..],
        false,
        config.filter.case_insensitive_regex,
    )?;

    // Filter state + argument filter (without modifying the filter state).
    let filter = state.filters.observe(filter_inst);

    // Filter papers.
    let mut selected = Vec::new();
    // Shortcut path for listing all papers.
    if filter.is_empty() {
        selected = (0..state.papers.len()).collect();
    }
    // Our filter is not empty.
    else {
        for (ind, paper) in state.papers.iter().enumerate() {
            if filter.matches(paper) {
                selected.push(ind);
            }
        }
    }

    Ok(CommandOutput::Papers(PaperList(selected)))
}
