# use-optimization

Composable facade crate for RustUse optimization primitives.

## Install

```toml
[dependencies]
use-optimization = "0.0.1"
```

## What it reexports

`use-optimization` reexports `use-objective`,
`use-optimization-constraint`, `use-search-space`, `use-score`, `use-loss`,
`use-grid-search`, and `use-local-search`.

## When to use it

Use this crate when you want one dependency for the full optimization workspace.
Depend on the focused crates directly when you only need a smaller slice.

## Status

This crate is experimental and may evolve while the workspace remains below
`0.3.0`.
