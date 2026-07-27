# Local dependency overrides

The SDK release manifests resolve packages owned by `radrootslabs/lib` through
their registry identities and exact migration-train versions. Production
`Cargo.toml` files must not contain sibling-repository paths or Git overrides.

The checked-in `.cargo/config.toml` supplies path patches only for development
inside a coordinated checkout before the initial packages exist in a registry.
Cargo patches do not alter packaged manifests. Package-realistic validation
must run the extracted `.crate` archives outside this repository so the local
configuration cannot satisfy or conceal a registry dependency.

These patches are temporary migration infrastructure. Remove them once all
lower packages are available to clean consumers from the qualification
registry. Do not add an application, build script, generated package, or public
crate dependency to this override surface.
