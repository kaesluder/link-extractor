use clap::{Parser, ValueHint};
use std::{fs::File, io::Read};

mod parser;
use crate::parser::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input filename
    #[clap(value_parser, value_hint = ValueHint::FilePath)]
    filename: std::path::PathBuf,

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_contents = match load_file(&args.filename) {
        Ok(contents) => contents,
        Err(e) => return Err(Box::new(e)),
    };

    let file_path_as_string = match &args.filename.as_path().to_str() {
        Some(path_string) => path_string,
        None => "",
    };

    let link_list = extract_links(&file_contents, file_path_as_string);
    if args.json {
        let json_output = serde_json::to_string(&link_list)?;
        println!("{}", json_output);
    } else {
        let text_output: String = link_list.iter().fold(String::new(), |mut output, link| {
            output += &format!(
                "{}{}{}{}{}\n",
                link.source_file, args.separator, link.description, args.separator, link.url
            );
            output
        });
        println!("{}", text_output);
    }

    Ok(())
}
