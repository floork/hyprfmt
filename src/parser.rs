use crate::ast::{ConfigAST, Node};
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_config_to_ast(path: &str) -> ConfigAST {
    let file = fs::File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut ast = ConfigAST { nodes: Vec::new() };

    let mut current_section: Option<(String, Vec<Node>)> = None;

    for line in reader.lines() {
        let line = line.expect("Unable to read line").trim().to_string();

        // Handling comments
        if line.starts_with('#') {
            ast.nodes.push(Node::Comment(line));
        } else if line.ends_with('{') {
            // Start a new section
            let section_name = line[..line.len() - 1].trim().to_string();
            current_section = Some((section_name, Vec::new()));
        } else if line.ends_with('}') {
            // End the current section
            if let Some((section_name, section_nodes)) = current_section.take() {
                ast.nodes.push(Node::Section(section_name, section_nodes));
            }
        } else if line.contains('=') {
            // Handle key-value pairs, with multiple values separated by commas
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            let key = parts[0].trim().to_string();
            let values: Vec<String> = parts[1]
                .split(',')
                .map(|value| value.trim().to_string())
                .collect();

            if let Some((_, section_nodes)) = current_section.as_mut() {
                section_nodes.push(Node::KeyValues(key, values));
            } else {
                ast.nodes.push(Node::KeyValues(key, values));
            }
        }
    }

    ast
}
