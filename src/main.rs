mod parser;
use crate::parser::*;

fn main() {
    println!("Hello, world!");
    let links = extract_links("[example](https://www.example.com)", "file.md");
    println!("{}", links[0].url)
}
