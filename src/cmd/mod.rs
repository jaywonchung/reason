use std::fmt;

use crate::config::Config;
use crate::error::Fallacy;
use crate::paper::Papers;
use crate::state::State;

pub mod prelude;
pub mod cd;
pub mod ls;

pub type ExecuteFn = fn(CommandInput, &mut State, &Config) -> Result<CommandOutput, Fallacy>;

pub struct CommandInput<'a> {
    pub args: &'a Vec<String>,
    pub papers: Option<Papers>,
}

pub enum CommandOutput {
    None,
    Papers(Papers),
    Message(String),
}

impl<'a> CommandInput<'a> {
    pub fn from_output(args: &'a Vec<String>, output: CommandOutput) -> Self {
        let papers = match output {
            CommandOutput::None => None,
            CommandOutput::Message(_) => None,
            CommandOutput::Papers(p) => Some(p),
        };
        Self { args, papers }
    }
}

impl fmt::Display for CommandOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            CommandOutput::None => "",
            CommandOutput::Message(s) => s,
            CommandOutput::Papers(p) => &p.to_string(),
        };

        write!(f, "{}", output)
    }
}

pub fn to_executor(command: &str) -> Result<ExecuteFn, Fallacy> {
    match command {
        "ls" => Ok(ls::execute),
        _ => Err(Fallacy::UnknownCommand(command.to_owned())),
    }
}
