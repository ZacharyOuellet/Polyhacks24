use crate::common::{Graph, Node};

use petgraph::graph::{Graph as PetGraph, UnGraph};

pub fn convert_to_petgraph(graph: &Graph) -> UnGraph<Node, f32> {
    let mut petgraph = PetGraph::new_undirected();
    let mut node_indices = Vec::new(); 

    for node in graph.nodes.iter() {
        let node_clone = Node {id:node.id, x: node.x, y: node.y };
        let i = petgraph.add_node(node_clone); 
        node_indices.push(i);
    }

    for edge in &graph.edges {
        let from_index = node_indices[edge.from_index];
        let to_index = node_indices[edge.to_index];

        let distance = ((graph.nodes[edge.from_index].x - graph.nodes[edge.to_index].x).powi(2) + (graph.nodes[edge.from_index].y - graph.nodes[edge.to_index].y).powi(2)).sqrt();
        petgraph.add_edge(from_index, to_index, distance); // Use the node indices to add the edge
    }

    petgraph
}

