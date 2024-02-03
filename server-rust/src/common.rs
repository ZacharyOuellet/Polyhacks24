pub struct Node {
    pub x: f32,
    pub y: f32
}

pub struct Edge{
    pub from_index: i32,
    pub to_index: i32,
}

pub struct Graph{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}