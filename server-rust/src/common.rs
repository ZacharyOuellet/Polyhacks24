use std::fmt::Debug;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub x: f32,
    pub y: f32,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ x: {}, y: {} }}", self.x, self.y)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge{
    pub from_index: usize,
    pub to_index: usize,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Graph{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionRequest{
    pub graph: Graph,
    pub driver_start: usize,
    pub driver_end: usize,
    pub passenger_start: usize,
    pub passenger_end: usize,
}

pub struct SolutionResponse{
    pub driver_alone: Vec<usize>,
    pub driver_alone_distance: f32,
    pub driver_passenger: Vec<usize>,
    pub driver_passenger_distance: f32,
}
