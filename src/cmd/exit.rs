use crate::cmd::prelude::*;

pub fn execute<'p>(
    _input: CommandInput<'p>,
    _state: &mut State,
    _config: &Config,
) -> Result<CommandOutput<'p>, Fallacy> {
    Err(Fallacy::ExitReason)
}
