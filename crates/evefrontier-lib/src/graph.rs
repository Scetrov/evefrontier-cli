use crate::db::{Jump, System};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;

pub type StarGraph = Graph<usize, (), Undirected>;

pub fn build_graph(systems: &[System], jumps: &[Jump]) -> StarGraph {
    let mut graph = StarGraph::new_undirected();
    let mut id_to_node = HashMap::<i64, NodeIndex>::new();

    for (i, sys) in systems.iter().enumerate() {
        let idx = graph.add_node(i);
        id_to_node.insert(sys.id, idx);
    }

    for j in jumps {
        if let (Some(&a), Some(&b)) = (id_to_node.get(&j.from_id), id_to_node.get(&j.to_id)) {
            graph.update_edge(a, b, ());
        }
    }

    graph
}
