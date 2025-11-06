# Reconstruction blueprint

This file is a minimal blueprint to recreate the project if the source code is deleted. It
intentionally focuses on the facts required to rebuild behaviorally equivalent artifacts (library +
CLI) rather than reproducing the exact source line-for-line.

Use this blueprint as the authoritative guide while redesigning specs and before deleting code.

1. Repository layout

- Root workspace:
  - `Cargo.toml` (workspace with members under `crates/`)
  - `crates/`
    - `evefrontier-lib/` — library crate (core logic)
      - `src/db.rs` — DB loader & schema detection
      - `src/github.rs` — optional GitHub downloader helper
      - `src/graph.rs` — graph construction from systems/jumps
      - `src/path.rs` — pathfinding algorithm(s)
      - `src/lib.rs` — public API and re-exports
    - `evefrontier-cli/` — CLI crate
      - `src/main.rs` — argument parsing and minimal glue to call library

2. Public API (library)

```
ensure_c3e6_dataset(target_dir: Option<&Path>) -> Result<PathBuf>
```

- Downloads or locates the dataset DB and returns an absolute path.
- Behavior: Accepts either a path to an existing `.db` file or downloads a release asset from
  GitHub, caching under the OS cache dir (e.g.
  `directories::BaseDirs::cache_dir()/evefrontier_datasets/`). Write-to-temp then atomically rename
  on success.

```
load_starmap(db_path: &Path) -> Result<Starmap>
```

- Loads systems and jumps into in-memory structures. `Starmap` contains:
  - systems: `HashMap<SystemId, System { id, name }>`
  - adjacency: a mapping from `SystemId` to a vector of neighbour `SystemId`s (undirected or
    directed depending on DB semantics)

```
build_graph(starmap: &Starmap) -> Graph
```

Build a graph used by the pathfinder. Use adjacency lists and choose BFS for unweighted hops or
Dijkstra if weights are introduced.

```
find_route(graph: &Graph, start: SystemId, goal: SystemId) -> Option<Vec<SystemId>>
```

Return an ordered list of system IDs (or names) that forms the route, or `None` when no path exists.

3. CLI surface

- Subcommands (minimum):
  - `download` — ensure dataset present (mirrors `ensure_c3e6_dataset`).
  - `route <SYSTEM>` — compute a route from a named system using library API.

- Configuration resolution order for data path:
  1. `--data-dir` CLI flag
  2. `EVEFRONTIER_DATA_DIR` env var
  3. XDG ProjectDirs default path
  4. Fallback `~/.local/evefrontier/static_data.db`

4. Database schema expectations & detection

- Supported schemas:
  - `static_data.db` style: tables `SolarSystems(solarSystemId, name)` and
    `Jumps(fromSystemId, toSystemId)`.
  - older style: `mapSolarSystems` and equivalent jumps tables.

- Detection approach:
  - Use `PRAGMA table_info('SolarSystems')` or query `sqlite_master` to find table names and column
    patterns. Based on results, select the appropriate SQL query set.

5. Downloader & caching

- Use the OS cache directory (via `directories` crate) to store downloaded assets under
  `evefrontier_datasets/`.
- Download behavior:
  - Download to a temporary file in the cache dir.
  - Validate the download (size / presence) and then atomically rename to the final filename.
  - If a release is a zip, extract the first `.db` that matches `*.db` or contains `c3e6` in the
    name.

6. Tests and fixtures

- Provide at least one small SQLite fixture for each supported schema. The fixture should include
  4–10 systems and a handful of jumps so pathfinding and loader logic can be exercised.
- Unit tests:
  - Loader tests validating detection and counts of systems/jumps.
  - Graph tests checking adjacency and a route-finding test (happy path).

7. Build & tooling

- Build commands:
  - `cargo build --workspace`
  - `cargo test --workspace`

- Tooling to document/pin:
  - `.rust-toolchain` with the pinned compiler (we added one in the repo).
  - `.nvmrc` or Volta config for Node used by docs tooling.

8. Release & signing

- Sign artifacts using GPG or `cosign`. Attach attestations for build/test and scan steps. Document
  the exact files and signature commands in `docs/RELEASE.md` when you redesign the spec.

9. Minimal implementation notes

- Pathfinding: start with BFS on an unweighted graph of systems (edges from `Jumps` table). If we
  later add weights, switch to Dijkstra.
- System identifiers: use `INTEGER` IDs from the DB as primary keys and maintain a
  `HashMap<i64, String>` for `id -> name`.

10. Before you delete code

- Commit and push the current tree (already done).
- Create this `RECONSTRUCT.md` (done). Keep bootstrap fixtures checked in (small `.db` files) if you
  want to recreate behavior precisely.

If you want, I can now generate minimal skeleton crates and a small fixture
`docs/fixtures/minimal_static_data.db` that follows the `static_data.db` schema to make the
reconstruction trivial. Tell me if you'd like that created before you delete the code.
