# Release V1 breaking changes

Release V1 establishes two public Rust front doors in the existing `oss/sdk`
repository, both at `0.1.0-alpha`:

- `radroots` is the curated ordinary-user facade.
- `radroots_sdk` is the advanced host-composition API.

The release deliberately removes the version-suffixed runtime-contract crate,
the Rust and TypeScript event-index binding packages, SDK-private runtime and
store wrappers, CLI generator ownership, prefixed SDK aliases, and sibling
source consumption. Runtime wire DTOs now come from
`radroots_protocol::runtime::v1`; projection/index behavior comes from
`radroots_storage`; operational listing planning and validation are owned by
`radroots_sdk::listing`.

See [`sdk-native-api-migration.md`](sdk-native-api-migration.md) for exact Rust
path replacements. There is no compatibility package, deprecated alias, dual
schema, or transitional source path.

Both public packages are enabled only for package-realistic validation. Actual
crates.io publication remains blocked until the approval packet is complete
and a separate operator action is explicitly authorized.
