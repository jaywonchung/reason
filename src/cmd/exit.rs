use crate::cmd::prelude::*;

pub static MAN: &'static str = "Usage: exit

Synchronizes the in-memory paper metadata to disk and quits
reason. This is equivalent to pressing <Ctrl-d> in the
command line.
";

pub fn execute(
    _input: CommandInput,
    _state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    Err(Fallacy::ExitReason)
}
