use std::io::Read;
use std::path::PathBuf;

use crate::error::Fallacy;

/// Expands the leading tilde (~) in the given `PathBuf` with the
/// current user's home directory.
pub fn expand_tilde(path: &PathBuf) -> Result<PathBuf, Fallacy> {
    if !path.starts_with("~") {
        return Ok(path.clone());
    }

    let path_str = match path.to_str() {
        Some(string) => string,
        None => return Err(Fallacy::InvalidUTF8(path.clone())),
    };

    match home::home_dir() {
        Some(mut home) => {
            // Filter out '~' and '~/'.
            if path_str.len() > 2 {
                home.push(&path_str[2..]);
            }
            Ok(home)
        }
        None => Err(Fallacy::Homeless),
    }
}

/// Expands the leading tilde (~) in the given `String` with the
/// current user's home directory.
pub fn expand_tilde_string(path: &String) -> Result<String, Fallacy> {
    if !path.starts_with("~") {
        return Ok(path.clone());
    }

    match home::home_dir() {
        Some(mut home) => {
            // Filter out '~' and '~/'.
            if path.len() >= 2 {
                home.push(&path[2..]);
            }
            match home.to_str() {
                Some(string) => Ok(string.to_string()),
                None => Err(Fallacy::InvalidUTF8(home.clone())),
            }
        }
        None => Err(Fallacy::Homeless),
    }
}

/// Asks confirmation to the user.
pub fn confirm(prompt: String, default: bool) -> Result<(), Fallacy> {
    let yn = if default { " [Y/n]" } else { " [y/N]" };
    print!("{}", prompt + yn);

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let default = if default {
        Ok(())
    } else {
        Err(Fallacy::FailedConfirmation("".to_owned()))
    };

    match buffer.to_ascii_lowercase().trim() {
        "y" | "yes" => Ok(()),
        "n" | "no" => Err(Fallacy::FailedConfirmation("".to_owned())),
        "" => default,
        _ => Err(Fallacy::FailedConfirmation("Invalid input.".to_owned())),
    }
}
