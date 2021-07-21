use crate::config::Config;
use crate::error::Fallacy;
use crate::paper::Papers;
use crate::state::State;

pub mod ls;
pub mod prelude;

pub type ExecuteFn = fn(CommandInput, &mut State, &Config) -> Result<CommandOutput, Fallacy>;

pub struct ReasonCmd {
    pub accepts_pipe: bool,
    pub accepts_args: bool,
    pub outputs_none: bool,
    pub outputs_papers: bool,
    pub outputs_message: bool,
    pub execute: ExecuteFn,
}

impl Default for ReasonCmd {
    fn default() -> Self {
        Self {
            accepts_pipe: false,
            accepts_args: false,
            outputs_none: false,
            outputs_papers: false,
            outputs_message: false,
            execute: noop,
        }
    }
}

impl ReasonCmd {
    fn build(inout: &str, execute: ExecuteFn) -> Self {
        let mut command = Self::default();

        // Parse and set input/output types
        // e.g. inout == "args|pipe -> papers"
        let (input, output) = inout
            .split_once(" -> ")
            .expect("Command inout format should contain \" -> \".");
        for input_type in input.split('|') {
            match input_type {
                "pipe" => command.accepts_args = true,
                "args" => command.accepts_args = true,
                _ => panic!("Wrong input type format: {}", input_type),
            }
        }
        for output_type in output.split('|') {
            match output_type {
                "none" => command.outputs_none = true,
                "paper" | "papers" => command.outputs_papers = true,
                "message" | "messages" => command.outputs_message = true,
                _ => panic!("Wrong output type format: {}", output_type),
            }
        }

        command.execute = execute;
        command
    }

    pub fn execute(
        &self,
        input: CommandInput,
        state: &mut State,
        config: &Config,
    ) -> Result<CommandOutput, Fallacy> {
        (self.execute)(input, state, config)
    }
}

pub struct CommandInput<'a> {
    args: Option<Vec<&'a str>>,
    pipe: Option<Papers>,
}

pub enum CommandOutput {
    None,
    Papers(Papers),
    Message(String),
}

impl From<CommandOutput> for CommandInput<'_> {
    fn from(output: CommandOutput) -> Self {
        match output {
            CommandOutput::None => panic!("Cannot pipe none"),
            CommandOutput::Message(_) => panic!("Cannot pipe message"),
            CommandOutput::Papers(p) => Self {
                args: None,
                pipe: Some(p),
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

fn noop(_: CommandInput, _: &mut State, _: &Config) -> Result<CommandOutput, Fallacy> {
    Ok(CommandOutput::None)
}
