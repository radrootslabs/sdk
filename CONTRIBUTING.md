# Contributing

Radroots SDK changes are contract-driven and independently verifiable. Before
editing, read `AGENTS.md`, then inspect the affected source lock, contracts,
package manifests, tools, generated outputs, and tests.

This repository is the public generated-package and source-lock consumer for
the Radroots SDK cohort. Canonical generator and Rust implementation source is
selected from `radrootslabs/lib` by the exact revision in
`radroots.lib.source-lock.v1.toml` and `Cargo.toml`. Human architecture and
execution authority is parent-owned under `docs/oss/sdk/**`; standalone
commands do not require that private parent documentation.

## Workflow

1. Inspect repository status and the current machine authority.
2. Make one coherent, commit-sized change at the owning contract, tool,
   generated-output, or package boundary.
3. If producer behavior changes, update the selected public lib source first,
   then regenerate every affected SDK output from that exact reachable
   revision.
4. Update contracts, tests, generated outputs, package metadata, and lockfiles
   together.
5. Run the narrowest repository-owned checks that prove the change, followed
   by the complete affected standalone lane.
6. Review the staged diff for source-lock drift, handwritten generated output,
   stale provenance, private dependencies, forbidden roots, secrets, and
   unrelated changes.

Run `cargo extbuild doctor` before the first mutating verification command and
route repository checks through `cargo extbuild run -- ...`. The primary
commands are:

```text
pnpm run contracts:check
pnpm run test:tools
pnpm run source:check
pnpm run check
```

The source and generation lanes require an absolute, canonical
`RADROOTS_LIB_SOURCE_ROOT` whose Git revision matches the checked-in source
lock. The contract and tool-test lanes remain usable without the parent
monorepo. This capsule has no local `cargo xtask` package.

## Commits and external actions

Use `<scope>: <lower-case imperative summary>` for focused commits. Do not add
capsule-local human authority, `.github/**`, or `.act/**`; do not create a
compatibility path for a breaking generated contract. Record any required
normative decision in the parent-owned services-hardening authority and update
the corresponding standalone machine contract.

Do not push, tag, publish, deploy, change registry ownership or trusted
publishers, or perform credential operations without separate explicit
authorization.
