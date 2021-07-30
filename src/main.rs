mod app;
mod cmd;
mod config;
mod error;
mod filter;
mod paper;
mod state;

use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Startup Reason.
    let mut reason = App::startup()?;

    // Run the main loop.
    // Errors will not terminate the program. We want to run the teardown logic.
    if let Err(e) = reason.main_loop() {
        eprintln!("Error during main loop: {}\nRunning teardown ...", e);
    }

    // Teardown Reason.
    reason.teardown();

    Ok(())
}
