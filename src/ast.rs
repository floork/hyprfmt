#[derive(Debug, Clone)]
pub enum Node {
    Text(String),
    Unicode(String, i16),
    SpaceOrLine(String),
    Line(String),
    Indent(Vec<Node>),
    Group(i16, Vec<Node>),
    Nodes(Vec<Node>),
}

#[derive(Debug)]
pub struct ConfigAST {
    pub nodes: Vec<Node>, // List of AST nodes representing the entire config file
}
