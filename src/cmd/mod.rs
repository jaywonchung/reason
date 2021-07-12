use crate::config::Config;
use crate::state::{State, Papers};
use crate::error::Fallacy;

mod ls;

trait Cmd {
    fn run(args: &Vec<String>, state: &mut State) -> Result<String, String>;
}

/// Runs a command entered by the user and returns a success or error message.
/// The command may mutate the current state object.
pub fn run_command(command: &str, state: &mut State, config: &Config) -> Result<String, Fallacy> {
    // Up to two commands can be chained with pipes.
    let mut chained_cmds: Vec<_> = command.split('|').take(3).collect();
    if chained_cmds.len() >= 3 {
        return Err(Fallacy::ChainTooLongError);
    }

    // First run the filter command.
    let papers: &Papers = ls::run(&chained_cmds[0])?;

    // If we have a chained action command, run it.
    // Otherwise, print the filtered papers.
    if chained_cmds.len() == 2 {
        Ok(dispatch_command(&chained_cmds[1], state, config)?);
    } else {
        Ok(papers.to_string())
    }
}

fn dispatch_command(command: &str, state: &mut State, config: &Config) -> Result<String, Fallacy> {
    Ok(String::new())
}
