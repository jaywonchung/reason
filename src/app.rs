use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::cmd::{parse_command, to_executor, CommandInput, CommandOutput};
use crate::config::Config;
use crate::error::Fallacy;
use crate::state::State;

pub struct App {
    config: Config,
    state: State,
    editor: Editor<()>,
}

impl App {
    /// Initialize a new Reason app.
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // Load reason configuration.
        let config: Config = match home::home_dir() {
            Some(mut p) => {
                p.push(".config/reason/config.toml");
                confy::load_path(p)?
            }
            None => {
                eprintln!("Failed to find your home directory. Using default configuration.");
                Config::default()
            }
        };

        // Load metadata state.
        let state = State::load(&config.state_path)?;

        // Setup readline.
        let builder = rustyline::config::Builder::default();
        let rlconfig = builder
            .max_history_size(config.max_history_size)
            .auto_add_history(true)
            .build();
        let mut editor = Editor::<()>::with_config(rlconfig);

        // Maybe create and load from command history file.
        let history_path = &config.history_path;
        if !history_path.exists() {
            if let Err(e) = std::fs::File::create(history_path) {
                eprintln!(
                    "Failed to create command history file {:?}: {}",
                    history_path, e
                );
            }
        } else {
            if let Err(e) = editor.load_history(history_path) {
                eprintln!(
                    "Failed to load command history from {:?}: {}",
                    history_path, e
                );
            }
        }

        Ok(Self {
            config,
            state,
            editor,
        })
    }

    /// The main command line loop.
    pub fn main_loop(&mut self) -> Result<(), Fallacy> {
        loop {
            let readline = self.editor.readline(">> ");
            match readline {
                Ok(line) => match self.execute(&line) {
                    Ok(msg) => print!("{}", msg),
                    Err(Fallacy::ExitReason) => break,
                    Err(e) => println!("{}", e),
                },
                Err(ReadlineError::Interrupted) => continue,
                Err(ReadlineError::Eof) => break,
                Err(e) => {
                    eprintln!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Teardown the app.
    /// This function only prints errors to stderr and does not fail immediately.
    /// - Save paper metadata state
    /// - Save readline history
    pub fn teardown(&mut self) {
        // Save state to state file.
        if let Err(e) = self.state.store(&self.config.state_path) {
            eprintln!("Error during teardown: {}", e);
        }

        // Save command history to history file.
        let history_path = &self.config.history_path;
        if !history_path.exists() {
            if let Err(e) = std::fs::File::create(history_path) {
                eprintln!(
                    "Error during teardown: {}",
                    Fallacy::HistoryStoreFailed(history_path.to_owned(), e)
                );
                return;
            }
        }
        if let Err(e) = self.editor.save_history(history_path) {
            eprintln!(
                "Error during teardown: {}",
                Fallacy::RLHistoryStoreFailed(history_path.to_owned(), e)
            );
            return;
        }
    }

    /// Runs a command entered by the user and returns a success or error message.
    /// The command may mutate the current state object.
    pub fn execute(&mut self, command: &str) -> Result<String, Fallacy> {
        // Parse the command.
        let commands = parse_command(command)?;

        // Run the command.
        self.run_command(commands)
            .map(|output| output.into_string(&self.state))
    }

    fn run_command(&mut self, mut commands: Vec<Vec<String>>) -> Result<CommandOutput, Fallacy> {
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
                let executor = to_executor(commands[0][0].clone())?;
                let input = CommandInput {
                    args: commands.remove(0),
                    papers: None,
                };
                return executor(input, &mut self.state, &self.config).map(|o| o.into());
            }
        }
        // A chained command.
        let mut output = CommandOutput::None;
        let num_commands = commands.len();
        for (ind, command) in commands.into_iter().enumerate() {
            // The command shouldn't be empty.
            if command.len() == 0 {
                let message: String = if ind == 0 {
                    "Command cannot begin with a pipe.".to_owned()
                } else if ind == num_commands - 1 {
                    "Command cannot end with a pipe.".to_owned()
                } else {
                    "Commands can only be chained with one pipe character.".to_owned()
                };
                return Err(Fallacy::InvalidCommand(message));
            }
            // Run the command.
            // A command is always given arguments. Commands that come after
            // the first one are given papers, but it's up to the command to
            // utilize it.
            let executor = to_executor(command[0].clone())?;
            let input = CommandInput::from_output(command, output);
            output = executor(input, &mut self.state, &self.config)?;
        }
        return Ok(output);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parse_test {
        ($name:ident: $command:expr, $answer:expr) => {
            #[test]
            fn $name() {
                // let app = App::new(Config::default(), State::default());
                let answer: Result<Vec<Vec<&str>>, Fallacy> = $answer;
                let answer: Result<Vec<Vec<String>>, Fallacy> = answer.map(|vec| {
                    vec.iter()
                        .map(|v| v.iter().map(|s| String::from(*s)).collect())
                        .collect()
                });
                let parsed = parse_command($command);
                if answer.is_err() {
                    assert_eq!(
                        parsed.unwrap_err().to_string(),
                        answer.unwrap_err().to_string()
                    );
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
