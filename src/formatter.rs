use crate::ast::{ASTNode, ConfigAST};
use crate::Config;

pub fn format_ast(ast: &ConfigAST, config: Config) -> String {
    let mut output = String::new();

    for node in &ast.nodes {
        match node {
            ASTNode::Comment(comment) => {
                output.push_str(&format!("{}\n", comment));
            }
            ASTNode::KeyValues(key, values) => {
                let values_str = values.join(", ");
                output.push_str(&format!("{} = {}\n", key, values_str));
            }
            ASTNode::Section(name, nodes) => {
                output.push_str(&format!("{} {{\n", name));
                for child in nodes {
                    let indent = (0..config.indentation).map(|_| " ").collect::<String>();
                    output.push_str(&format!(
                        "{}{}",
                        indent,
                        &format_ast(
                            &ConfigAST {
                                nodes: vec![child.clone()],
                            },
                            config.clone(),
                        )
                    ))
                }
                output.push_str("}\n");
            }
            ASTNode::SpaceOrLine(_) => {
                output.push_str("\n"); // Always add a newline for space or line nodes
            }
        }
    }

    output
}
