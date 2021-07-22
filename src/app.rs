use crate::cmd::prelude::*;
use crate::cmd::to_executor;
use crate::config::Config;
use crate::error::Fallacy;
use crate::state::State;

pub struct App {
    state: State,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self, Fallacy> {
        let config: Config = match confy::load("reason") {
            Ok(c) => c,
            Err(e) => return Err(Fallacy::ConfigLoadFailed(e)),
        };
        let state = State::load(&config.state_path)?;

        Ok(Self { state, config })
    }

    /// Runs a command entered by the user and returns a success or error message.
    /// The command may mutate the current state object.
    pub fn execute(&mut self, command: &str) -> Result<String, Fallacy> {
        // Split chained commands.
        let chained_cmds: Vec<Vec<_>> = command
            .split('|')
            .map(|cmd| cmd.split_ascii_whitespace().collect())
            .collect();

        // Run the command.
        self.run_command(&chained_cmds).map(|output| output.into())
    }

    fn run_command(&self, commands: &Vec<Vec<&str>>) -> Result<CommandOutput, Fallacy> {
        // Probably impossible.
        if commands.len() == 0 {
            return Ok(CommandOutput::None);
        }
        // A single command.
        if commands.len() == 1 {
            // An empty line.
            if commands[0].len() == 0 {
                return Ok(CommandOutput::None);
            } else {
                let executor = to_executor(commands[0][0])?;
                let input = CommandInput {
                    args: Some(&commands[0]),
                    papers: None,
                };
                return executor(input, &mut self.state, &self.config).map(|o| o.into());
            }
        }
        // A chained command.
        let result: CommandOutput;
        for (ind, command) in commands.iter().enumerate() {
            // The command shouldn't be empty.
            if command.len() == 0 {
                let message: String = if ind == 0 {
                    "Command cannot begin with a pipe.".to_owned()
                } else if ind == commands.len() - 1 {
                    "Command cannot end with a pipe.".to_owned()
                } else {
                    "Commands can only be chained with one pipe character.".to_owned()
                };
                return Err(Fallacy::InvalidCommand(message));
            }
            // Run the command.
            let executor = to_executor(command[0])?;
            let input = if ind == 0 {
                CommandInput {
                    args: Some(command),
                    papers: None,
                }
            } else {
                result.into()
            };
            result = executor(input, &mut self.state, &self.config)?;
        }
        return Ok(result);
    }
}
