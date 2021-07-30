use crate::cmd::prelude::*;

pub fn execute<'p>(
    _input: CommandInput,
    state: &'p mut State,
    _config: &Config,
) -> Result<CommandOutput<'p>, Fallacy> {
    Ok(CommandOutput::Message(state.filters.current().to_string()))
}
