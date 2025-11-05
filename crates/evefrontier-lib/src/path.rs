use crate::graph::StarGraph;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;

pub fn optimal_route(graph: &StarGraph, start_idx: usize) -> Vec<usize> {
    let node_count = graph.node_count();
    let mut visited = vec![false; node_count];
    let mut route: Vec<usize> = Vec::new();

    let start_node = NodeIndex::new(start_idx);
    let mut current = start_node;
    route.push(current.index());
    visited[current.index()] = true;

    loop {
        let mut best_path: Option<(f32, Vec<NodeIndex>)> = None;

        for target in graph.node_indices() {
            if visited[target.index()] {
                continue;
            }

            if let Some((cost, path)) = astar(
                graph,
                current,
                |n| n == target,
                |_| 1.0_f32,
                |_| 0.0_f32,
            ) {
                match &best_path {
                    None => best_path = Some((cost, path)),
                    Some((best_cost, _)) if cost < *best_cost => {
                        best_path = Some((cost, path));
                    }
                    _ => {}
                }
            }
        }

        match best_path {
            Some((_cost, path)) => {
                for node in path.iter().skip(1) {
                    let idx = node.index();
                    route.push(idx);
                    visited[idx] = true;
                }
                current = *path.last().unwrap();
            }
            None => break,
        }
    }

    if current != start_node {
        if let Some((_cost, back)) = astar(
            graph,
            current,
            |n| n == start_node,
            |_| 1.0_f32,
            |_| 0.0_f32,
        ) {
            for node in back.iter().skip(1) {
                route.push(node.index());
            }
        }
    }

    route
}
