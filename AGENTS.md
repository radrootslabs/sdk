# Radroots SDK agent specification

This file applies to the complete standalone SDK repository. Read
`CONTRIBUTING.md` before editing. A closer `AGENTS.md` overrides this file for
its subtree.

## Current authority

- This capsule is an independently verifiable public generated-package and
  source-lock consumer. Its Rust workspace contains only the unpublished
  `radroots_sdk_source_lock` package; canonical Rust SDK implementation and
  generators remain in the exact public `radrootslabs/lib` revision selected
  by `radroots.lib.source-lock.v1.toml` and `Cargo.toml`.
- `radroots.lib.source-lock.v1.toml` is the exact lib source-lock authority.
  `contracts/provenance/**`, `contracts/packages/**`, and
  `contracts/exports/**` own generated artifact provenance and package/export
  selection.
- `contracts/historical_authority.v1.json` owns the closed Release V1 machine
  artifact inventory and its exact digests. Historical API baselines live at
  `contracts/api_baselines/**`; other retained machine history lives below
  `contracts/architecture/**` and `contracts/crates/release_v1/**`.
- Human specifications, decisions, migration history, and qualification
  evidence are parent-owned under `docs/oss/sdk/**`. They are not present in a
  standalone clone and must never become a build, test, generation, package,
  or release input for this capsule.
- Current source, generated output, tests, and lockfiles are implementation
  evidence. They do not silently override the selected source revision or
  checked-in contracts.

## Repository boundary

- Keep the repository standalone, forge agnostic, and open-source-readable.
  Do not depend on a non-public parent path, non-public contract, local sibling
  checkout, unpublished local artifact, or internal coordination context.
- Production source selection must use the exact remotely reachable public Git
  revision recorded by both source-lock surfaces. Floating branches, tags,
  local paths, and mismatched revisions are forbidden.
- The `.radroots-consumer-root` marker must remain exactly `sdk` followed by
  LF. Source resolution must remain absolute, canonical, non-symlinked, and
  explicitly supplied through `RADROOTS_LIB_SOURCE_ROOT`.
- `docs/**`, `.github/**`, and `.act/**` are forbidden tracked roots. Public
  validation commands live in this repository; private cross-repository
  orchestration belongs only to the parent repository's root `.act/**`.
- Do not make this repository responsible for private applications,
  deployment policy, service runtime ownership, or compatibility packages.

## Generated artifacts and packages

- `tools/radroots_sdk_artifact.mjs` is the governed generation/check adapter.
  It delegates generation and source-lock verification to the selected public
  lib checkout through lib's `cargo xtask` surface.
- Generated artifacts are reproducible outputs of checked-in source locks,
  package/export contracts, and producer generators. Do not hand-edit
  `generated/**`, generated files under `packages/**`, provenance JSON, or
  package source-lock output.
- Update generators and canonical contracts first, regenerate, inspect the
  complete diff, and run freshness checks. Generated output never dictates a
  native source model or creates a second source authority.
- Keep package manifests, the pnpm lockfile, provenance, exports, generated
  source, and consumer-facing package READMEs synchronized.
- Do not reintroduce retired prototype evidence, outcomes, receipts, event
  models, runtime contracts, or compatibility aliases. Services-hardening
  generated changes must expose the approved four coverage states and three
  outcomes together across every applicable language/package surface.

## Working and verification rules

- Inspect `git status --short`, relevant contracts, package manifests, tools,
  generated outputs, and tests before editing. Preserve unrelated work.
- Run `cargo extbuild doctor` before the first mutating build, test, check,
  dependency, package, or generation command, then route it through
  `cargo extbuild run -- ...`.
- `pnpm run contracts:check` validates the exact historical inventory and the
  absence of forbidden public roots without requiring a lib checkout.
- `pnpm run test:tools` runs standalone tool and boundary tests.
- `pnpm run source:check` and generation/freshness commands require an exact
  `RADROOTS_LIB_SOURCE_ROOT` matching the checked-in lock. `pnpm run check` is
  the full generated-package lane.
- This repository has no local `cargo xtask` package. Do not document or invoke
  nonexistent SDK-local xtask commands; the artifact adapter invokes the
  selected producer's governed xtask explicitly.
- Use the narrowest check that proves a change while iterating, followed by the
  complete affected standalone lane. Never claim a check passed unless it ran
  successfully.
- Prefer explicit typed models, deterministic behavior, bounded inputs, narrow
  side effects, and fail-closed validation. Avoid hidden production panics and
  `unsafe`; if `unsafe` becomes unavoidable, document and test its invariants.
- Never expose secrets, credentials, tokens, private identifiers, sensitive
  user data, or sensitive event content in source, logs, fixtures, generated
  output, examples, or errors.

## Changes, commits, and external gates

- Make one coherent, reviewable target-state change at a time. Do not mix
  unrelated cleanup, speculative abstraction, compatibility scaffolding, or
  roadmap work.
- Use commit subjects in the form `<scope>: <lower-case imperative summary>`.
- A machine-contract change must update its validator and negative tests in the
  same checkpoint. A generated contract change must update all affected
  outputs and consumer qualification evidence in its owning sequence.
- Repository evidence that invalidates an active parent specification is a
  review finding to record in parent-owned authority; do not create a local
  human deviation ledger or silently redefine behavior.
- Do not push, tag, publish packages, mutate registry ownership, change trusted
  publishers, deploy, or perform credential operations without the separate
  authority required for that external action.

## Definition of done

- The change is complete at the source-lock, contract, generator, or package
  boundary that owns it.
- Contracts, tools, tests, package metadata, generated outputs, and lockfiles
  agree, with zero tracked `docs/**`, `.github/**`, or `.act/**` paths.
- Relevant standalone validation passed, exact failures are reported, the diff
  contains no private dependency or unrelated change, and the next sequence
  step is explicitly safe or blocked by a real external gate.
