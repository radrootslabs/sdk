# Crates.io publication freeze

Crates.io upload remains frozen for the complete release-v1 crate refactor.
Step 305 enabled validation metadata for exactly `radroots_sdk` and `radroots`:

```toml
publish = ["crates-io"]
```

`contracts/releases/publication.toml` is the machine authority. `cargo xtask
check` rejects an unexpected registry, package, order, version, or enablement
checkpoint; every other workspace package remains private.

This validation-only state permits packaging, crates.io dry-runs, and local
ephemeral-registry qualification. It does not authorize upload or any crates.io
mutation.

Changing the freeze requires an independently reviewed release-control commit.
Actual publication, tag creation, registry ownership changes, and
trusted-publisher changes always require separate explicit authorization.
