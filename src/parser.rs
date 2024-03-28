use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Link {
    pub description: String,
    pub url: String,
    pub source_file: String,
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

/// Extracts links from the given Markdown text.
pub fn extract_links(markdown_input: &str, file_path: &str) -> Vec<Link> {
    let arena = Arena::new();
    let options = ComrakOptions::default();
    let root = parse_document(&arena, markdown_input, &options);

    let links = RefCell::new(Vec::new());
    iter_nodes(root, &mut |node| {
        if let NodeValue::Link(link) = &node.data.borrow().value {
            let url = link.url.clone();

            // Initialize an empty String to accumulate link text
            let mut title = String::new();

            // Iterate through the children of the link node to accumulate text
            for child in node.children() {
                if let NodeValue::Text(text) = &child.data.borrow().value {
                    title.push_str(text);
                }
            }

            links.borrow_mut().push(Link {
                source_file: file_path.to_string(),
                description: title,
                url,
            });
        }
    });
    links.into_inner()
}

#[cfg(test)]
mod tests {
    // Import everything from the outer module to make it available for tests
    use super::*;

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

    #[test]
    fn pass_over_fake_link() {
        let test_markdown = "[example] (https://www.example.com)";
        let test_markdown2 = "(https://www.example.com)";
        assert!(extract_links(test_markdown, "file.md").is_empty());
        assert!(extract_links(test_markdown2, "file.md").is_empty());
    }

    #[test]
    fn empty_string() {
        assert!(extract_links("", "").is_empty());
    }
}
