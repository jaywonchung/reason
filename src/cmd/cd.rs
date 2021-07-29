use crate::cmd::prelude::*;
use crate::filter::PaperFilter;
use crate::state::FilterInst;

pub fn execute(
    input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Convert arguments to a filter
    let args = input.args;
    let filter_inst = if args.len() <= 1 {
        FilterInst::Reset
    } else if args.len() == 2 {
        match args[1].as_ref() {
            "." => FilterInst::Here,
            ".." => FilterInst::Parent,
            "-" => FilterInst::Prev,
            _ => FilterInst::Add(PaperFilter::from_args(&args[1..2])?),
        }
    } else {
        FilterInst::Add(PaperFilter::from_args(&args[1..])?)
    };

    state.filters.record(filter_inst);

    Ok(CommandOutput::None)
}
