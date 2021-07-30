use crate::cmd::prelude::*;

pub fn execute(
    _input: CommandInput,
    _state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    Err(Fallacy::ExitReason)
}
