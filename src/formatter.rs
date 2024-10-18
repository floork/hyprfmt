use crate::ast::{ASTNode, ConfigAST};
use crate::Config;

pub fn format_ast(ast: &ConfigAST, config: Config, current_indent: usize) -> String {
    let mut output = String::new();
    let indent_str = (0..current_indent).map(|_| " ").collect::<String>();

    // Calculate the maximum width for inline comment alignment within this scope
    let max_width = if config.align_comments {
        ast.nodes
            .iter()
            .filter_map(|node| {
                if let ASTNode::KeyValues(key, values, _) = node {
                    Some(indent_str.len() + key.len() + 3 + values.join(", ").len())
                // "key = values"
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    } else {
        0
    };

    for node in &ast.nodes {
        match node {
            ASTNode::Comment(comment) => {
                let trimmed_comment = comment.trim();
                output.push_str(&format!("{}{}\n", indent_str, trimmed_comment));
            }
            ASTNode::KeyValues(key, values, inline_comment) => {
                let values_str = values.join(", ");
                output.push_str(&format!("{}{} = {}", indent_str, key, values_str));

                // Align the inline comment if present
                if let Some(comment) = inline_comment {
                    let trimmed_comment = comment.trim_start_matches('#').trim_start();
                    if config.align_comments && max_width > 0 {
                        let padding = max_width
                            .saturating_sub(indent_str.len() + key.len() + 3 + values_str.len());
                        output.push_str(&format!(
                            " {:padding$}# {}\n",
                            "",
                            trimmed_comment,
                            padding = padding
                        ));
                    } else {
                        output.push_str(&format!(" # {}\n", trimmed_comment));
                    }
                } else {
                    output.push_str("\n");
                }
            }
            ASTNode::Section(name, nodes) => {
                output.push_str(&format!("{}{} {{\n", indent_str, name));
                // Recursively format the section's child nodes, applying the same logic
                output.push_str(&format_ast(
                    &ConfigAST {
                        nodes: nodes.clone(),
                    },
                    config.clone(),
                    current_indent + config.indentation as usize,
                ));
                output.push_str(&format!("{}}}\n", indent_str));
            }
            ASTNode::SpaceOrLine(line) => {
                if line.trim().is_empty() {
                    output.push_str("\n");
                } else {
                    output.push_str(&format!("{}{}\n", indent_str, line));
                }
            }
        }
    }

    output
}
