#!/usr/bin/env node
const fs = require('fs');
const { spawnSync } = require('child_process');

function nxIsConfigured() {
  try {
    const cfg = JSON.parse(fs.readFileSync('nx.json', 'utf8'));
    return (
      cfg &&
      cfg.tasksRunnerOptions &&
      cfg.tasksRunnerOptions.default &&
      cfg.tasksRunnerOptions.default.runner
    );
  } catch (e) {
    return false;
  }
}

if (!nxIsConfigured()) {
  console.log('Nx runner not configured; skipping nx affected step.');
  process.exit(0);
}

// Run nx affected format then lint
const run = (args) => {
  const res = spawnSync('npx', ['nx', ...args], { stdio: 'inherit' });
  if (res.error || res.status !== 0) {
    process.exit(res.status || 1);
  }
};
// Run outdated checks across all projects (cargo audit for crates, pnpm outdated at root)
run(['run-many', '--target=outdated', '--all']);

// Then run affected format and lint as before
run(['affected', '--target=format', '--base=main', '--head=HEAD']);
run(['affected', '--target=lint', '--base=main', '--head=HEAD']);
