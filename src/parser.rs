use crate::ast::{ASTNode, ConfigAST};
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_config_to_ast(path: &str) -> ConfigAST {
    let file = fs::File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut ast = ConfigAST { nodes: Vec::new() };

    // Stack to handle nested sections
    let mut section_stack: Vec<(String, Vec<ASTNode>)> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let trimmed_line = line.trim();

        // Handle standalone comments
        if trimmed_line.starts_with('#') {
            if let Some((_, section_nodes)) = section_stack.last_mut() {
                section_nodes.push(ASTNode::Comment(line.clone()));
            } else {
                ast.nodes.push(ASTNode::Comment(line.clone()));
            }
        } else if trimmed_line.contains('{') {
            // Start a new section
            let section_name = trimmed_line.split('{').next().unwrap().trim().to_string();
            section_stack.push((section_name, Vec::new()));
        } else if trimmed_line.ends_with('}') {
            // End the current section
            if let Some((section_name, section_nodes)) = section_stack.pop() {
                let section_node = ASTNode::Section(section_name, section_nodes);
                if let Some((_, parent_nodes)) = section_stack.last_mut() {
                    parent_nodes.push(section_node);
                } else {
                    ast.nodes.push(section_node);
                }
            }
        } else if trimmed_line.contains('=') {
            // Handle key-value pairs with possible inline comment
            let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
            let key = parts[0].trim().to_string();
            let value_and_comment = parts[1].splitn(2, '#').collect::<Vec<&str>>();
            let values: Vec<String> = value_and_comment[0]
                .split(',')
                .map(|value| value.trim().to_string())
                .collect();

            // Extract inline comment if present
            let inline_comment = if value_and_comment.len() > 1 {
                Some(value_and_comment[1].trim().to_string())
            } else {
                None
            };

            let key_value_node = ASTNode::KeyValues(key, values, inline_comment);
            if let Some((_, section_nodes)) = section_stack.last_mut() {
                section_nodes.push(key_value_node);
            } else {
                ast.nodes.push(key_value_node);
            }
        } else if trimmed_line.is_empty() {
            // Handle blank lines explicitly
            if let Some((_, section_nodes)) = section_stack.last_mut() {
                section_nodes.push(ASTNode::SpaceOrLine(line.clone()));
            } else {
                ast.nodes.push(ASTNode::SpaceOrLine(line.clone()));
            }
        } else {
            // Handle other non-empty lines that are not recognized
            if let Some((_, section_nodes)) = section_stack.last_mut() {
                section_nodes.push(ASTNode::SpaceOrLine(line.clone()));
            } else {
                ast.nodes.push(ASTNode::SpaceOrLine(line.clone()));
            }
        }
    }

    // Handle any remaining open sections (in case the file doesn't end cleanly)
    while let Some((section_name, section_nodes)) = section_stack.pop() {
        ast.nodes
            .push(ASTNode::Section(section_name, section_nodes));
    }

    ast
}
