use crate::common::Graph;
use crate::pathfinding::path_types::PathNode;

use petgraph::graph::Graph as PetGraph;

pub fn convert_to_petgraph(graph: &Graph) -> PetGraph<PathNode, f32> {
    let mut petgraph = PetGraph::new();
    let mut nodes = Vec::new();
    for (index, node) in graph.nodes.iter().enumerate() {
        nodes.push(petgraph.add_node(PathNode {
            index: index,
            x: node.x,
            y: node.y,
            seen: false,
            explored: false,
            distance: 0.0,
        }));
    }
    for edge in &graph.edges {
        let distance = ((graph.nodes[edge.from_index].x - graph.nodes[edge.to_index].x).powi(2) + (graph.nodes[edge.from_index].y - graph.nodes[edge.to_index].y).powi(2)).sqrt();
        petgraph.add_edge(nodes[edge.from_index], nodes[edge.to_index], distance);
    }
    petgraph
}

pub fn test() {
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
}
