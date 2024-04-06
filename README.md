# Markdown Link Extractor

**WIP**

## Simple Explanation

A simple utility to quickly and accurately extract links from Markdown files. 
Intended to be a tool for markdown-based personal wikis and notetaking systems.

Planned features:

- Read multiple files, output to STDOUT
- JSON and character-delimited text formats
- Fully documented cli options and source code

## Uses

(May be eventually bundled in this project.)

- Pipe output to fzf for terminal-based bookmark manager
- Automatic link checking
- Create maps of links (MOLs, like MOCs but external resources)
- Obsessive self-analysis, with statistics

## Technical Details

Uses [comrak](https://github.com/kivikakk/comrak) to parse markdown to an abstract syntax tree (AST).
This will *hopefully* be more accurate than attempting to extract links with regular expressions (regex). 


