# Issue

Next.js fails to load/find native node modules compiled as part of a pnpm workspace.

## How to reproduce

- Install dependencies.

```bash
pnpm i
```

- Build the `rust-lib` package (needs a working rust installation).

```bash
pnpm -C packages/rust-lib build
```

- Try to build the `web-app` package.

```bash
pnpm -C packages/web-app build
```

The `web-app` build process will throw the following error: `[cause]: Error: Cannot find module '../rust-lib/dist/index.node'`.
