# Dependency checks via Nx

You can run a pnpm outdated check via the root Nx project target:

```pwsh
pnpm exec nx run evefrontier-pathfinder:outdated
```

If you'd prefer a pretty report that fails with a readable JSON payload, use:

```pwsh
pnpm run outdated:report
```

Both commands are used by the repository pre-commit flow.
