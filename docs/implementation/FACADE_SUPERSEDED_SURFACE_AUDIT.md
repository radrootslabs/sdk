# Facade superseded-surface audit

Step 260 searched every checked-out first-party OSS capsule before the first
`radroots` crate release. No earlier facade crate, facade experiment, public
`radroots::sdk` namespace, or wildcard SDK reexport exists in canonical source.
There is therefore no compatibility package or module to retain, deprecate, or
publish.

## Canonical SDK capsule

The only package named `radroots` in this workspace is `crates/radroots`. Its
manifest is enabled only for validation staging after Step 305, its exact
dependency and feature graphs are governed, and its source is limited to the curated
ordinary-user modules. The only advanced engine edge is its private Cargo
dependency on `radroots_sdk`; the facade does not duplicate engine logic.

The release policy approves only `radroots_sdk` and `radroots` for eventual
package validation. Every other local Cargo package remains private, and the
architecture gate rejects additional publishable identities. No deprecation
placeholder package exists.

## Other checked-out OSS capsules

Before its Step 313 cutover, the separate CLI capsule used Cargo package/binary
name `radroots`. That was an application identity, not a facade experiment,
but its Cargo package identity conflicted with the new library front door.
Steps 269-272 migrated the consumer, Step 294 qualified it, and Step 313
removed the final source branch without introducing a shim.

The daemon, Apple/mobile, Studio, web, and other checked-out capsules contain
no alternate Rust facade package or `radroots::sdk` namespace. Their real
consumer migrations completed in Steps 269-294.

`oss/.sdk_step064_worktree` is an untracked local recovery checkout, not a
canonical repository, workspace member, release input, or compatibility
surface. It was intentionally left untouched and excluded from conclusions
about releasable source.

## Repeatable gate

From the OSS parent, search canonical capsule source for:

```sh
rg -n 'radroots[_-](facade|client)|radroots::sdk|pub use radroots_sdk::\*|name = "radroots"' \
  oss --glob 'Cargo.toml' --glob '*.rs' --glob '*.md' --glob '*.toml'
```

Then run the SDK capsule's workspace tests and `cargo xtask architecture-ci`.
Expected matches are limited to the final facade, normative specifications and
guards, and clean-smoke fixture names.
Any new package or namespace match blocks release until it is removed or an
explicit later migration step owns it.
