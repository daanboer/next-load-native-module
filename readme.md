# Issue

Next.js (turbopack) fails to load/find native node modules compiled as part of
a pnpm workspace.

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

The `web-app` build process will throw the following error:

```
  Error occurred prerendering page "/". Read more: https://nextjs.org/docs/messages/prerender-error
  TypeError: (void 0) is not a function
      at c (.next/server/chunks/ssr/[root-of-the-server]__9aa9e4ec._.js:1:392)
      at stringify (<anonymous>) {
    digest: '3868626329'
  }
  Export encountered an error on /page: /, exiting the build.
```
