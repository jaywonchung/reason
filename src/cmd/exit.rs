use crate::cmd::prelude::*;

pub static MAN: &str = include_str!("../../man/exit.md");

pub fn execute(
    _input: CommandInput,
    _state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    Err(Fallacy::ExitReason)
}
