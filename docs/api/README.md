# Public API baselines

These files are reviewed release artifacts for the standalone SDK capsule.
They contain the simplified (`-sss`) all-features API emitted by
`cargo-public-api`; private implementation items and automatically generated
trait noise are intentionally omitted.

Regenerate either Rust front-door baseline from this repository root with:

```sh
cargo public-api -p radroots_sdk --all-features -sss --color never
cargo public-api -p radroots --all-features -sss --color never
```

An API change is not accepted merely because the baseline can be regenerated.
Review additions, removals, and changed signatures against the relevant package
charter and versioning policy before replacing a baseline. The facade snapshot
must remain curated: it must not contain a `radroots::sdk` module, facade-owned
traits, or an undifferentiated copy of the advanced SDK surface.
