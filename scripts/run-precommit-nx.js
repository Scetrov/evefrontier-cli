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

run(['affected', '--target=format', '--base=main', '--head=HEAD']);
run(['affected', '--target=lint', '--base=main', '--head=HEAD']);
