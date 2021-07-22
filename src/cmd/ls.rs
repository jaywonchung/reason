use crate::cmd::prelude::*;

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    let args = input
        .args
        .expect("Failed to unwrap args from CommandInput.");
}
