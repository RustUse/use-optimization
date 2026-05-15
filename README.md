# use-optimization

Composable primitive optimization utilities for Rust.

`use-optimization` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-color`, `use-text`, `use-time`, and `use-units`.
It groups small, focused crates for objective evaluation, bounds checking,
search-space generation, scoring, loss functions, and simple optimization
strategies.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `f64` helpers over framework-style abstractions
- dependencies stay minimal so each crate is easy to audit and adopt

## Workspace crates

- `use-optimization`: thin facade crate that reexports the full optimization workspace
- `use-objective`: objective direction and objective value helpers
- `use-constraint`: bounds and simple constraint checks
- `use-search-space`: one-dimensional range and linspace helpers
- `use-score`: simple scoring, ranking, and normalization helpers
- `use-loss`: common error and loss helpers for `f64` slices
- `use-grid-search`: primitive one-dimensional and two-dimensional grid search helpers
- `use-local-search`: minimal hill-climbing style local search for one dimension

## Facade crate

If you want one dependency for the whole workspace, use `use-optimization`. It
reexports each focused crate and exposes the focused APIs directly so this works:

```rust
use use_optimization::*;

let direction = ObjectiveDirection::Minimize;
let best = best_value(&[4.0, 2.0, 3.0], direction).unwrap();
let bounds = Bounds {
    min: Some(0.0),
    max: Some(10.0),
};

assert_eq!(best, 2.0);
assert!(bounds.contains(5.0));
assert_eq!(absolute_error(3.0, 2.5), 0.5);
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
optimization surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
