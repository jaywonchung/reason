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
        let chained_cmds: Vec<Vec<_>> = command
            .split('|')
            .map(|cmd| cmd.split_ascii_whitespace().collect())
            .collect();

        // Run the command.
        self.run_command(&chained_cmds).map(|output| output.into())
    }

    /// Split chained commands.
    /// Pipe characters and whitesapces need extra care.
    ///
    /// ```
    /// ls shadowtutor    => [["ls", "shadowtutor"]]
    /// ls 'shadow tutor' => [["ls", "shadow tutor"]]
    /// ls shadow|tutor   => [["ls", "shadow"], ["tutor"]]
    /// ls 'shadow|tutor' => [["ls", "shadow|tutor"]]
    /// ```
    fn parse_command(&self, command: &str) -> Vec<Vec<String>> {
        let mut current_cmd: Vec<String> = vec![String::new()];
        let mut current_piece: usize = 0; // index into `current_cmd`
        let mut parsed_cmds: Vec<Vec<String>> = Vec::new(); // final result

        let mut inside_quotes = false; // whether we're inside single quotes
        let mut command_iter = command.chars().peekable();

        for c in command_iter {
            // Escaped single quote
            if c == '\\' && command_iter.peek() == Some(&'\'') {
                command_iter.next();
                current_cmd[current_piece].push('\'');
            }
            // Unescaped single quote
            else if c == '\'' {
                inside_quotes = !inside_quotes;
            }
            // Pipe
            // If we're inside quotes, this is merely a character part of a regex.
            // Otherwise, this indicates a command chain.
            else if c == '|' {
                if inside_quotes {
                    current_cmd[current_piece].push(c);
                } else {
                    // Wrap up the previous command and start a new one.
                    parsed_cmds.push(vec![String::new()]);
                    std::mem::swap(parsed_cmds.last_mut().unwrap(), &mut current_cmd);
                }
            }
            // Whitespace
            // Advance to next piece.
            else if c.is_whitespace() {
                continue;
            }
            else {
                current_cmd.push(c);
            }
        }
        parsed_cmds.push(current_cmd);
        parsed_cmds
    }

    fn run_command(&mut self, commands: &Vec<Vec<&str>>) -> Result<CommandOutput, Fallacy> {
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
        let mut result = CommandOutput::None;
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
