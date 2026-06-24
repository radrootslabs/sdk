pub use radroots_events_indexed as upstream;

pub const TYPE_EXPORTS: &[&str] = &[
    "RadrootsEventsIndexedShardId",
    "RadrootsEventsIndexedIdRange",
    "RadrootsEventsIndexedShardMetadata",
    "RadrootsEventsIndexedManifest",
    "RadrootsEventsIndexedShardCheckpoint",
    "RadrootsEventsIndexedIndexCheckpoint",
];

#[cfg(test)]
mod tests {
    use super::TYPE_EXPORTS;

    #[test]
    fn exports_indexed_manifest_and_checkpoint_types() {
        assert!(TYPE_EXPORTS.contains(&"RadrootsEventsIndexedManifest"));
        assert!(TYPE_EXPORTS.contains(&"RadrootsEventsIndexedIndexCheckpoint"));
        assert!(TYPE_EXPORTS.contains(&"RadrootsEventsIndexedShardId"));
    }
}
