use crate::config::Config;
use crate::error::Fallacy;
use crate::paper::Papers;
use crate::state::State;

pub mod ls;
pub mod prelude;

pub type ExecuteFn = fn(CommandInput, &mut State, &Config) -> Result<CommandOutput, Fallacy>;

#[derive(Default)]
pub struct CommandInput<'a> {
    pub args: Option<&'a Vec<&'a str>>,
    pub papers: Option<Papers>,
}

pub enum CommandOutput {
    None,
    Papers(Papers),
    Message(String),
}

impl From<CommandOutput> for CommandInput<'_> {
    fn from(output: CommandOutput) -> Self {
        match output {
            CommandOutput::None => Self::default(),
            CommandOutput::Message(_) => Self::default(),
            CommandOutput::Papers(p) => Self {
                args: None,
                papers: Some(p),
            },
        }
    }
}

impl From<CommandOutput> for String {
    fn from(output: CommandOutput) -> Self {
        match output {
            CommandOutput::None => String::new(),
            CommandOutput::Message(s) => s,
            CommandOutput::Papers(p) => p.to_string(),
        }
    }
}

pub fn to_executor(command: &str) -> Result<ExecuteFn, Fallacy> {
    match command {
        "ls" => Ok(ls::execute),
        _ => Err(Fallacy::UnknownCommand(command.to_owned())),
    }
}
