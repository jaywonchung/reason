use crate::config::Config;
use crate::error::Fallacy;
use crate::paper::PaperList;
use crate::state::State;

mod cd;
mod exit;
mod ls;
mod man;
mod open;
pub mod prelude;
mod pwd;
mod read;
mod rm;
mod touch;

pub static MAN: &'static str = "The Reason command line.

## Shell-like experience

Reason provides a real shell-like experience. You can press
- <Up> to load the previous command in history
- <Down> to load the next command in history
- <Ctrl-a> to move the cursor to the beginning of the line
- <Ctrl-e> to move the cursor to the end of the line
- <Ctrl-l> to clear the screen
- <Ctrl-u> to kill the entire line
and more!

## Running commands

Here are the basic rules by which commands are parsed:
- Whitespaces separate arguments.
  Ex) `ls shadowtutor` consists of two arguments.
- Pipes separate commands. Papers are passed to the next command.
  Ex) `ls shadowtutor | rm` will remove all papers whose title
  matches 'shadowtutor'.
- Single-quote your commands to escape from the above rules.
  Ex) `ls 'shadow | tutor'` (still) consists of two arguments.

## Piping commands

As seen above, commands can be chained with pipes. When a
previous command produces a list of papers, for instance `ls`,
the list of papers can be passed to the next command for
further processing.

For instance, `ls shadowtutor | open` will open all papers
that have the word 'shadwotutor' in their titles.

Most commands output a paper list - all those whose outputs
are displayed as tables: `ls`, `open`, `read`, and `touch`.
";

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
    pub fn into_string(self, state: &State, config: &Config) -> String {
        match self {
            CommandOutput::None => "".to_string(),
            CommandOutput::Message(s) => s,
            CommandOutput::Papers(p) => p.into_string(state, config),
        }
    }
}

pub fn to_executor(command: String) -> Result<ExecuteFn, Fallacy> {
    match command.as_ref() {
        "cd" => Ok(cd::execute),
        "exit" => Ok(exit::execute),
        "ls" => Ok(ls::execute),
        "man" => Ok(man::execute),
        "open" => Ok(open::execute),
        "pwd" => Ok(pwd::execute),
        "read" => Ok(read::execute),
        "rm" => Ok(rm::execute),
        "touch" => Ok(touch::execute),
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

    // Command shouldn't start with a pipe.
    if command_iter.peek() == Some(&'|') {
        return Err(Fallacy::InvalidCommand(
            "Command cannot start with a pipe.".to_owned(),
        ));
    }

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
            "Command cannot end with a dangling pipe.".to_owned(),
        ));
    }
    parsed_cmds.push(current_cmd);
    Ok(parsed_cmds)
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parse_test {
        ($name:ident: $command:expr, $answer:expr) => {
            #[test]
            fn $name() {
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
        Err(Fallacy::InvalidCommand("Command cannot end with a dangling pipe.".to_owned()))
    );
    parse_test!(ends_with_pipe2:
        "ls shadowtutor | ",
        Err(Fallacy::InvalidCommand("Command cannot end with a dangling pipe.".to_owned()))
    );
    parse_test!(starts_with_pipe1:
        "|ls shadowtutor",
        Err(Fallacy::InvalidCommand("Command cannot start with a pipe.".to_owned()))
    );
    parse_test!(starts_with_pipe2:
        " |  ls shadowtutor",
        Err(Fallacy::InvalidCommand("Command cannot start with a pipe.".to_owned()))
    );
}
