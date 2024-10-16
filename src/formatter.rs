use crate::ast::{ASTNode, ConfigAST};
use crate::Config;

pub fn format_ast(ast: &ConfigAST, config: Config, current_indent: usize) -> String {
    let mut output = String::new();

    // Create the current indentation string
    let indent_str = (0..current_indent).map(|_| " ").collect::<String>();

    for node in &ast.nodes {
        match node {
            ASTNode::Comment(comment) => {
                // Trim any leading spaces and ensure only one # is present
                let trimmed_comment = comment.trim_start_matches('#').trim_start();
                output.push_str(&format!("{}# {}\n", indent_str, trimmed_comment));
            }
            ASTNode::KeyValues(key, values) => {
                let values_str = values.join(", ");
                // Add current indentation for key-value pairs
                output.push_str(&format!("{}{} = {}\n", indent_str, key, values_str));
            }
            ASTNode::Section(name, nodes) => {
                // Add current indentation for section start
                output.push_str(&format!("{}{} {{\n", indent_str, name));
                // Recursively format the section's child nodes with increased indentation
                output.push_str(&format_ast(
                    &ConfigAST {
                        nodes: nodes.clone(),
                    },
                    config.clone(),
                    current_indent + config.indentation as usize, // Increase the indentation
                ));
                // Add current indentation for section end
                output.push_str(&format!("{}}}\n", indent_str));
            }
            ASTNode::SpaceOrLine(_) => {
                // Preserve the line as it is with the current indentation
                output.push_str(&format!("{}\n", indent_str));
            }
        }
    }

    output
}
