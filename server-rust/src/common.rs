use std::fmt::Debug;

use serde::Serialize;
use serde::ser::SerializeStruct;
pub struct Node {
    pub x: f32,
    pub y: f32
}
impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.end()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ x: {}, y: {} }}", self.x, self.y)
    }
}



pub struct Edge{
    pub from_index: usize,
    pub to_index: usize,
}

pub struct Graph{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}