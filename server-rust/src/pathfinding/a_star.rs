use petgraph::graph::UnGraph;
use petgraph::prelude::NodeIndex;
use crate::pathfinding::path_types::PathNode;
use std::collections::BinaryHeap;
use crate::common::{Graph, Node};
use crate::pathfinding::graph_converter::convert_to_petgraph;



fn distance_between_nodes(node1: &Node, node2: &Node) -> f32 {
    ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2)).sqrt()
}

pub fn shortest_path(graph: &UnGraph<Node, f32>, start: usize, end: usize) -> Vec<usize> {

    let mut path_nodes: Vec<PathNode> = Vec::new();
    
    let end_node = graph.node_weight(NodeIndex::new(end));
    let mut to_explore: BinaryHeap<&PathNode> = BinaryHeap::new();
    let start_node = graph.node_weight(NodeIndex::new(start));
    Vec::new()
}

pub fn test(){
    let graph = Graph {
        nodes: vec![
            crate::common::Node { x: 0.0, y: 0.0 },
            crate::common::Node { x: 1.5, y: 0.0 },
            crate::common::Node { x: 1.0, y: 2.0 },
            crate::common::Node { x: 0.0, y: 1.0 },
        ],
        edges: vec![
            crate::common::Edge {
                from_index: 0,
                to_index: 1,
            },
            crate::common::Edge {
                from_index: 1,
                to_index: 2,
            },
            crate::common::Edge {
                from_index: 2,
                to_index: 3,
            },
            crate::common::Edge {
                from_index: 3,
                to_index: 0,
            },
        ],
    };
    let petgraph = convert_to_petgraph(&graph);
    println!("{:?}", petgraph);
    let path = shortest_path(&petgraph, 0, 2);
    println!("{:?}", path);
}