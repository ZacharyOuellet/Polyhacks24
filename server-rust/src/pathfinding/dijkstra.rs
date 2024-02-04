use petgraph::graph::UnGraph;
use petgraph::prelude::NodeIndex;
use crate::pathfinding::path_types::PathNode;
use std::collections::BinaryHeap;
use std::convert::Infallible;
use crate::common::{Node, SolutionRequest, SolutionResponse};
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

struct PathResult {
    distance: f32,
    path: Vec<usize>,
}

fn shortest_path(graph: &UnGraph<Node, f32>, start: usize, end: usize) -> PathResult {
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
    println!("Path:{:?}", path_nodes);

    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        current = path_nodes[current].predecessor;
    }
    path.push(start);
    path.reverse();
    PathResult { distance: path_nodes[end].distance, path: path }
}

use axum::Json;
pub async fn solution_handler(Json(request): Json<SolutionRequest>) -> Result<Json<SolutionResponse>,  Infallible> {
    let conv_graph = convert_to_petgraph(&request.graph);
    let driver_alone = shortest_path(&conv_graph, request.driver_start, request.driver_end);

    let get_to_passenger = shortest_path(&conv_graph, request.driver_start, request.passenger_start);
    let mut passenger_path = shortest_path(&conv_graph, request.passenger_start, request.passenger_end);
    
    passenger_path.path.pop();
    passenger_path.path.remove(0);

    let get_to_destination = shortest_path(&conv_graph, request.passenger_end, request.driver_end);
    let driver_passenger_distance = get_to_passenger.distance + passenger_path.distance + get_to_destination.distance;
    let driver_passenger_path = get_to_passenger.path.iter().chain(passenger_path.path.iter()).chain(get_to_destination.path.iter()).map(|x| *x).collect();

    let return_value = SolutionResponse {
        driver_alone: driver_alone.path,
        driver_alone_distance: driver_alone.distance,
        driver_passenger: driver_passenger_path,
        driver_passenger_distance: driver_passenger_distance,
    };

    Ok(Json(return_value))
}
