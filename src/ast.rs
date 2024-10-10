#[derive(Debug, Clone)]
pub enum ASTNode {
    Section(String, Vec<ASTNode>), // Represents sections like "decoration { ... }"
    KeyValues(String, Vec<String>), // Represents commands with multiple values
    Comment(String),               // Represents comments
}

#[derive(Debug)]
pub struct ConfigAST {
    pub nodes: Vec<ASTNode>, // List of AST nodes representing the entire config file
}
