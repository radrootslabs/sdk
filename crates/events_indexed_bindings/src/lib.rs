pub use radroots_events_indexed as upstream;

use radroots_sdk_binding_model as ts;

pub fn types_module() -> ts::TsModule {
    ts::module(vec![
        ts::type_alias("RadrootsEventsIndexedShardId", ts::string()),
        ts::type_alias(
            "RadrootsEventsIndexedIdRange",
            ts::object(vec![
                ts::field("start", ts::string()),
                ts::field("end", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsEventsIndexedShardMetadata",
            ts::object(vec![
                ts::field("file", ts::string()),
                ts::field("count", ts::number()),
                ts::field("first_id", ts::string()),
                ts::field("last_id", ts::string()),
                ts::field("first_published_at", ts::number()),
                ts::field("last_published_at", ts::number()),
                ts::field("sha256", ts::string()),
            ]),
        ),
        ts::type_alias(
            "RadrootsEventsIndexedManifest",
            ts::object(vec![
                ts::field("country", ts::string()),
                ts::field("total", ts::number()),
                ts::field("shard_size", ts::number()),
                ts::field("first_published_at", ts::number()),
                ts::field("last_published_at", ts::number()),
                ts::field(
                    "shards",
                    ts::array(ts::reference("RadrootsEventsIndexedShardMetadata")),
                ),
            ]),
        ),
        ts::type_alias(
            "RadrootsEventsIndexedShardCheckpoint",
            ts::object(vec![
                ts::field("shard_id", ts::reference("RadrootsEventsIndexedShardId")),
                ts::field("last_created_at", ts::number()),
                ts::field("last_event_id", ts::nullable(ts::string())),
                ts::field("cursor", ts::nullable(ts::string())),
            ]),
        ),
        ts::type_alias(
            "RadrootsEventsIndexedIndexCheckpoint",
            ts::object(vec![
                ts::field("generated_at", ts::number()),
                ts::field(
                    "shards",
                    ts::array(ts::reference("RadrootsEventsIndexedShardCheckpoint")),
                ),
            ]),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::types_module;

    #[test]
    fn exports_indexed_manifest_and_checkpoint_types() {
        let rendered = types_module().render();
        assert!(rendered.contains("export type RadrootsEventsIndexedManifest"));
        assert!(rendered.contains("export type RadrootsEventsIndexedIndexCheckpoint"));
        assert!(rendered.contains("export type RadrootsEventsIndexedShardId = string"));
    }
}
