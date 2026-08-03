# SDK package conformance

`radroots_sdk` is qualified as a standalone advanced-host package with the
repository-owned checks below. Run every command through the configured build
output router.

The package matrix contains no-default, default, every public feature in
isolation, `native`, `full`, and all-features, always with all targets. Strict
Clippy covers the no-default and all-feature endpoints. Package tests cover
safe defaults, lifecycle, product planning and commits, native errors, public
API shape, dependency boundaries, and feature law. Rustdoc is checked and
tested with all features.

The clean-host smoke command is:

```sh
cargo xtask smoke sdk-rust-local
```

It creates a temporary external Cargo application, depends on `radroots_sdk`
through its package path, supplies migration-only local patches for the 17
lower public packages, and compiles against only the final `ClientBuilder`,
`ErrorKind`, and fail-closed construction surface. It does not rely on a
workspace member, private SDK module, legacy feature, or compatibility alias.
Package-realistic registry and extracted-crate qualification remains owned by
the later release-validation sequence while publication is frozen.

`cargo xtask architecture-ci` statically validates the same exact feature
vocabulary and activation graph. The public API tests reject SDK-owned host
traits, prefixed native types, public native struct fields, private lower
package dependencies, broad root reexports, and implementation-type leakage.
