use std::convert::Infallible;

use crate::common::{Edge, Node, Graph};
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use axum::Json;
#[derive(Serialize, Deserialize)]
struct Data {
    node_json: serde_json::Value,
    edge_json: serde_json::Value,
}

pub async fn generate_grid() -> Result<Json<Graph>, Infallible> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();


    let n_nodes = 1000;

    let mut connected_to_n: Vec<usize> = vec![0; n_nodes];

    nodes.push(create_random_node(0));
    nodes.push(create_random_node(1));
    edges.push(Edge {
        from_index: 0,
        to_index: 1,
    });

    for new_node_i in 2..n_nodes {
        nodes.push(create_random_node(new_node_i));
        let mut available_edges: Vec<Edge> = Vec::new();
        
        for other_node_i in 0..nodes.len()-1 {
            if new_node_i == other_node_i {
                continue;
            }
            let mut intersects = false;
            for edge in edges.iter() {
                if edge.from_index == new_node_i || edge.to_index == new_node_i {
                    continue;
                }
                let mut point11 = nodes.get(new_node_i).unwrap();
                let mut point12 = nodes.get(other_node_i).unwrap();

                if point11.x > point12.x {
                    std::mem::swap(&mut point11, &mut point12);
                }
                
                let mut point21 = nodes.get(edge.from_index).unwrap();
                let mut point22 = nodes.get(edge.to_index).unwrap();
                if point21.x > point22.x {
                    std::mem::swap(&mut point21, &mut point22);
                }

                let x_diff1 = point12.x - point11.x;
                let y_diff1 = point12.y - point11.y;
                let x_diff2 = point22.x - point21.x;
                let y_diff2 = point22.y - point21.y;
                let determinant = x_diff1 * y_diff2 - x_diff2 * y_diff1;

                if determinant != 0.0 {
                    // Lines are not parallel
                    let x_diff3 = point21.x - point11.x;
                    let y_diff3 = point21.y - point11.y;

                    let t = (x_diff3 * y_diff2 - x_diff2 * y_diff3) / determinant;
                    let u = (x_diff1 * y_diff3 - x_diff3 * y_diff1) / -determinant;
                    if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
                        intersects = true;
                        break; // Lines intersect
                    }
                }
            }
            if !intersects {
                let available = Edge {
                    from_index: new_node_i,
                    to_index: other_node_i,
                };

                available_edges.push(available);
            }
        }
        available_edges.sort_by(|a, b| connected_to_n.get(b.to_index).cmp(&connected_to_n.get(a.to_index)));
        let n_edges = rand::thread_rng().gen_range(1..=3);
        for _ in 0..n_edges {
            // Do something in the loop
            if available_edges.is_empty() {
                break;
            }
            let rand_edge = available_edges.pop().unwrap();
            *connected_to_n.get_mut(rand_edge.from_index).unwrap() += 1;
            *connected_to_n.get_mut(rand_edge.to_index).unwrap() += 1;
            edges.push(rand_edge);
        }
    }

    Ok(Json(Graph {
        nodes: nodes,
        edges: edges,
    }))
}

pub fn create_random_node(id:usize) -> Node {
    Node {
        id: id,
        x: thread_rng().gen_range(0.0..1.0),
        y: thread_rng().gen_range(0.0..1.0),
    }
}
