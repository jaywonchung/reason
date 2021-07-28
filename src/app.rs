use crate::cmd::prelude::*;
use crate::cmd::to_executor;
use crate::config::Config;
use crate::error::Fallacy;
use crate::state::State;

pub struct App {
    config: Config,
    state: State,
}

impl App {
    pub fn new(config: Config, state: State) -> Self {
        Self { config, state }
    }

    /// Runs a command entered by the user and returns a success or error message.
    /// The command may mutate the current state object.
    pub fn execute(&mut self, command: &str) -> Result<String, Fallacy> {
        // Parse the command.
        let commands = self.parse_command(command)?;

        // Run the command.
        self.run_command(&commands).map(|output| output.into())
    }

    /// Split chained commands.
    ///
    /// Reason implements its own command line parser.
    /// By default arguments are delimited with whitespace, but they can be chunked
    /// by grouping them in single quotes.
    /// Literal single quotes can be entered by escaping them with a backslash.
    ///
    /// Multiple commands can be piped with the pipe(`|`) character.
    fn parse_command(&self, command: &str) -> Result<Vec<Vec<String>>, Fallacy> {
        let mut current_piece: String = String::new();
        let mut current_cmd: Vec<String> = Vec::new();
        let mut parsed_cmds: Vec<Vec<String>> = Vec::new(); // final result

        let mut inside_quotes = false; // whether we're inside single quotes
        let mut command_iter = command.chars().peekable();

        // A helper closure that consumes all whitespaces from an char peekable iterator.
        let consume_whitespace = |iter: &mut std::iter::Peekable<core::str::Chars>| {
            while let Some(c) = iter.peek() {
                if c.is_whitespace() {
                    iter.next();
                } else {
                    break;
                }
            }
        };

        // Trim whitespace in the beginning.
        consume_whitespace(&mut command_iter);

        // Parse commands.
        while let Some(c) = command_iter.next() {
            // Escaped single quote
            if c == '\\' && command_iter.peek() == Some(&'\'') {
                command_iter.next();
                current_piece.push('\'');
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
                    current_piece.push(c);
                } else {
                    // Wrap up the previous command and start a new one.
                    if current_piece.len() != 0 {
                        current_cmd.push(String::new());
                        std::mem::swap(current_cmd.last_mut().unwrap(), &mut current_piece);
                    }
                    // Pipe encountered when `current_cmd` is empty: double pipes!
                    if current_cmd.len() == 0 {
                        return Err(Fallacy::InvalidCommand("Invalid use of pipes.".to_owned()));
                    }
                    parsed_cmds.push(Vec::new());
                    std::mem::swap(parsed_cmds.last_mut().unwrap(), &mut current_cmd);
                }
            }
            // Whitespace
            // If we're inside quotes, this is merely a character part of a regex.
            // Otherwise, this indicates the end of the current piece.
            else if c.is_whitespace() {
                if inside_quotes {
                    current_piece.push(c);
                } else {
                    // Consume all whitespaces that follow.
                    consume_whitespace(&mut command_iter);

                    // Wrap up the previous piece and start a new one.
                    if current_piece.len() != 0 {
                        current_cmd.push(String::new());
                        std::mem::swap(current_cmd.last_mut().unwrap(), &mut current_piece);
                    }
                }
            }
            // Everything else.
            else {
                current_piece.push(c);
            }
        }
        // No need to push empty pieces.
        if current_piece.len() != 0 {
            current_cmd.push(current_piece);
        }
        // Command ended with a pipe.
        if current_cmd.len() == 0 && parsed_cmds.len() != 0 {
            return Err(Fallacy::InvalidCommand("Command ends with a dangling pipe.".to_owned()));
        }
        parsed_cmds.push(current_cmd);
        Ok(parsed_cmds)
    }

    fn run_command(&mut self, commands: &Vec<Vec<String>>) -> Result<CommandOutput, Fallacy> {
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
                let executor = to_executor(&commands[0][0])?;
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
            let executor = to_executor(&command[0])?;
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parse_test {
        ($name:ident: $command:expr, $answer:expr) => {
            #[test]
            fn $name() {
                let app = App::new(Config::default(), State::default());
                let answer: Result<Vec<Vec<&str>>, Fallacy> = $answer;
                let answer: Result<Vec<Vec<String>>, Fallacy> = answer.map(|vec| vec.iter().map(|v| v.iter().map(|s| String::from(*s)).collect()).collect());
                let parsed = app.parse_command($command);
                if answer.is_err() {
                    assert_eq!(parsed.unwrap_err().to_string(), answer.unwrap_err().to_string());
                } else {
                    assert_eq!(parsed.unwrap(), answer.unwrap());
                }
            }
        };
    }

    // Correct commands
    parse_test!(normal_single:
        "ls shadowtutor",
        Ok(vec![vec!["ls", "shadowtutor"]])
    );
    parse_test!(normal_many:
        "ls   shadowtutor by  	Chung  ",
        Ok(vec![vec!["ls", "shadowtutor", "by", "Chung"]])
    );
    parse_test!(pipe_single:
        "ls shadowtutor | printf",
        Ok(vec![vec!["ls", "shadowtutor"], vec!["printf"]])
    );
    parse_test!(pipe_many:
        "ls shadow|tutor by| Chung on icpp |2020 ",
        Ok(vec![vec!["ls", "shadow"], vec!["tutor", "by"], vec!["Chung", "on", "icpp"], vec!["2020"]])
    );
    parse_test!(quote_whitespace:
        "ls 'shadow tutor'",
        Ok(vec![vec!["ls", "shadow tutor"]])
    );
    parse_test!(quote_pipe:
        "ls 'shadow|tutor'",
        Ok(vec![vec!["ls", "shadow|tutor"]])
    );
    parse_test!(all_in_one:
        r"  ls  ' shadow| tutor\'' | 'printf ' 	 this\' paper  ",
        Ok(vec![vec!["ls", " shadow| tutor'"], vec!["printf ", "this'", "paper"]])
    );
    parse_test!(empty:
        "",
        Ok(vec![vec![]])
    );

    // Wrong commands
    parse_test!(double_pipe:
        "ls shadowtutor || printf",
        Err(Fallacy::InvalidCommand("Invalid use of pipes.".to_owned()))
    );
    parse_test!(ends_with_pipe1:
        "ls shadowtutor|",
        Err(Fallacy::InvalidCommand("Command ends with a dangling pipe.".to_owned()))
    );
    parse_test!(ends_with_pipe2:
        "ls shadowtutor | ",
        Err(Fallacy::InvalidCommand("Command ends with a dangling pipe.".to_owned()))
    );
    parse_test!(starts_with_pipe1:
        "|ls shadowtutor",
        Err(Fallacy::InvalidCommand("Invalid use of pipes.".to_owned()))
    );
    parse_test!(starts_with_pipe2:
        "|  ls shadowtutor",
        Err(Fallacy::InvalidCommand("Invalid use of pipes.".to_owned()))
    );
}
