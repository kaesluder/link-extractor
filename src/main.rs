use clap::{Parser, ValueHint};
use csv::WriterBuilder;
use std::{fs::File, io::Read};

mod parser;
use crate::parser::*;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Extracts link data from markdown files producing json or character-delimited text.\nOutput is sent to STDOUT."
)]
struct Args {
    /// Input filenames
    #[clap(value_parser, value_hint = ValueHint::FilePath)]
    filenames: Vec<std::path::PathBuf>,

    /// Output JSON format
    #[clap(short, long)]
    json: bool,

    /// Field separator
    #[clap(short, long, default_value = ",")]
    separator: String,
}

/// Load text as `String` from filename
///
/// # Inputs
///
/// - `filename`: `std::path::PathBuf` pointing to markdown file
///
/// # Results
///
/// - `Ok(string)`: String contents of file.
/// - `Err(e)`: Error condition.
fn load_file(filename: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parse markdown from `filename` returning a `Result` with a `Vec` of `Link`s
///
/// # Inputs
///
/// * `filename`: std::path::PathBuf pointing to the markdown file
///
/// # Results
///
/// - `Ok(links)`: `Vec` of `Link` structs. List may be empty if no links in file.
/// - `Err(e)`: Error result. Handle this!
///
fn parse_from_filename(filename: &std::path::PathBuf) -> Result<Vec<parser::Link>, std::io::Error> {
    let contents = load_file(filename)?;
    let links = extract_links(&contents, &filename.to_string_lossy());
    Ok(links)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let link_list: Vec<parser::Link> = args
        .filenames
        .iter()
        .filter_map(|filename| match parse_from_filename(filename) {
            Ok(links) => Some(links),
            Err(e) => {
                eprintln!("Error parsing file {:?}: {}", filename, e);
                None
            }
        })
        .flat_map(|links| links.into_iter())
        .collect();

    if args.json {
        // json serializer
        println!("{}", serde_json::to_string_pretty(&link_list)?);
    } else {
        // csv serializer
        let mut wtr = WriterBuilder::new()
            .delimiter(args.separator.as_bytes()[0])
            .from_writer(std::io::stdout());
        for link in link_list {
            wtr.serialize(link)?;
        }
        wtr.flush()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    // Import everything from the outer module to make it available for tests
    use super::*;

    /// Check that loading function can load a string
    #[test]
    fn load_existing_file_returns_string() -> std::io::Result<()> {
        let filepath = std::path::PathBuf::from("test_markdown/three_links.md");
        let test_text = load_file(&filepath)?;
        assert!(test_text.contains("https://example.com"));

        Ok(())
    }

    /// Check that loading function returns an err if given a bad filepath
    #[test]
    fn load_non_existing_file_returns_err() {
        let filepath = std::path::PathBuf::from("");
        assert!(load_file(&filepath).is_err());
    }

    /// Test parsing of a simple test file in the test_markdown dir
    /// test file should have three links in the format:
    /// `[three links: <a, b, c>](https://example.com)`
    #[test]
    fn parse_markdown_with_links() {
        let filepath = std::path::PathBuf::from("test_markdown/three_links.md");
        let links = parse_from_filename(&filepath).unwrap();
        assert_eq!(links.len(), 3);
        assert_eq!(links[0].url, "https://example.com");
        assert!(links[0].description.contains("three links: a"))
    }

    /// Test parsing of file without any links
    /// File can be any format as long as there are no links.
    /// Parser should return empty list.
    #[test]
    fn parse_markdown_without_links() {
        let filepath = std::path::PathBuf::from("test_markdown/no_links.md");
        let links = parse_from_filename(&filepath).unwrap();
        assert_eq!(links.len(), 0)
    }

    /// Test handling of bad filepath.
    /// Errors should trickle up to interface logic for handling.
    #[test]
    fn parse_nofile_returns_err() {
        let filepath = std::path::PathBuf::from("");
        assert!(parse_from_filename(&filepath).is_err());
    }
}
