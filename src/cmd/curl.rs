use md5;
use std::fs::File;
use std::io::{stdin, stdout, Cursor, Write};
use std::path::PathBuf;
use std::time::Duration;
use symlink::symlink_file;

use crate::cmd::prelude::*;
use crate::paper::{Paper, PaperList};
use crate::utils::{as_filename, select};

use soup::prelude::*;

pub static MAN: &str = include_str!("../../man/curl.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Check if source is given.
    let mut args = input.args;
    if args.len() <= 1 {
        return Err(Fallacy::CurlNoSource);
    }

    // Parse source and route to appropriate function.
    let url = args.remove(1);
    let paper = if url.contains("arxiv") {
        from_arxiv(url.as_ref(), config)?
    } else if url.contains("usenix") {
        from_usenix(url.as_ref(), config)?
    } else if url.contains("pdf") {
        from_pdf(url.as_ref(), config)?
    } else {
        return Err(Fallacy::CurlUnknownSource(url));
    };

    // Add paper to state.
    state.papers.push(paper);

    Ok(CommandOutput::Papers(PaperList(vec![
        state.papers.len() - 1,
    ])))
}

/// Prompt with information extracted from pdf as default.
fn prompt_with_default(name: &str, pdf: &pdf::file::File<Vec<u8>>, field: &str) -> String {
    /// Some cleaning on the extracted metadata.
    fn parse_value<T>(s: T) -> String
    where
        T: ToString,
    {
        let s = s.to_string().trim().to_owned();
        let s = s.strip_prefix("\"").unwrap_or(&s).to_owned();
        s.strip_suffix("\"").unwrap_or(&s).to_owned()
    }

    let value = pdf
        .trailer
        .info_dict
        .as_ref()
        .and_then(|d| d.get(field).map(parse_value))
        .unwrap_or(String::new());

    let mut buffer = String::new();

    // prompt
    loop {
        print!("{0} (default: \"{1}\"): ", &name, &value);
        let _ = stdout().flush();
        match stdin().read_line(&mut buffer) {
            Ok(_) => break,
            _ => buffer.clear(),
        }
    }

    buffer = buffer.trim().to_owned();

    (if buffer.is_empty() { value } else { buffer })
        .trim()
        .to_string()
}

/// If the title is known, then just use the title.
/// Otherwise, the title is parsed by some parser.
enum Title<'a> {
    Just(&'a str),
    Parser(&'a mut dyn FnMut(&PathBuf) -> Result<String, Fallacy>),
}

/// Download PDF file to local storage with the title as its name.
/// If the title is not given, prompt with information parsed from pdf file as default.
/// The URL will be remembered by creating a symlink to downloaded PDF file named with its MD5 digest.
/// If the URL is already downloaded, don't overwrite but raise an error.
fn download_pdf(
    url: &str,
    title: Title,
    client: &reqwest::blocking::Client,
    config: &Config,
) -> Result<PathBuf, Fallacy> {
    // Remove broken links under directory just before every time we download.
    // Broken symlinks may be due to the last failed downloading or
    // user manually deleting the rdbpath.
    for entry in config.storage.file_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if entry.metadata()?.file_type().is_symlink() && !entry.path().exists() {
                std::fs::remove_file(entry.path())?;
            }
        }
    }

    // Create a md5 digest for url, and the local path of the url (md5path).
    // As the title of the pdf is unknown in the beginning, we first download
    // the url file to the md5path. After getting the title, we further
    // rename the downloaded file to a readable path (rdbpath)
    // and change the md5 path as a symlink to the rdbpath.
    let digest = PathBuf::from(format!("{:x}.pdf", md5::compute(&url)));
    let mut md5path = config.storage.file_dir.clone();
    md5path.push(&digest);

    if let Ok(rdbpath) = std::fs::read_link(&md5path) {
        // If md5path exists and the file it points to also exists, raise a file exists error.
        return Err(Fallacy::CurlFileExistsError(rdbpath));
    } else if md5path.as_path().exists() {
        // If, for some unknown reason, md5path is there but not a symlink, delete it.
        std::fs::remove_file(&md5path)?;
    }

    // Download the file.
    let mut cursor = Cursor::new(
        client
            .get(url)
            .timeout(Duration::from_secs(90))
            .send()?
            .bytes()?,
    );
    let mut file = File::create(&md5path)?;
    std::io::copy(&mut cursor, &mut file)?;

    // Determine the readable file path.
    // If the title is a Just, then use it directly.
    // Otherwise, parse it from the md5path.
    let filename = match title {
        Title::Just(title) => as_filename(title) + ".pdf",
        Title::Parser(parse) => as_filename(parse(&md5path)?.as_ref()) + ".pdf",
    };
    let filepath = PathBuf::from(filename);

    let mut rdbpath = config.storage.file_dir.clone();
    rdbpath.push(&filepath);

    if rdbpath.exists() {
        // Remove the newly downloaded file, make md5path link to the old rdbpath.
        // Raise the file exists error.
        std::fs::remove_file(&md5path).unwrap();
        symlink_file(&rdbpath, &md5path)?;
        return Err(Fallacy::CurlFileExistsError(rdbpath));
    }

    // Rename the unreadable md5path to readable path
    std::fs::rename(&md5path, &rdbpath).unwrap();

    // Reverse link
    symlink_file(&rdbpath, &md5path).unwrap();

    Ok(filepath)
}

fn from_arxiv(url: &str, config: &Config) -> Result<Paper, Fallacy> {
    // NOTE: There's the arXiv export API, but we need to parse XML to use that.
    //       xml-rs seems good enough, but I'd rather not add another dependency
    //       just for this. As of now our use case is simple and parsing HTML
    //       with soup seems tractable. However, if things get more complicated,
    //       consider switching to using the API.

    // Parse and validate url.
    // https://arxiv.org/
    let parsed_url = url::Url::parse(url)?;
    if parsed_url.cannot_be_a_base() {
        return Err(Fallacy::CurlInvalidSourceUrl(url.to_owned()));
    }
    let segments: Vec<_> = parsed_url.path_segments().unwrap().collect();
    if segments.len() != 2
        || !parsed_url.has_host()
        || !parsed_url.host_str().unwrap().ends_with("arxiv.org")
        || segments[0] != "abs"
    {
        return Err(Fallacy::CurlInvalidSourceUrl(url.to_owned()));
    }
    let pieces: Vec<_> = segments[1].split('.').collect();
    if pieces.len() != 2 || pieces[0].len() != 4 || pieces[1].len() != 5 {
        return Err(Fallacy::CurlInvalidSourceUrl(url.to_owned()));
    }
    if !pieces.iter().all(|p| p.chars().all(|c| c.is_numeric())) {
        return Err(Fallacy::CurlInvalidSourceUrl(url.to_owned()));
    }
    let venue = "arXiv".to_owned();
    let year = format!("20{}", &pieces[0][..2]);

    // Initialize HTTP client.
    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;

    // Parse title.
    let res = client.get(url).send()?;
    let soup = Soup::from_reader(res)?;
    let title = match soup.class("title").find() {
        Some(title) => title,
        None => {
            return Err(Fallacy::CurlCannotFindTitle(
                "No class named 'title' found.".to_owned(),
            ))
        }
    };
    let title = match title.children().last() {
        Some(title) => title.text(),
        None => {
            return Err(Fallacy::CurlCannotFindTitle(
                "Class 'title' has no children.".to_owned(),
            ))
        }
    };

    // Parse author list.
    let authors = match soup.class("authors").find() {
        Some(authors) => authors,
        None => {
            return Err(Fallacy::CurlCannotFindAuthor(
                "No class named 'authors' found.".to_owned(),
            ))
        }
    };
    let authors: Vec<String> = authors.tag("a").find_all().map(|a| a.text()).collect();

    // Download paper PDF.
    let filepath = download_pdf(
        &format!("https://arxiv.org/pdf/{}.pdf", segments[1]),
        Title::Just(&title),
        &client,
        &config,
    )?;

    Ok(Paper {
        title,
        authors,
        venue,
        year,
        filepath: Some(filepath),
        ..Default::default()
    })
}

fn from_usenix(url: &str, config: &Config) -> Result<Paper, Fallacy> {
    // Parse and validate source url.
    // https://usenix.org/conference/atc21/presentation/lee
    let parsed_url = url::Url::parse(url)?;
    if parsed_url.cannot_be_a_base() {
        return Err(Fallacy::CurlInvalidSourceUrl(url.to_owned()));
    }
    let segments: Vec<_> = parsed_url.path_segments().unwrap().collect();
    if segments.len() != 4
        || !parsed_url.has_host()
        || !parsed_url.host_str().unwrap().ends_with("usenix.org")
        || segments[0] != "conference"
        || segments[2] != "presentation"
    {
        return Err(Fallacy::CurlInvalidSourceUrl(url.to_owned()));
    }
    let conf = segments[1];
    let venue = {
        if conf.to_lowercase().contains("usenix") {
            conf[6..conf.len() - 2].to_uppercase()
        } else {
            conf[..conf.len() - 2].to_uppercase()
        }
    };
    let year = format!("20{}", &conf[conf.len() - 2..]);

    // Initialize HTTP client.
    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;

    // Parse title.
    let res = client.get(url).send()?;
    let soup = Soup::from_reader(res)?;
    let title = match soup.attr("id", "page-title").find() {
        Some(title) => title.text(),
        None => {
            return Err(Fallacy::CurlCannotFindTitle(
                "No element with id 'page-title' found.".to_owned(),
            ))
        }
    };

    // Parse author list.
    let authors = match soup.class("field-name-field-paper-people-text").find() {
        Some(authors) => authors,
        None => {
            return Err(Fallacy::CurlCannotFindAuthor(
                "No class named 'field-name-field-paper-people-text' found.".to_owned(),
            ))
        }
    };
    let p = match authors.tag("p").find() {
        Some(p) => p,
        None => {
            return Err(Fallacy::CurlCannotFindAuthor(
                "Cannot find 'p' tag inside author element.".to_owned(),
            ))
        }
    };
    let authors: Vec<String> = p
        .children()
        .filter(|child| child.is_text())
        .flat_map(|child| {
            child
                .display()
                .replace("and", ",")
                .split(',')
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .collect();

    // Parse PDF url.
    // Some presentations have both a pre-print and a camera-ready version (e.g.,
    // USENIX Security). We should ask the user which one to download.
    let pdf = {
        // Find file elements that have a link inside.
        let mut files: Vec<_> = soup
            .class("file")
            .find_all()
            .filter_map(|f| {
                if let Some(a) = f.tag("a").find() {
                    a.get("href").map(|href| (a.text(), href))
                } else {
                    None
                }
            })
            .collect();

        if files.is_empty() {
            None
        } else if files.len() == 1 {
            Some(files.remove(0).1)
        } else {
            let selected = select("Multiple files found:", files.iter().map(|f| f.0.as_ref()))?;
            Some(files.remove(selected).1)
        }
    };

    // Maybe download paper PDF.
    let filepath = if let Some(pdf) = pdf {
        Some(download_pdf(&pdf, Title::Just(&title), &client, &config)?)
    } else {
        println!("Paper PDF not found. Skipping PDF download.");
        None
    };

    // Create a `Paper` object and return it.
    Ok(Paper {
        title,
        authors,
        venue,
        year,
        filepath,
        ..Default::default()
    })
}

fn from_pdf(url: &str, config: &Config) -> Result<Paper, Fallacy> {
    let parsed_url = url::Url::parse(url)?;
    if parsed_url.cannot_be_a_base() {
        return Err(Fallacy::CurlInvalidSourceUrl(url.to_owned()));
    }

    // Initialize HTTP client.
    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;

    let mut maybe_title = None;
    let mut maybe_pdf = None;

    let mut parse = |path: &PathBuf| -> Result<String, Fallacy> {
        let pdf = pdf::file::File::open(path.to_owned())?;
        let title = prompt_with_default("title", &pdf, "Title");
        maybe_title = Some(title.clone());
        maybe_pdf = Some(pdf);
        Ok(title)
    };

    let filepath = download_pdf(&url, Title::Parser(&mut parse), &client, &config)?;

    let title = maybe_title.unwrap();
    let pdf = maybe_pdf.unwrap();

    let authors = prompt_with_default("authors", &pdf, "Author")
        .split(",")
        .map(str::trim)
        .map(str::to_string)
        .collect();
    let venue = prompt_with_default("venue", &pdf, "Venue");
    let year = prompt_with_default("year", &pdf, "Year");

    Ok(Paper {
        title,
        authors,
        venue,
        year,
        filepath: Some(filepath),
        ..Default::default()
    })
}
