pub use radroots_replica_db_schema as upstream;

mod model;

pub use model::types_module;

#[cfg(test)]
mod tests {
    use super::types_module;

    #[test]
    fn preserves_replica_schema_exports() {
        let rendered = types_module().render();
        assert!(rendered.contains("export type Farm"));
        assert!(rendered.contains("export type GcsLocation"));
        assert!(rendered.contains("export type IGcsLocationFindManyResolve"));
    }
}
