use crate::cmd::prelude::*;

pub static MAN: &str = include_str!("../../man/pwd.md");

pub fn execute(
    _input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    Ok(CommandOutput::Message(state.filters.current().to_string()))
}
