# Crates.io publication freeze

Crates.io publication is frozen for the complete release-v1 crate refactor.
Every workspace package, including `radroots_sdk` and the future `radroots`
facade, must set:

```toml
publish = false
```

`contracts/releases/publication.toml` is the machine authority for this
freeze. `cargo xtask check` rejects a missing publication policy, an unexpected
registry or enablement checkpoint, and any package that is implicitly or
explicitly publishable.

The only planned exception is release plan Step 305, after the final package
inventory, resolved public dependency graph, API surface, target matrix, and
security gates are green. That checkpoint may set `publication.frozen = false`
and enable exactly `radroots_sdk` and `radroots` for package-validation
staging. It does not authorize upload or any crates.io mutation.

Changing the freeze requires an independently reviewed release-control commit.
Actual publication, tag creation, registry ownership changes, and
trusted-publisher changes always require separate explicit authorization.
