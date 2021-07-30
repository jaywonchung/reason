use crate::config::Config;
use crate::error::Fallacy;
use crate::paper::PaperList;
use crate::state::State;

mod cd;
mod exit;
mod ls;
pub mod prelude;
mod pwd;

pub type ExecuteFn = fn(CommandInput, &mut State, &Config) -> Result<CommandOutput, Fallacy>;

pub struct CommandInput {
    pub args: Vec<String>,
    pub papers: Option<PaperList>,
}

pub enum CommandOutput {
    None,
    Papers(PaperList),
    Message(String),
}

impl CommandInput {
    pub fn from_output(args: Vec<String>, output: CommandOutput) -> Self {
        let papers = match output {
            CommandOutput::None => None,
            CommandOutput::Message(_) => None,
            CommandOutput::Papers(p) => Some(p),
        };
        Self { args, papers }
    }
}

impl CommandOutput {
    pub fn into_string(self, state: &State) -> String {
        match self {
            CommandOutput::None => "".to_string(),
            CommandOutput::Message(s) => s,
            CommandOutput::Papers(p) => p.into_string(state),
        }
    }
}

// impl<'p> fmt::Display for CommandOutput<'p> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CommandOutput::None => write!(f, ""),
//             CommandOutput::Message(s) => write!(f, "{}", s),
//             CommandOutput::Papers(p) => write!(f, "{}", p),
//         }
//     }
// }

pub fn to_executor(command: String) -> Result<ExecuteFn, Fallacy> {
    match command.as_ref() {
        "cd" => Ok(cd::execute),
        "pwd" => Ok(pwd::execute),
        "ls" => Ok(ls::execute),
        "exit" => Ok(exit::execute),
        _ => Err(Fallacy::UnknownCommand(command.to_owned())),
    }
}

/// Split chained commands.
///
/// Reason implements its own command line parser.
/// By default arguments are delimited with whitespace, but they can be chunked
/// by grouping them in single quotes.
/// Literal single quotes can be entered by escaping them with a backslash.
///
/// Multiple commands can be piped with the pipe(`|`) character. Pipes should
/// only come between two commands.
pub fn parse_command(command: &str) -> Result<Vec<Vec<String>>, Fallacy> {
    let mut current_piece: String = String::new();
    let mut current_cmd: Vec<String> = Vec::new();
    let mut parsed_cmds: Vec<Vec<String>> = Vec::new(); // final result

    let mut inside_quotes = false; // whether we're inside single quotes
    let mut command_iter = command.chars().peekable();

    // A helper closure that consumes all whitespaces from an char peekable iterator.
    let consume_whitespace = |iter: &mut std::iter::Peekable<core::str::Chars>| {
        while let Some(c) = iter.peek() {
            if c.is_whitespace() {
                iter.next();
            } else {
                break;
            }
        }
    };

    // Trim whitespace in the beginning.
    consume_whitespace(&mut command_iter);

    // Parse commands.
    while let Some(c) = command_iter.next() {
        // Escaped single quote
        if c == '\\' && command_iter.peek() == Some(&'\'') {
            command_iter.next();
            current_piece.push('\'');
        }
        // Unescaped single quote
        else if c == '\'' {
            inside_quotes = !inside_quotes;
        }
        // Pipe
        // If we're inside quotes, this is merely a character part of a regex.
        // Otherwise, this indicates a command chain.
        else if c == '|' {
            if inside_quotes {
                current_piece.push(c);
            } else {
                // Wrap up the previous command and start a new one.
                if current_piece.len() != 0 {
                    current_cmd.push(String::new());
                    std::mem::swap(current_cmd.last_mut().unwrap(), &mut current_piece);
                }
                // Pipe encountered when `current_cmd` is empty: double pipes!
                if current_cmd.len() == 0 {
                    return Err(Fallacy::InvalidCommand("Invalid use of pipes.".to_owned()));
                }
                parsed_cmds.push(Vec::new());
                std::mem::swap(parsed_cmds.last_mut().unwrap(), &mut current_cmd);
            }
        }
        // Whitespace
        // If we're inside quotes, this is merely a character part of a regex.
        // Otherwise, this indicates the end of the current piece.
        else if c.is_whitespace() {
            if inside_quotes {
                current_piece.push(c);
            } else {
                // Consume all whitespaces that follow.
                consume_whitespace(&mut command_iter);

                // Wrap up the previous piece and start a new one.
                if current_piece.len() != 0 {
                    current_cmd.push(String::new());
                    std::mem::swap(current_cmd.last_mut().unwrap(), &mut current_piece);
                }
            }
        }
        // Everything else.
        else {
            current_piece.push(c);
        }
    }
    // No need to push empty pieces.
    if current_piece.len() != 0 {
        current_cmd.push(current_piece);
    }
    // Command ended with a pipe.
    if current_cmd.len() == 0 && parsed_cmds.len() != 0 {
        return Err(Fallacy::InvalidCommand(
            "Command ends with a dangling pipe.".to_owned(),
        ));
    }
    parsed_cmds.push(current_cmd);
    Ok(parsed_cmds)
}
