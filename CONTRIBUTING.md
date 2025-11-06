# Contributing

Thank you for contributing! This document explains how to set up a local development environment,
run the standard checks, and contribute changes to the repository.

Environment

1. Install Rust (recommended: rustup)

- Install `rustup` from the official site: <https://rustup.rs>

- Pin a stable toolchain for the repository (example):

  rustup toolchain install stable rustup override set stable

2. Install Node.js and pnpm (for developer tooling)

- Use a tool like `volta` or `nvm` to manage Node versions.
- Install `pnpm` (recommended):

  npm install -g pnpm

3. Optional developer tools

- Install `cargo-audit` for dependency scanning:

  cargo install cargo-audit

Local checks

- Build the Rust workspace:

  cargo build --workspace

- Run tests:

  cargo test --workspace

- Run Rust lints:

  cargo clippy --all-targets --all-features -- -D warnings

- Format and lint markdown:

  pnpm install pnpm run lint:md

- Format code and docs:

  pnpm run format

Pre-commit and CI

- The project uses `husky` and `lint-staged` for local pre-commit hooks. These run quick formatters
  on staged files. Keep hooks fast — heavy checks run in CI.
- CI should run the full matrix: `cargo test`, `clippy`, `cargo audit` (or nightly),
  `pnpm run lint:md`, and any additional integration tests.

Signing and releases

- For release artifacts, prefer signing binaries or release archives using a dedicated signing
  process (GPG or `cosign`), and publish attestations for the important pipeline steps
  (build/test/scan).

Notes

- If you need deterministic runs for tests that use the dataset downloader, call
  `ensure_c3e6_dataset(Some(path))` to provide a fixed dataset path.
- If you don't want to install Node locally, you can still build and test the Rust crates; however
  markdown linting and other Node-based tooling won't run until Node/pnpm is available.

Thanks for helping improve the project. If you add new developer-facing tools, please update
`CONTRIBUTING.md` and `docs/adrs/0006-software-components.md`.
