//! Platform-neutral scanner core. OS- and vendor-specific collectors live at
//! the boundary and must not leak in (ADR-0001).

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn version_comes_from_manifest() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }
}
