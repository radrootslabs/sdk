# Public API baselines

These files are reviewed release artifacts for the standalone SDK capsule.
They contain the simplified (`-sss`) all-features API emitted by
`cargo-public-api`; private implementation items and automatically generated
trait noise are intentionally omitted.

Regenerate the `radroots_sdk` baseline from this repository root with:

```sh
cargo public-api -p radroots_sdk --all-features -sss --color never
```

An API change is not accepted merely because the baseline can be regenerated.
Review additions, removals, and changed signatures against the package charter
and versioning policy before replacing a baseline.
