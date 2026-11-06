use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Node {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Relationship {
    name: String,
    from: Node,
    to: Node,
}

#[derive(Debug, Deserialize)]
pub struct GraphSchema {
    nodes: Vec<Node>,
    relationships: Vec<Relationship>,
}
