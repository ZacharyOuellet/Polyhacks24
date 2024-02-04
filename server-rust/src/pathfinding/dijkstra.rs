use petgraph::graph::UnGraph;
use petgraph::prelude::NodeIndex;
use crate::pathfinding::path_types::PathNode;
use std::collections::BinaryHeap;
use std::path;
use crate::common::{Graph, Node};
use crate::pathfinding::graph_converter::convert_to_petgraph;


struct HeapElement {
    index: usize,
    distance: f32,
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

pub fn shortest_path(graph: &UnGraph<Node, f32>, start: usize, end: usize) -> Vec<usize> {
    let mut path_nodes = vec![PathNode {index:0,distance:f32::INFINITY, seen: false, explored: false, predecessor: 0 }; graph.node_count()];
    path_nodes[start].index = start;
    path_nodes[start].distance = 0.0;
    path_nodes[start].seen = true;
    path_nodes[start].explored = false;
    
    let mut to_explore: BinaryHeap<HeapElement> = BinaryHeap::new();
    to_explore.push(HeapElement { index: path_nodes[start].index, distance: path_nodes[start].distance });
    while !to_explore.is_empty() {
        let current = to_explore.pop().unwrap().index;
        path_nodes[current].explored = true;
        for neighbor in graph.neighbors(NodeIndex::new(current)) {
            let edge = graph.find_edge(NodeIndex::new(current), neighbor);
            let distance = path_nodes[current].distance + graph.edge_weight(edge.unwrap()).unwrap();
            let neighbor_node = &mut path_nodes[neighbor.index()];
            if distance < neighbor_node.distance {
                neighbor_node.distance = distance;
                neighbor_node.predecessor = current;
                if !neighbor_node.seen {
                    neighbor_node.seen = true;
                    to_explore.push(HeapElement { index: neighbor.index(), distance: distance });
                }
            }
        }
    }
    println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA{:?}", path_nodes);

    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        current = path_nodes[current].predecessor;
    }
    path.push(start);
    path.reverse();
    path
}

pub fn test() -> Vec<usize>{
    let graph = Graph {
        nodes: vec![
            crate::common::Node { x: 0.0, y: 0.0 },
            crate::common::Node { x: 0.1, y: 1.0 },
            crate::common::Node { x: 1.0, y: 1.0 },
            crate::common::Node { x: 0.5, y: 0.2 },
            crate::common::Node { x: 0.5, y: 0.0 },
            crate::common::Node { x: 1.0, y: 0.05 },
        ],
        edges: vec![
            crate::common::Edge {
                from_index: 0,
                to_index: 1,
            },
            crate::common::Edge {
                from_index: 0,
                to_index: 3,
            },
            crate::common::Edge {
                from_index: 0,
                to_index: 4,
            },
            crate::common::Edge {
                from_index: 4,
                to_index: 5,
            },
            crate::common::Edge {
                from_index: 5,
                to_index: 3,
            },
            crate::common::Edge {
                from_index: 3,
                to_index: 1,
            },
            crate::common::Edge {
                from_index: 1,
                to_index: 2,
            },
            crate::common::Edge {
                from_index: 2,
                to_index: 5,
            },
        ],
    };
    let petgraph = convert_to_petgraph(&graph);
    println!("{:?}", petgraph);
    let path = shortest_path(&petgraph, 3, 2);
    println!("{:?}", path);
    path
}


use axum::Json;
pub async fn placeholder(Json(graph): Json<Graph>) -> Json<Vec<usize>> {
    let conv_graph = convert_to_petgraph(&graph);
    let return_value = shortest_path(&conv_graph, 0, graph.nodes.len() - 1);
    Json(return_value)
}
