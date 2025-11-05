mod github;
mod db;
mod graph;
mod path;

use anyhow::{Context, Result};
use db::load_starmap;
use github::ensure_c3e6_dataset;
use graph::build_graph;
use path::optimal_route;
use std::env;

fn main() -> Result<()> {
    // Read starting system name from CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <SYSTEM_NAME>", args[0]);
        eprintln!("Example: {} \"P:STK3\"", args[0]);
        std::process::exit(1);
    }
    let start_name = &args[1];

    let dataset_path = ensure_c3e6_dataset()
        .context(" failed to ensure local copy of latest evefrontier_datasets (c3e6.db) ")?;
    println!("Using dataset: {}", dataset_path.display());

    let (systems, jumps) = load_starmap(&dataset_path)?;
    println!("Loaded {} systems and {} jumps.", systems.len(), jumps.len());

    let graph = build_graph(&systems, &jumps);

    let start_idx = systems
        .iter()
        .position(|s| &s.name == start_name)
        .with_context(|| format!("Start system '{}' not found in dataset", start_name))?;

    let route = optimal_route(&graph, start_idx);

    println!();
    println!("Optimal gate route starting and ending at {}:", start_name);
    println!("(Total jumps: {})", route.len().saturating_sub(1));
    println!();

    for (i, idx) in route.iter().enumerate() {
        let sys = &systems[*idx];
        let dup = if route[..i].contains(idx) { " D" } else { "" };
        println!("<a href=\"showinfo:5//{}\">{}</a>{}", sys.id, sys.name, dup);
    }

    Ok(())
}
