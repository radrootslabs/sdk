# Swift SDK FFI bindings

`radroots_sdk_ffi` is the private Rust source of truth. Generate Swift source,
the C header/module map, and `source.lock` from the SDK root with:

```sh
cargo extbuild run -- cargo xtask generate bindings swift
```

The generator builds the FFI library under the extbuild-owned target root,
extracts UniFFI metadata, writes deterministic text artifacts under
`generated/swift`, and binds every output hash to the exact FFI source hash.
Mobile keychain prompts, background execution, scheduling, and presentation
models are deliberately absent and remain owned by the Apple host.
