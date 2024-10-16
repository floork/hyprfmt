use crate::ast::{ASTNode, ConfigAST};
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_config_to_ast(path: &str) -> ConfigAST {
    let file = fs::File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut ast = ConfigAST { nodes: Vec::new() };

    let mut current_section: Option<(String, Vec<ASTNode>)> = None;

    for line in reader.lines() {
        let line = line.expect("Unable to read line").trim().to_string();

        // Handle comments
        if line.starts_with('#') {
            // Push comments only if there's an active section
            if let Some((_, section_nodes)) = current_section.as_mut() {
                section_nodes.push(ASTNode::Comment(line));
            } else {
                ast.nodes.push(ASTNode::Comment(line));
            }
        } else if line.contains('{') {
            // Start a new section
            let section_name = line.split('{').next().unwrap().trim().to_string();
            current_section = Some((section_name, Vec::new()));
        } else if line.ends_with('}') {
            // End the current section
            if let Some((section_name, section_nodes)) = current_section.take() {
                ast.nodes
                    .push(ASTNode::Section(section_name, section_nodes));
            }
        } else if line.contains('=') {
            // Handle key-value pairs
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            let key = parts[0].trim().to_string();
            let mut values: Vec<String> = parts[1]
                .split(',')
                .map(|value| value.trim().to_string())
                .collect();

            // Check for inline comments
            if let Some(comment_index) = line.find('#') {
                let inline_comment = line[comment_index..].trim().to_string();
                values.push(inline_comment); // Treat inline comments as values for key-value pairs
            }

            if let Some((_, section_nodes)) = current_section.as_mut() {
                section_nodes.push(ASTNode::KeyValues(key, values));
            } else {
                ast.nodes.push(ASTNode::KeyValues(key, values));
            }
        } else if !line.is_empty() {
            if let Some((_, section_nodes)) = current_section.as_mut() {
                section_nodes.push(ASTNode::SpaceOrLine(line));
            } else {
                ast.nodes.push(ASTNode::SpaceOrLine(line));
            }
        } else if line.is_empty() {
            ast.nodes.push(ASTNode::SpaceOrLine(line));
        }
    }

    ast
}
