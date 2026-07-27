# Contributing

Radroots SDK changes are contract-driven and independently reviewable. Before
editing, read these files in order:

1. `AGENTS.md`
2. `docs/specs/README.md`
3. `docs/specs/radroots_crates_release_v1.md` for crate-surface work
4. the affected manifests, implementation, contracts, generators, and tests

The release-v1 architecture identifier is `radroots.crates.release.v1`. This
repository owns `radroots_sdk` and the ordinary-user `radroots` facade; the
standalone core-library repository owns the other 17 public packages.

## Workflow

1. Inspect repository status and the current source authority.
2. Make one coherent, commit-sized change.
3. Update public contracts, tests, generators, checked-in outputs, and docs
   with the implementation they govern.
4. Run the narrowest repository-owned checks that prove the change, followed
   by the broader workspace or package lane required by its scope.
5. Review the staged diff for API leakage, private dependencies, generated
   drift, secrets, hidden side effects, and unrelated changes.

Use `cargo xtask check` for the repository-wide Rust and generated-package
lane where applicable. Run targeted format, check, test, Clippy, contract, and
generated-freshness commands while iterating.

## Commits and deviations

Use this commit form:

```text
<scope>: <lower-case imperative summary>
```

Keep commits focused and keep public commit language independent of any
private checkout. Do not publish, tag, merge, or change registry ownership
without explicit authorization.

When current evidence proves a planned step obsolete or unsafe, follow
`docs/implementation/DEVIATIONS.md` and validate the machine-readable ledger
with `cargo xtask architecture`. Complete
`docs/implementation/STEP_REPORT_TEMPLATE.md`, and keep
`docs/implementation/TRACEABILITY.md` aligned with durable requirements.
Record the evidence and affected spec anchor before changing the plan; do not
silently redefine the architecture.
