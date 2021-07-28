use rustyline::error::ReadlineError;
use rustyline::Editor;

mod cmd;
mod state;
mod app;
mod config;
mod error;
mod filter;
mod paper;

use crate::app::App;
use crate::config::Config;
use crate::state::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize app.
    let config = Config::load()?;
    let state = State::load(&config.state_path)?;
    let mut reason = App::new(config, state);

    // Setup readline.
    let mut editor = Editor::<()>::new();

    // Start main loop.
    loop {
        let readline = editor.readline(">> ");
        match readline {
            Ok(line) => match reason.execute(&line) {
                Ok(msg) => print!("{}", msg),
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
