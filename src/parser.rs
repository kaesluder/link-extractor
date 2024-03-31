use comrak::{
    arena_tree::NodeEdge,
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

fn iter_nodes_return_list<'a, F, R>(node: &'a AstNode<'a>, f: &F) -> Vec<R>
where
    F: Fn(&'a AstNode<'a>) -> Vec<R>,
{
    let mut results = Vec::new();

    // Collect results from the current node
    results.extend(f(node));

    // Recursively collect results from child nodes
    for c in node.children() {
        results.extend(iter_nodes_return_list(c, f));
    }

    results
}

fn extract_text<'a>(node: &'a AstNode<'a>) -> String {
    let texts: Vec<String> = iter_nodes_return_list(node, &|node| match node.data.borrow().value {
        NodeValue::Text(ref text) => vec![text.clone()],
        _ => vec![],
    });

    // Concatenate all strings collected from the nodes
    texts.concat()
}

fn extract_text_traverse<'a>(root: &'a AstNode<'a>) -> String {
    let mut output_text = String::new();

    // Use `traverse` to get an iterator of `NodeEdge` and process each.
    for edge in root.traverse() {
        if let NodeEdge::Start(node) = edge {
            // Handle the Start edge to process the node's value.
            if let NodeValue::Text(ref text) = node.data.borrow().value {
                // If the node is a text node, append its text to `output_text`.
                output_text.push_str(text);
            }
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
    iter_nodes(root, &mut |node| {
        if let NodeValue::Link(link) = &node.data.borrow().value {
            let url = link.url.clone();

            // Initialize an empty String to accumulate link text
            let title = extract_text_traverse(node);

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

    #[test]
    fn extract_text_test() {
        let markdown_input = "Hello, *worl[d](https://example.com/)*";
        let arena = Arena::new();
        let options = ComrakOptions::default();
        let root = parse_document(&arena, markdown_input, &options);
        assert_eq!("Hello, world", extract_text(root));
    }
    #[test]
    fn extract_text_traverse_test() {
        let markdown_input = "Hello, *worl[d](https://example.com/)*";
        let arena = Arena::new();
        let options = ComrakOptions::default();
        let root = parse_document(&arena, markdown_input, &options);
        assert_eq!("Hello, world", extract_text_traverse(root));
    }
}
