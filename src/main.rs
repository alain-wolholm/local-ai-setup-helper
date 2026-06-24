//! Thin `shardsmith` binary stub.
//!
//! Intentionally minimal scaffolding for issue `shardsmith-pfc.1`: it links the
//! platform-neutral [`shardsmith`] library, prints its version, and exits
//! cleanly. Scanning behavior arrives in later slices.

fn main() {
    println!("shardsmith {}", shardsmith::version());
}
