use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to data DB file (overrides env var and default)
    #[arg(long)]
    data_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download or ensure the dataset
    Download {
        #[arg(long)]
        force: bool,
    },
    /// Compute route starting at SYSTEM_NAME
    Route {
        start: String,
    },
}

fn resolve_data_path(cli: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli { return p; }
    if let Ok(env) = std::env::var("EVEFRONTIER_DATA_DIR") {
        return PathBuf::from(env);
    }
    if let Some(proj) = directories::ProjectDirs::from("com", "evefrontier", "evefrontier") {
        return proj.data_dir().join("static_data.db");
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local")
        .join("evefrontier")
        .join("static_data.db")
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let data_path = resolve_data_path(cli.data_dir.clone());

    match cli.command {
        Commands::Download { force: _ } => {
            let db = evefrontier_lib::ensure_c3e6_dataset(Some(&data_path))?;
            println!("Downloaded/Using: {}", db.display());
        }
        Commands::Route { start } => {
            let (systems, jumps) = evefrontier_lib::load_starmap(&data_path)?;
            let graph = evefrontier_lib::build_graph(&systems, &jumps);
            let start_idx = systems
                .iter()
                .position(|s| s.name == start)
                .ok_or_else(|| anyhow::anyhow!("Start system '{}' not found", start))?;
            let route = evefrontier_lib::optimal_route(&graph, start_idx);
            for idx in route {
                let s = &systems[idx];
                println!("{} (id={})", s.name, s.id);
            }
        }
    }

    Ok(())
}
