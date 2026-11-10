use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Node {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
    name: String,
    from: Node,
    to: Node,
}

#[derive(Debug, Deserialize)]
pub struct GraphSchema {
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}
