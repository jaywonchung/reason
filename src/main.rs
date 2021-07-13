use crate::app::App;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod app;
mod config;
mod error;
mod paper;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reason = App::new()?;
    let mut editor = Editor::<()>::new();

    loop {
        let readline = editor.readline(">> ");
        match readline {
            Ok(line) => match reason.run(&line) {
                Ok(msg) => print!("{}", msg),
                Err(e) => println!("{}", e),
            },
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
