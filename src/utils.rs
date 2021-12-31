use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

use crate::error::Fallacy;

/// Expands the leading tilde (~) in the given `PathBuf` with the
/// current user's home directory.
pub fn expand_tilde(path: &Path) -> Result<PathBuf, Fallacy> {
    if !path.starts_with("~") {
        return Ok(path.to_path_buf());
    }

    let path_str = match path.to_str() {
        Some(string) => string,
        None => return Err(Fallacy::PathInvalidUTF8(path.to_path_buf())),
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
pub fn expand_tilde_str(path: &str) -> Result<String, Fallacy> {
    if !path.starts_with('~') {
        return Ok(path.to_string());
    }

    match home::home_dir() {
        Some(mut home) => {
            // Filter out '~' and '~/'.
            if path.len() >= 2 {
                home.push(&path[2..]);
            }
            match home.to_str() {
                Some(string) => Ok(string.to_string()),
                None => Err(Fallacy::PathInvalidUTF8(home.clone())),
            }
        }
        None => Err(Fallacy::Homeless),
    }
}

/// Ask the user to input something.
/// Automatically appends ": " to the prompt string.
pub fn ask_for(prompt: &str, default: Option<String>) -> Result<String, Fallacy> {
    // Ask.
    match default {
        Some(ref value) => print!("{} (\"{}\"): ", prompt, value),
        None => print!("{}: ", prompt),
    };
    stdout().flush()?;

    // Get input.
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    if buffer.trim().is_empty() && default.is_some() {
        buffer = default.unwrap();
    }

    Ok(buffer.trim().to_string())
}

/// Ask confirmation to the user.
pub fn confirm(prompt: String, default: bool) -> Result<(), Fallacy> {
    // Ask.
    let yn = if default { " [Y/n] " } else { " [y/N] " };
    print!("{}", prompt + yn);
    stdout().flush()?;

    // Get input.
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    let default = if default {
        Ok(())
    } else {
        Err(Fallacy::FailedUserInteraction("".to_owned()))
    };

    match buffer.to_ascii_lowercase().trim() {
        "y" | "yes" => Ok(()),
        "n" | "no" => Err(Fallacy::FailedUserInteraction("".to_owned())),
        "" => default,
        _ => Err(Fallacy::FailedUserInteraction("Invalid input.".to_owned())),
    }
}

/// Ask the user to select among candidates.
pub fn select<'i, I>(prompt: &str, candidate: I) -> Result<usize, Fallacy>
where
    I: Iterator<Item = &'i str>,
{
    // Ask.
    print!("{}", prompt);
    let mut len = 0;
    for (index, cand) in candidate.enumerate() {
        len += 1;
        print!(" {}) {}.", index, cand);
    }
    print!("\n: ");
    stdout().flush()?;

    // Get input.
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    match buffer.trim().parse() {
        Ok(num) => {
            if num >= len {
                Err(Fallacy::FailedUserInteraction(
                    "Input out of range.".to_owned(),
                ))
            } else {
                Ok(num)
            }
        }
        Err(e) => Err(Fallacy::FailedUserInteraction(e.to_string())),
    }
}

/// Generate an appropriate filename from a papaer title.
/// Remove all non-alphanumeric characters and replace whitespaces to hyphens.
pub fn as_filename(title: &str) -> String {
    title
        .replace(|c: char| c.is_whitespace(), "-")
        .replace(|c: char| c != '-' && !c.is_ascii_alphanumeric(), "")
}

/// Append hyphen numbers at the end of the file path to find a path
/// that doesn't already exist in the filesystem.
pub fn make_unique_path(dir: &Path, name: &str, ext: &str) -> PathBuf {
    let mut attempt = 0;
    loop {
        let mut path = PathBuf::from(dir);
        let mut filename = name.to_owned();
        if attempt == 0 {
            filename.push_str(ext);
        } else {
            filename.push_str(&format!("-{}{}", attempt, ext));
        }
        path.push(&filename);

        if !path.exists() {
            return path;
        } else {
            attempt += 1;
        }
    }
}
