use crate::cmd::prelude::*;

pub static MAN: &'static str = "Usage: pwd

Print the current default filter set by `cd`.
";

pub fn execute(
    _input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    Ok(CommandOutput::Message(state.filters.current().to_string()))
}
