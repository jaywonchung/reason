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
    pub fn run(&mut self, command: &str) -> Result<String, Fallacy> {
        // Split chained commands.
        let chained_cmds: Vec<_> = command.split('|').take(3).collect();

        // Check if the command chain is legal.
        let (legal, msg) = self.check_command(&chained_cmds);
        if !legal {
            return Err(Fallacy::InvalidChain(msg));
        }

        // Run the command.
        self.run_command(chained_cmds)
    }

    fn check_command(&self, commands: &Vec<&str>) -> (bool, String) {
        (true, String::new())
    }

    fn run_command(&mut self, commands: Vec<&str>) -> Result<String, Fallacy> {
        Ok(String::new())
    }
}
