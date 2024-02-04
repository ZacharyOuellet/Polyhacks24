use crate::common::{Edge, Node};
use rand::prelude::*;
use serde_json;

pub fn generate_grid() {
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    nodes.push(create_random_node());

    for i in 0..98 {
        nodes.push(create_random_node());
        let mut available_nodes: Vec<usize> = Vec::new();

        for (index, node) in nodes.iter().enumerate() {
            if check_if_edge_is_valid(&edges, &nodes) {
                available_nodes.push(index);
            }
        }
        let mut k = 0;
        while k <= 5 {
            // Do something in the loop
            if available_nodes.last().is_none() {
                print!("RAWR");
                break;
            }
            let mut rng = thread_rng();
            edges.shuffle(&mut rng);
            let i2 = available_nodes.pop().unwrap();
            if(i2 != i){
            if nodes.get(i).unwrap().x < nodes.get(i2).unwrap().x {
                edges.push(Edge {
                    from_index: i,
                    to_index: i2,
                });
            } else {
                edges.push(Edge {
                    from_index: i2,
                    to_index: i,
                });
            }}
            k += 1;
        }
    }
    println!("{:?}", nodes);
    println!("{:?}", edges);

    // Convert the arrays to JSON
    let nodes_json = serde_json::to_string(&nodes)
        .unwrap_or_else(|err| panic!("Failed to convert nodes to JSON: {}", err));
    let edges_json = serde_json::to_string(&edges)
        .unwrap_or_else(|err| panic!("Failed to convert edges to JSON: {}", err));

    // Save the JSON to files
    std::fs::write("src/data_nodes.json", &nodes_json)
        .unwrap_or_else(|err| panic!("Failed to write nodes JSON to file: {}", err));
    std::fs::write("src/data_edges.json", &edges_json)
        .unwrap_or_else(|err| panic!("Failed to write edges JSON to file: {}", err));

    println!("done!");
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

pub fn create_random_node() -> Node {
    Node {
        x: thread_rng().gen_range(0.0..1.0),
        y: thread_rng().gen_range(0.0..1.0),
    }
}
