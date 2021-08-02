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
        let mut config: Config = match home::home_dir() {
            Some(mut p) => {
                p.push(".config/reason/config.toml");
                match confy::load_path(p) {
                    Ok(config) => config,
                    Err(e) => return Err(e.into()),
                }
            }
            None => {
                eprintln!("Failed to find your home directory. Using default configuration.");
                Config::default()
            }
        };

        // Check and fix the contents of the config.
        config.audit()?;

        // Load metadata state.
        let state = State::load(&config.storage.paper_metadata)?;

        // Setup readline.
        let builder = rustyline::config::Builder::default();
        let rlconfig = builder
            .max_history_size(config.storage.max_history_size)
            .auto_add_history(true)
            .build();
        let mut editor = Editor::<()>::with_config(rlconfig);

        // Maybe create and load from command history file.
        let history_path = &config.storage.command_history;
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
        if let Err(e) = self.state.store(&self.config.storage.paper_metadata) {
            eprintln!("Error during teardown: {}", e);
        }

        // Save command history to history file.
        let history_path = &self.config.storage.command_history;
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
            .map(|output| output.into_string(&self.state, &self.config))
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
                // Skip comments.
                if commands[0][0] == "#" {
                    return Ok(CommandOutput::None);
                }
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
        for command in commands.into_iter() {
            // The command shouldn't be empty.
            if command.len() == 0 {
                return Err(Fallacy::InvalidCommand(
                    "Command cannot be empty.".to_owned(),
                ));
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
