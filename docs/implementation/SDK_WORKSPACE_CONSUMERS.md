# Rust front-door canonical-workspace consumer cutover

Step 258 re-audited every Cargo package and Rust source in the standalone SDK
workspace after the final `radroots` facade became active. Cargo metadata has
one front-door dependency edge:

```text
radroots -> radroots_sdk
```

That is the normative engine composition edge. No other workspace package
depends on `radroots` or `radroots_sdk`:

- binding and WASM packages consume their final owning lower crates;
- `radroots_sdk_sql_wasm_runtime` is a distinct private implementation package,
  not a predecessor spelling or an SDK front-door consumer;
- xtask smoke sources construct clean external `radroots` and `radroots_sdk`
  hosts and intentionally exercise both public package paths;
- package READMEs and engineering documents use the final identities and do not
  create Cargo dependency edges.

The ordinary example uses only `radroots::client`; the advanced examples use
`radroots_sdk` and explicit lower capability types. There are no application
packages in this capsule to rewrite, no legacy facade package, and no temporary
compatibility dependency to retain.

CLI, Studio, FFI/mobile, daemon, and other applications are separate repository
capsules. Their migrations remain assigned to Steps 269-294; this workspace
checkpoint does not edit or claim those consumers. The external CLI still has
the previously recorded sibling-SDK path and legacy API debt in
[`SDK_SUPERSEDED_SURFACE_AUDIT.md`](SDK_SUPERSEDED_SURFACE_AUDIT.md).

The repeatable audit is:

```sh
cargo metadata --no-deps --format-version 1
rg -n 'radroots_sdk|radroots::|radroots =' --glob 'Cargo.toml' --glob '*.rs'
cargo xtask smoke front-doors-rust-local
```

The result is a deliberately narrow canonical workspace: ordinary code enters
through `radroots`; advanced host composition enters through `radroots_sdk`;
lower packages and generated bindings do not route through either front door.
