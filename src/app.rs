use crate::cmd::ls;
use crate::cmd::prelude::*;
use crate::config::Config;
use crate::error::Fallacy;
use crate::paper::Papers;
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

        // ls::execute(CommandInput {
        //     args: None,
        //     pipe: None,
        // })

        // Run the command.
        self.run_command(&chained_cmds)
    }

    fn run_command(&self, commands: &Vec<Vec<&str>>) -> Result<String, Fallacy> {
        // Probably impossible.
        if commands.len() == 0 {
            return Ok(String::new());
        }
        // A single command.
        if commands.len() == 1 {
            // An empty line.
            if commands[0].len() == 0 {
                return Ok(String::new());
            } else {
                // Convert to reason command.
                let rcmd = to_cmd(commands[0][0])?;
                if !rcmd.accepts_args && rcmd.accepts_pipe {
                    return Err(Fallacy::InvalidCommand(
                        "Command only accepts input from pipe.".to_owned(),
                    ));
                }
                // Execute command.
                let input = CommandInput {
                    args: Some(commands[0]),
                    pipe: None,
                };
                return rcmd
                    .execute(input, &mut self.state, &self.config)
                    .map(|output| output.into());
            }
        }
        // A chained command.
        let result: CommandOutput;
        for window in commands.windows(2) {
            let (cmd1, cmd2) = (&window[0], &window[1]);
            if cmd1.len() == 0 || cmd2.len() == 0 {
                return Err(Fallacy::InvalidCommand("Empty command chained.".to_owned()));
            }

            // Check command.
            let (rcmd1, rcmd2) = (to_cmd(cmd1[0])?, to_cmd(cmd2[0])?);
            if rcmd1.outputs_none && (rcmd2.accepts_args || rcmd2.accepts_pipe) {
                return Err(Fallacy::InvalidCommand(format!(
                    "'{}' cannot be chained with '{}' since '{}' outputs none.",
                    cmd1[0], cmd2[0], cmd1[0]
                )));
            }
            if rcmd1.outputs_message && (rcmd2.accepts_args || rcmd2.accepts_pipe) {
                return Err(Fallacy::InvalidCommand(format!(
                    "'{}' cannot be chained with '{}' since '{}' only outputs a message.",
                    cmd1[0], cmd2[0], cmd1[0]
                )));
            }
            if rcmd1.outputs_papers && !rcmd2.accepts_pipe {
                return Err(Fallacy::InvalidCommand(format!(
                    "'{}' cannot be chained with '{}' since '{}' does not accept input from pipe.",
                    cmd1[0], cmd2[0], cmd2[0]
                )));
            }

            // Execute command.
            // TODO: `rm me | ls` works in shells!
        }
        Ok("".to_owned())
    }
}

fn to_cmd(command: &str) -> Result<&ReasonCmd, Fallacy> {
    match command {
        "ls" => Ok(&ls::LS),
        _ => Err(Fallacy::UnknownCommand(command.to_owned())),
    }
}
