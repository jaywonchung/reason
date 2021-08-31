mod app;
mod cmd;
mod config;
mod error;
mod filter;
mod paper;
mod state;
mod utils;

use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Handle command line arguments.
    let args = std::env::args();
    for arg in args {
        if arg == "--help" || arg == "-h" {
            println!("Reason: A Shell for Research Papers");
            println!("Start reason and run `man man` for the top-level documentation.");
            std::process::exit(0);
        }
        if arg == "--version" || arg == "-v" {
            println!("Reason v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
    }

    // Startup Reason.
    let mut reason = App::init()?;

    // Run the main loop.
    // Errors will not terminate the program. We want to run the teardown logic.
    if let Err(e) = reason.main_loop() {
        eprintln!("Error during main loop: {}\nRunning teardown ...", e);
    }

    // Terminate Reason.
    reason.terminate();

    Ok(())
}
