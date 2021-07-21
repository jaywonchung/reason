use crate::cmd::prelude::*;

reason_command!(LS: "args -> papers", execute);

fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    let args = input
        .args
        .expect("Failed to unwrap args from CommandInput.");
}
