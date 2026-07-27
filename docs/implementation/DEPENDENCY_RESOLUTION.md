# Dependency resolution

This standalone repository owns its `Cargo.lock`. Under `RCRV1-DEV-001`, the
release-v1 refactor does not combine it with the core-library lockfile or make
either repository depend on the other's workspace state.

Step 017 repaired the stale lock through Cargo's minimal existing-lock
resolution. It added 12 missing transitive package records, added the new
`radroots` workspace package, and refreshed dependency lists without upgrading
existing locked package versions. The resulting checksum is
`422787033afa12d00be7a402f6772bfed194ef420320189b517ca151765eae9c`.
Repeated locked architecture tests leave it unchanged.

The repaired lock lets `cargo test --workspace --locked` reach compilation. It
currently stops in the private preview wrapper `radroots_replica_sync_wasm`:
that crate imports `RadrootsReplicaIngestOutcome` without enabling the
`radroots_replica_sync/legacy-ingest` feature that gates the type. This is a
source/feature boundary failure, not lockfile drift, and remains owned by a
later crate-surface checkpoint. It must not be described as a green workspace
lane until repaired or retired by the final architecture.

Dependency changes must use repository-owned extbuild commands, preserve
`--locked` zero-diff validation, and update this evidence when the resolved
graph intentionally changes.
