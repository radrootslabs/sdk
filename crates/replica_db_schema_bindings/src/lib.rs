pub use radroots_replica_db_schema as upstream;

pub const TYPES_TS: &str = include_str!("typescript/types.ts");

#[cfg(test)]
mod tests {
    use super::TYPES_TS;

    #[test]
    fn preserves_replica_schema_exports() {
        assert!(TYPES_TS.contains("export type Farm"));
        assert!(TYPES_TS.contains("export type GcsLocation"));
        assert!(TYPES_TS.contains("export type IGcsLocationFindManyResolve"));
    }
}
