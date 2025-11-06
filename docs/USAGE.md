# EveFrontier CLI & Library — Usage

This document describes how to build and use the `evefrontier-cli` workspace and its library crate
`evefrontier-lib`.

Build

1. Build the entire workspace:

   cargo build --workspace

2. Build only the library and CLI crates:

   cargo build -p evefrontier-lib -p evefrontier-cli

Run the CLI

From the repository root you can run the CLI using `cargo run` and passing subcommands.

Examples

- Download dataset (places DB at resolved path):

  cargo run -p evefrontier-cli -- download

- Compute a route starting at a system name:

  cargo run -p evefrontier-cli -- route "P:STK3"

Configuration & data path resolution

The CLI resolves the data path in the following order:

1. CLI `--data-dir` flag (if provided)
2. `EVEFRONTIER_DATA_DIR` environment variable
3. XDG `directories::ProjectDirs` default location
4. Fallback to `~/.local/evefrontier/static_data.db`

Downloader & caching

The downloader stores cached release assets under the OS cache directory in a
`evefrontier_datasets/` subdirectory. It writes to a temporary file then atomically renames it to
the final path to avoid partial writes.

Database schema compatibility

The library supports different release schema variants by detecting the DB schema and adapting
queries. The loader supports the `static_data.db` schema (tables `SolarSystems(solarSystemId, name)`
and `Jumps(fromSystemId, toSystemId)`) and older `mapSolarSystems` schemas. If adding support for
additional schemas, update `crates/evefrontier-lib/src/db.rs` and add a small unit test in
`crates/evefrontier-lib/tests/`.

Library API

Key library entrypoints (in `crates/evefrontier-lib`):

- `ensure_c3e6_dataset(target_dir: Option<&Path>)` — download and ensure dataset is present (accepts
  optional explicit path for deterministic testing).
- Graph & path: functions in `graph.rs` and `path.rs` for building system graphs and computing
  routes.

Testing

Run unit tests across the workspace:

cargo test --workspace

Notes

- The downloader uses OS cache directories, call `ensure_c3e6_dataset(Some(path))` to control where
  the DB is placed during tests.
