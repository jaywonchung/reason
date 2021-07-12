use rustyline::error::ReadlineError;
use rustyline::Editor;

mod cmd;
mod config;
mod error;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: config::Config = confy::load("reason")?;
    let mut state = state::State::load(&config.state_path)?;
    let mut editor = Editor::<()>::new();
    loop {
        let readline = editor.readline(">> ");
        match readline {
            Ok(line) => {
                match cmd::run_command(&line, &mut state, &config) {
                    Ok(msg) => print!("{}", msg),
                    Err(e) => println!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
    Ok(())
}
