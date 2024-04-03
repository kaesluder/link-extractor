use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

/// Represents a hyperlink extracted from a markdown document.
///
/// # Fields
/// * `description` - A `String` that holds the text description or the anchor text of the link.
/// * `url` - A `String` containing the URL the link points to. This should be a valid URL.
/// * `source_file` - A `String` specifying the path or name of the source file from which
///   the link was extracted.
///
/// # Example
/// ```
/// use serde_json::{json, to_string};
///
/// let link = Link {
///     description: "Example".to_string(),
///     url: "https://www.example.com".to_string(),
///     source_file: "file.md".to_string(),
/// };
///
/// // Example of serializing the `Link` struct to a JSON string
/// let serialized_link = to_string(&link).unwrap();
/// println!("{}", serialized_link);
///
/// // Output: {"description":"Example","url":"https://www.example.com","source_file":"file.md"}
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Link {
    pub description: String,
    pub url: String,
    pub source_file: String,
}

/// Extracts and concatenates all text from a given abstract syntax tree (AST) node and its descendants.
///
/// This function traverses an AST, starting from a specified root node, and accumulates all text
/// content found within text nodes into a single `String`. This process involves iterating over the
/// descendants of the root node, identifying text nodes, and appending their content to the output string.
///
/// # Arguments
/// * `root` - A reference to the root `AstNode` from which to start text extraction. This node and its
///            descendants will be traversed to find and concatenate text.
///
/// # Returns
/// A `String` containing all text content extracted from the root node and its descendants. If no text
/// nodes are found, an empty string is returned.
///
/// # Examples
/// Parse an AST starting at the root of the document or a tag. Using `extract_text` on the root node of this
/// AST would return all the plaintext content from the document, stripped of any markdown formatting.
///
/// ```ignore
/// let arena = Arena::new();
/// let options = ComrakOptions::default();
/// let root = parse_document(&arena, markdown_input, &options);
/// let extracted_text = extract_text(&root);
/// assert_eq!(extracted_text, "Hello WorldThis is a sample text.");
/// ```
///
/// # Note
/// This function does not preserve the original formatting. It purely concatenates text content found within text nodes.
fn extract_text<'a>(root: &'a AstNode<'a>) -> String {
    let mut output_text = String::new();

    // Use `traverse` to get an iterator of `NodeEdge` and process each.
    for node in root.descendants() {
        if let NodeValue::Text(ref text) = node.data.borrow().value {
            // If the node is a text node, append its text to `output_text`.
            output_text.push_str(text);
        }
    }

    output_text
}

/// Extracts links from the given Markdown text.
pub fn extract_links(markdown_input: &str, file_path: &str) -> Vec<Link> {
    let arena = Arena::new();
    let options = ComrakOptions::default();
    let root = parse_document(&arena, markdown_input, &options);

    let links = RefCell::new(Vec::new());
    for node in root.descendants() {
        if let NodeValue::Link(link) = &node.data.borrow().value {
            let url = link.url.clone();

            // Initialize an empty String to accumulate link text
            let title = extract_text(node);

            links.borrow_mut().push(Link {
                source_file: file_path.to_string(),
                description: title,
                url,
            });
        }
    }
    links.into_inner()
}

#[cfg(test)]
mod tests {
    // Import everything from the outer module to make it available for tests
    use super::*;

    /// Tests the `extract_links` function with an ideal markdown link string.
    ///
    /// # Expected Result:
    /// The function is expected to return a vector with a single `Link` struct that exactly matches
    /// the `target` struct defined in the test, indicating that the function correctly extracted the link
    /// from the provided markdown string.    
    #[test]
    fn extract_from_ideal_string() {
        let target = Link {
            description: "example".to_string(),
            url: "https://www.example.com".to_string(),
            source_file: "file.md".to_string(),
        };
        let test_markdown = "[example](https://www.example.com)";
        assert_eq!(vec![target], extract_links(test_markdown, "file.md"));
    }

    /// Tests the `extract_links` function with a string containing two links.
    ///
    /// # Expected Result:
    /// The function is expected to return a vector with 2 `Link` structs that exactly match
    /// the `target` struct defined in the test, indicating that the function correctly extracted the links
    /// from the provided markdown string.    
    #[test]
    fn extract_multiple_from_ideal_string() {
        let target = vec![
            Link {
                description: "example".to_string(),
                url: "https://www.example.com".to_string(),
                source_file: "file.md".to_string(),
            },
            Link {
                description: "example".to_string(),
                url: "https://www.example.com".to_string(),
                source_file: "file.md".to_string(),
            },
        ];
        let test_markdown = "* [example](https://www.example.com)
        *  [example](https://www.example.com) ";
        assert_eq!(target, extract_links(test_markdown, "file.md"));
        assert_eq!(2, extract_links(test_markdown, "file.md").len());
    }

    /// Tests the `extract_links` function with two examples of malformed markdown.
    ///
    /// # Expected Result:
    /// The function is expected to return an empty vector for each example.
    #[test]
    fn pass_over_fake_link() {
        let test_markdown = "[example] (https://www.example.com)";
        let test_markdown2 = "(https://www.example.com)";
        assert!(extract_links(test_markdown, "file.md").is_empty());
        assert!(extract_links(test_markdown2, "file.md").is_empty());
    }

    /// Tests the `extract_links` function with empty strings for markdown and filename.
    ///
    /// # Expected Result:
    /// The function is expected to return an empty vector.
    #[test]
    fn empty_string() {
        assert!(extract_links("", "").is_empty());
    }

    /// Tests the `extract_text` function with nested markdown.
    ///
    /// # Expected Result:
    /// The function is expected to return text elements striped of
    /// markup.
    #[test]
    fn extract_text_test() {
        let markdown_input = "Hello, *worl[d](https://example.com/)*";
        let arena = Arena::new();
        let options = ComrakOptions::default();
        let root = parse_document(&arena, markdown_input, &options);
        assert_eq!("Hello, world", extract_text(root));
    }
}
