use crate::parser::Link;
use comrak::{markdown_to_html, Options};
use scraper::Html;

pub fn parse_file_scraper(file_path: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path)?;
    let markdown_content = markdown_to_html(&file_content, &Options::default());
    let fragment = Html::parse_fragment(&markdown_content);
    Ok(fragment)
}

pub fn extract_links_scraper(
    document: &Html,
    file: &str,
) -> Result<Vec<Link>, Box<dyn std::error::Error>> {
    use scraper::Selector;
    let selector = Selector::parse("a")?;
    let mut links = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let text = element.text().collect::<String>();
            links.push(Link {
                description: text,
                url: href.to_string(),
                source_file: file.to_string(),
            });
        }
    }
    Ok(links)
}

// parser_scraper.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_load_file() {
        // Create a temporary file
        let temp_file_path = PathBuf::from("temp_test_file.txt");
        let mut temp_file = File::create(&temp_file_path).expect("Failed to create temp file");

        // Write some data to the file
        writeln!(temp_file, "Hello, world!").expect("Failed to write to temp file");

        // Use load_file function
        let result = crate::load_file(&temp_file_path);

        // Assert the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!\n");
    }

    #[test]
    fn test_parse_file_scraper() {
        let test_file_path = "test_markdown/three_links.md";
        // Call the function to be tested
        let result = parse_file_scraper(test_file_path);

        // Check the result
        assert!(result.is_ok());
        let fragment = result.unwrap();
        let links: Vec<_> = fragment
            .select(&scraper::Selector::parse("a").unwrap())
            .collect();
        assert_eq!(links.len(), 3);
    }

    #[test]
    fn test_extract_links_scraper() {
        let test_file_path = "test_markdown/three_links.md";

        let document = parse_file_scraper(&test_file_path).unwrap();
        // Call the function to be tested
        let result = extract_links_scraper(&document, &test_file_path);

        // Check the result
        assert!(result.is_ok());
        let links = result.unwrap();
        assert_eq!(links.len(), 3);
        assert_eq!(links[0].url, "https://example.com");
        assert!(links[0].description.contains("three links: a"));
        assert_eq!(links[2].url, "https://example.com");
        assert!(links[2].description.contains("three links: c"));
    }
}
