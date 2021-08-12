use crate::cmd::prelude::*;

pub static MAN: &str = include_str!("../../man/set.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // We need papers from pipe.
    if input.papers.is_none() {
        return Err(Fallacy::SetNoPapers);
    }

    // Apply changes.
    for &ind in input.papers.as_ref().unwrap().0.iter() {
        state.papers[ind].apply_from_args(input.args.as_ref())?;
    }

    Ok(CommandOutput::Papers(input.papers.unwrap()))
}
