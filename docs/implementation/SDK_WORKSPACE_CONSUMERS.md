# SDK canonical-workspace consumer cutover

Step 246 evaluated every Cargo package in the standalone SDK workspace before
the ordinary-user facade was activated. Cargo metadata reports no package with
a dependency on `radroots_sdk` at this checkpoint. That zero-dependent state is
intentional:

- `radroots` remains the pulled-forward, dependency-free scaffold until Steps
  250–253 establish its curated modules, builders, and feature forwarding.
- binding and WASM packages consume their owning lower crates and do not route
  through the native advanced-host SDK.
- the xtask clean-host smoke now uses only the final `ClientBuilder` and
  `ErrorKind` surface. Its predecessor knowledge smoke, which referenced
  removed SDK features and prefixed types, is retired.
- CLI, Studio, FFI/mobile, daemon, and other application capsules are separate
  repositories/checkouts. Their ordered migrations remain assigned to Steps
  269–294 and are not folded into this standalone repository step.

The package-owned predecessor examples and inactive integration sources under
`crates/sdk` are not Cargo consumers because the manifest sets `autotests =
false` and `autoexamples = false`. They remain quarantined only until the
ordered documentation and package-surface cleanup in Steps 247–248; they are
not compiled, exported, or authorized as compatibility APIs.

No compatibility alias or feature was added for an external consumer. The
next consumer edge created in this repository must be the exact `radroots ->
radroots_sdk` dependency and forwarding contract specified for the facade.
