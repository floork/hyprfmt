use crate::ast::{ASTNode, ConfigAST};

pub fn format_ast(ast: &ConfigAST) -> String {
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
                    output.push_str(&format_ast(&ConfigAST {
                        nodes: vec![child.clone()],
                    }));
                }
                output.push_str("}\n");
            }
        }
    }

    output
}
