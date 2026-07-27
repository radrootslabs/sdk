# Dependency resolution

This standalone repository owns its `Cargo.lock`. Under `RCRV1-DEV-001`, the
release-v1 refactor does not combine it with the core-library lockfile or make
either repository depend on the other's workspace state.

Step 017 repaired the stale lock through Cargo's minimal existing-lock
resolution. It added 12 missing transitive package records, added the new
`radroots` workspace package, and refreshed dependency lists without upgrading
existing locked package versions. The resulting checksum is
`422787033afa12d00be7a402f6772bfed194ef420320189b517ca151765eae9c`.

Step 020 repaired the private preview/test feature boundary by explicitly
enabling `radroots_replica_sync/legacy-ingest` only for the existing wrapper and
SDK development test that consume that gated API. Cargo refreshed only the
local package dependency list; no package version changed. The resulting
checksum is
`246de979d4b2b249182860c4b0adc37fc90c11143c3a279c49201227501d84d9`.
The full locked workspace check, test, Clippy, and rustdoc lanes now pass.

Dependency changes must use repository-owned extbuild commands, preserve
`--locked` zero-diff validation, and update this evidence when the resolved
graph intentionally changes.
