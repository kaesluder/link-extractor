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

fn load_file(filename: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_from_filename(filename: &std::path::PathBuf) -> Result<Vec<parser::Link>, std::io::Error> {
    let contents = load_file(filename)?;
    let links = extract_links(&contents, &filename.to_string_lossy().into_owned());
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
