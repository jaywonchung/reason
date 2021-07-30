use crate::cmd::prelude::*;

pub fn execute<'p>(
    _input: CommandInput<'p>,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput<'p>, Fallacy> {
    Ok(CommandOutput::Message(state.filters.current().to_string()))
}
