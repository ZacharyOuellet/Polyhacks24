use rand::prelude::*;
use serde_json;
use crate::common::{Node, Edge, Graph};

pub fn generate_grid() {
        let mut nodes: Vec<(f64, f64)> = Vec::new();
        let mut lines: Vec<usize> = Vec::new();

        nodes.push((thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0)));
        for i in 0..98 {
                nodes.push((thread_rng().gen_range(0.0..1.0), thread_rng().gen_range(0.0..1.0)));
                for j in 0..i {
                        
                }
                lines.push(i);
                //take the last node na
        }

        println!("{:?}", nodes);
        println!("{:?}", lines);

        // Convert the arrays to JSON
        let nodes_json = serde_json::to_string(&nodes).unwrap_or_else(|err| panic!("Failed to convert nodes to JSON: {}", err));
        let lines_json = serde_json::to_string(&lines).unwrap_or_else(|err| panic!("Failed to convert lines to JSON: {}", err));

        // Save the JSON to files
        std::fs::write("src/data_nodes.json", &nodes_json).unwrap_or_else(|err| panic!("Failed to write nodes JSON to file: {}", err));
        std::fs::write("src/data_lines.json", &lines_json).unwrap_or_else(|err| panic!("Failed to write lines JSON to file: {}", err));

        println!("done!");
}
