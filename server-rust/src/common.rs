use std::fmt::Debug;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub x: f32,
    pub y: f32
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ x: {}, y: {} }}", self.x, self.y)
    }
}


#[derive(Serialize, Deserialize)]
pub struct Edge{
    pub from_index: usize,
    pub to_index: usize,
}


#[derive(Serialize, Deserialize)]
pub struct Graph{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}