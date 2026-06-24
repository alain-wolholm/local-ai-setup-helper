//! Shardsmith scanner core (platform-neutral).
//!
//! This crate holds the platform-neutral domain model and orchestration for the
//! Shardsmith scanner. Per ADR-0001 (keep the scanner core platform-neutral),
//! operating-system- and vendor-specific collectors live at the boundary and
//! must not leak into this core.
//!
//! This is scaffolding for issue `shardsmith-pfc.1`: there is no domain model
//! and no scanning behavior yet, and deliberately **no native/platform-specific
//! collector code**. Later foundation slices build on this platform-neutral
//! base; native collectors arrive behind the boundary at that point.

/// The scanner's version string, sourced from the package manifest.
///
/// Exposed so the binary (and later tooling) can report a single source of
/// truth rather than hard-coding a version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_manifest() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn version_is_not_empty() {
        assert!(!version().is_empty());
    }
}
