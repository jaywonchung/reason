use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

use mdbook::book::{Link, MDBook, SectionNumber, Summary, SummaryItem};
use mdbook::config::Config as MDBookConfig;

use crate::cmd::prelude::*;
use crate::paper::PaperList;

pub static MAN: &str = include_str!("../../man/printf.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Build paper list from input.
    let selected = match input.papers {
        // Papers are given through pipe.
        Some(list) => list.0,
        // Papers are specified as filter.
        None => {
            match crate::cmd::ls::execute(input, state, config)? {
                CommandOutput::Papers(paper_list) => paper_list.0,
                // `ls` always returns CommandOutput::Papers.
                _ => panic!("ls did not return CommandOutput::Papers."),
            }
        }
    };

    // Build book config.
    let config_str = "
        [book]
        title = 'Reason'
        src = ''
        multilingual = true
        
        [build]
        create-missing = false
        
        [output.html]
        mathjax-support = true";
    let book_config = MDBookConfig::from_str(config_str)?;

    // Build summary.
    let mut formatted = Vec::new();
    let mut summary = Summary::default();
    for (sec, idx) in selected.into_iter().enumerate() {
        let p = &mut state.papers[idx];
        let notepath = match p.note_path(&config.storage.note_dir) {
            Ok(path) => path,
            Err(Fallacy::FailedUserInteraction(_)) => {
                println!("Skipping!");
                continue;
            }
            Err(_) => continue,
        };
        summary.numbered_chapters.push(SummaryItem::Link(Link {
            name: p.title.clone(),
            location: Some(notepath),
            number: Some(SectionNumber(vec![sec as u32 + 1])),
            nested_items: Vec::new(),
        }));
        formatted.push(idx);
    }

    // Build book.
    MDBook::load_with_config_and_summary(&config.storage.note_dir, book_config, summary)?
        .build()?;

    // Open book.
    let mut path = config.storage.note_dir.clone();
    path.push("book");
    path.push("index.html");
    if spawn(build_browser_command(&path, config)) {
        Ok(CommandOutput::Papers(PaperList(formatted)))
    } else {
        Ok(CommandOutput::None)
    }
}

fn spawn(mut command: Command) -> bool {
    match command.spawn() {
        Ok(_) => true,
        Err(e) => {
            if matches!(e.kind(), std::io::ErrorKind::NotFound) {
                println!("Invalid browser command: '{:?}'", e);
            } else {
                println!("Failed to spawn subprocess: '{:?}'", e);
            }
            false
        }
    }
}

fn build_browser_command(file: &Path, config: &Config) -> Command {
    let mut ret = Command::new(&config.output.browser_command[0]);
    let mut curly = false;
    for command in &config.output.browser_command[1..] {
        if command == "{}" {
            ret.arg(file);
            curly = true;
        } else {
            ret.arg(command);
        }
    }
    if !curly {
        ret.arg(file);
    }
    ret.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    ret
}
