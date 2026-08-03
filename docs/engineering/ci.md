# Architecture continuous integration

The pull-request architecture lane is a thin GitHub adapter over the
repository-owned dispatcher:

```sh
cargo xtask architecture-ci
```

The command validates the synchronized release specification, workspace and
package metadata, production dependency paths, the Cargo-resolved package-tier
graph, public API implementation leakage, SDK feature boundaries, publication
freeze, facade conformance, language contracts, and generated-source freshness.

Facade conformance pins its exact dependency and feature-forwarding graphs,
curated modules and root exports, absence of new facade-owned traits and
wildcard reexports, explicit ordinary example, and compiled rejection of a
`radroots::sdk` namespace. Clean temporary consumers are executable through
`cargo xtask smoke front-doors-rust-local`.

SDK feature qualification additionally runs no-default, default, each of the
eleven public features in isolation, the `native` and `full` bundles, and
all-features. Every lane uses `--all-targets`; strict Clippy mirrors the
no-default and all-feature endpoints. Package-boundary tests reject any feature
outside `memory`, `sqlite`, `sync`, `nostr`, `nip46`, `local-signing`,
`radrootsd`, `geonames`, `knowledge`, `native`, and `full`, and verify that
optional dependencies are activated through explicit `dep:` entries.
The workflow invokes the same dispatcher with `cargo run --locked` so lockfile
drift fails rather than being resolved implicitly.

Until the lower public packages are available from the registry, the checked-in
developer patch configuration requires a coordinated `radrootslabs/lib`
checkout. The workflow pins that public source to
`bb9832fa4c33f68b4262140599c111abb5d5480d`; update the pin only after a
replacement commit is publicly reachable and passes the library architecture
lane. This checkout does not alter any production dependency declaration.

The workflow grants only read access to repository contents. Action
dependencies are pinned to full commit identifiers. It caches only Cargo
registry and Git downloads, keyed by both repositories' lockfiles and governed
toolchains; generated outputs and build artifacts are never restored from the
cache.

Repository administrators may require the `Architecture / architecture`
status after the pinned library commit and this workflow commit are publicly
reachable. Changing branch protection or other repository administration
remains a separate authorized operation.
