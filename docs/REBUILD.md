# Rebuild from scratch — quick recipe

This document gives a minimal reproducible set of steps to rebuild the project from a fresh machine.
It intentionally stays short so you can redesign the spec and expand later.

Prerequisites

- Rust toolchain: see `.rust-toolchain` in repository root.
- Node.js: see `.nvmrc` in repository root (used for developer tooling only).

Steps

1. Install Rust via `rustup` and ensure the toolchain is the one in `.rust-toolchain`:

   rustup toolchain install $(cat .rust-toolchain) rustup override set $(cat .rust-toolchain)

2. Install Node.js matching `.nvmrc` (or use `volta`/`nvm`):

   # Example using nvm

   nvm install $(cat .nvmrc) nvm use $(cat .nvmrc)

3. Install developer dependencies (Node) and run format/lint tools:

   pnpm install pnpm run lint:md pnpm run format

4. Build and test the Rust workspace:

   cargo build --workspace cargo test --workspace

Notes

- If the dataset is required for specific tests, use `ensure_c3e6_dataset(Some(path))` in tests or
  provide the dataset via environment/config when running the CLI.
- See `docs/DEPENDENCIES.md` and `CONTRIBUTING.md` for more setup details.
