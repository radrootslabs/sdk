pub use radroots_event_index as upstream;

pub const TYPE_EXPORTS: &[&str] = &[
    "RadrootsEventIndexShardId",
    "RadrootsEventIndexIdRange",
    "RadrootsEventIndexShardMetadata",
    "RadrootsEventIndexManifest",
    "RadrootsEventIndexShardCheckpoint",
    "RadrootsEventIndexCheckpoint",
];

#[cfg(test)]
mod tests {
    use super::TYPE_EXPORTS;

    #[test]
    fn exports_indexed_manifest_and_checkpoint_types() {
        assert!(TYPE_EXPORTS.contains(&"RadrootsEventIndexManifest"));
        assert!(TYPE_EXPORTS.contains(&"RadrootsEventIndexCheckpoint"));
        assert!(TYPE_EXPORTS.contains(&"RadrootsEventIndexShardId"));
    }
}
