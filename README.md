# Shardsmith

Shardsmith describes a heterogeneous local machine as observable compute
resources whose usefulness must be established through **evidence** rather than
assumed from specifications.

The **Scanner** produces an evidence-backed snapshot of the local compute
environment. It separates passive discovery from active verification and makes
no performance or workload-suitability claims.

## Status

Early scaffolding. This is the Rust project foundation: a platform-neutral
library crate plus a thin `shardsmith` binary stub, with a Windows CI baseline.
Domain modelling and scanning behavior arrive in later slices.

## Build

```sh
cargo build
cargo test
cargo run        # prints: shardsmith <version>
```

## Design

- The scanner core is platform-neutral; operating-system- and vendor-specific
  collectors live at the boundary and must not leak into the core.
- `unsafe` is denied package-wide and permitted only at documented native
  boundaries.
- Dependencies are minimized and audited; the scanner runs fully offline.
