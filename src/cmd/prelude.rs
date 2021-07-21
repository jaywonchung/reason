pub use crate::cmd::{CommandInput, CommandOutput, ReasonCmd};
pub use crate::config::Config;
pub use crate::error::Fallacy;
pub use crate::reason_command;
pub use crate::state::State;

#[macro_export]
macro_rules! reason_command {
    ($name:ident: $spec:expr, $exec:ident) => {
        lazy_static::lazy_static! {
            pub static ref $name: ReasonCmd = ReasonCmd::build($spec, $exec);
        }
    };
}
