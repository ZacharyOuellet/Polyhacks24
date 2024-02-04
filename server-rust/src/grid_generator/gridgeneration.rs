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

    nodes.push(create_random_node(0));
    nodes.push(create_random_node(1));
    edges.push(Edge {
        from_index: 0,
        to_index: 1,
    });

    for new_node_i in 2..10 {
        println!("New node: {}", new_node_i);
        nodes.push(create_random_node(new_node_i));
        let mut available_edges: Vec<Edge> = Vec::new();
        
        for other_node_i in 0..nodes.len()-1 {
            if new_node_i == other_node_i {
                continue;
            }
            let mut intersects = false;
            for edge in edges.iter() {
                println!("  Checking edge: {:?}", edge);
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
                    println!("    t: {}, u: {}", t, u);
                    if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
                        println!("    Intersects: ({:?},{:?}) ({:?},{:?})", point11, point12, point21, point22);
                        intersects = true;
                        break; // Lines intersect
                    }
                }
            }
            println!("  Done checking, intersects? {}", intersects);
            if !intersects {
                let available = Edge {
                    from_index: new_node_i,
                    to_index: other_node_i,
                };
                println!("  adding {:?}", available);

                available_edges.push(available);
            }
        }
        println!("Available edges: {:?}", available_edges);
        for _ in 0..3 {
            // Do something in the loop
            if available_edges.last().is_none() {
                break;
            }
            let mut rng = thread_rng();
            available_edges.shuffle(&mut rng);
            let rand_edge = available_edges.pop().unwrap();
            edges.push(rand_edge);
        }
    }

    Ok(Json(Graph {
        nodes: nodes,
        edges: edges,
    }))
}

pub fn check_if_edge_is_valid(edges: &Vec<Edge>, nodes: &Vec<Node>) -> bool {
    // Check if the edge is valid
    // Return true if valid, false otherwise
    let mut is_valid: bool = true;
    let mut edge: Edge = Edge {
        from_index: nodes.len(),
        to_index: nodes.len(),
    };

    for i in 0..nodes.len()-1 {
        if nodes.get(nodes.len() - 1).unwrap().x < nodes.get(i).unwrap().x {
            edge = Edge {
                from_index: nodes.len() - 1,
                to_index: i,
            };
        } else {
            edge = Edge {
                from_index: i,
                to_index: nodes.len() - 1,
            };
        }

        for possible_collision in edges.iter() {
            let point11 = nodes.get(edge.from_index).unwrap();
            let point12 = nodes.get(edge.to_index).unwrap();
            let slope1 = (point12.y - point11.y) / (point12.x - point11.x);

            let point21 = nodes.get(possible_collision.from_index).unwrap();
            let point22 = nodes.get(possible_collision.to_index).unwrap();
            let slope2 = (point22.y - point21.y) / (point22.x - point21.x);

            if might_touch(&edge, &possible_collision, &nodes) {
                if (slope1 > 0.0 && slope2 > 0.0) || (slope1 < 0.0 && slope2 < 0.0)
                {
                    is_valid = true;
                } else {
                    is_valid = false;
                }
            }
        }
    }
    is_valid
}

pub fn might_touch(edge1: &Edge, edge2: &Edge, nodes: &Vec<Node>) -> bool {
    let mut touches = false;

    let point11 = nodes.get(edge1.from_index).unwrap();
    let point12 = nodes.get(edge1.to_index).unwrap();

    let point21 = nodes.get(edge2.from_index).unwrap();
    let point22 = nodes.get(edge2.to_index).unwrap();

    let max_x1 = point11.x.max(point12.x);
    let min_x1 = point11.x.min(point12.x);

    let max_x2 = point21.x.max(point22.x);
    let min_x2 = point21.x.min(point22.x);

    if max_x1 < min_x2 || max_x2 < min_x1 {
        let max_y1 = point11.y.max(point12.y);
        let min_y1 = point11.y.min(point12.y);

        let max_y2 = point21.y.max(point22.y);
        let min_y2 = point21.y.min(point22.y);

        if max_y1 < min_y2 || max_y2 < min_y1 {
            touches = true;
        }
    }
    touches
}

pub fn create_random_node(id:usize) -> Node {
    Node {
        id: id,
        x: thread_rng().gen_range(0.0..1.0),
        y: thread_rng().gen_range(0.0..1.0),
    }
}
