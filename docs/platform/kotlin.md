# Kotlin SDK FFI bindings

Kotlin uses the same private `radroots_sdk_ffi` contract as Swift. Generate the
Kotlin/JNA source and `source.lock` from the SDK root with:

```sh
cargo extbuild run -- cargo xtask generate bindings kotlin
```

The repository currently owns no Android or Kotlin build. Accordingly, the
release gate validates deterministic generation, exact source/output hashes,
and the versioned DTO schema inventory only. A future Android host must add its
own JNA/runtime packaging and compile lane; it must not create a parallel
product engine or move keychain, background, scheduling, or presentation
responsibilities into this binding crate.
