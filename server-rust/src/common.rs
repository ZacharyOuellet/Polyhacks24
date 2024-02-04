pub struct Node {
    pub x: f32,
    pub y: f32
}

pub struct Edge{
    pub from_index: usize,
    pub to_index: usize,
}

pub struct Graph{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}