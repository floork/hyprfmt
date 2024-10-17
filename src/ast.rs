#[derive(Debug, Clone)]
pub enum ASTNode {
    SpaceOrLine(String),
    Section(String, Vec<ASTNode>), // Represents sections like "decoration { ... }"
    KeyValues(String, Vec<String>, Option<String>), // Represents commands with multiple values, with optional inline comment
    Comment(String),                                // Represents standalone comments
}

#[derive(Debug)]
pub struct ConfigAST {
    pub nodes: Vec<ASTNode>, // List of AST nodes representing the entire config file
}
