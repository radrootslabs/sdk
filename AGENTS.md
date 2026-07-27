# Radroots SDK agent specification

This file applies to the full standalone SDK repository. Read
`CONTRIBUTING.md` for the contributor workflow. A closer `AGENTS.md`
overrides this file for its subtree.

## Source of intent

- Read `docs/specs/README.md` and
  `docs/specs/radroots_crates_release_v1.md` before changing a public
  package, dependency, feature, binding, or release control.
- The Markdown specification is normative. Its TOML catalog is the executable
  package and dependency representation; the CSV and DOT files are review
  aids.
- Current source and tests are implementation evidence. They do not silently
  override `radroots.crates.release.v1`.
- Record any evidence-based plan deviation in
  `docs/implementation/deviations.toml`, following
  `docs/implementation/DEVIATIONS.md`, before proceeding. Validate it with
  `cargo xtask architecture`.

## Repository operating model

- This repository owns the Radroots SDK workspace, including Rust SDK APIs,
  generated language bindings, FFI layers, WebAssembly surfaces, package
  metadata, and SDK validation flows.
- It owns `radroots_sdk` and the ordinary-user `radroots` facade. The 17
  lower release-v1 packages remain owned by the standalone
  `radrootslabs/lib` repository.
- Do not make this repository responsible for downstream apps, private
  layouts, deployment policy, or compatibility packages unless represented by
  a public contract here.
- Keep commits and handoff language standalone and open-source-readable. Do
  not reference private checkout structure or internal coordination context.
- Prefer the smallest coherent target-state change. Do not mix unrelated
  cleanup, speculative abstraction, compatibility scaffolding, or roadmap work.

## Preflight and engineering rules

- Inspect the relevant specs, manifests, implementation, tests, package
  metadata, generators, and generated outputs before editing.
- Inspect `git status --short` and preserve unrelated work.
- Use checked-in repository commands and the narrowest validation that proves
  the change; never claim a check passed unless it ran successfully.
- Work spec-first. Do not invent packages, bindings, exports, compatibility
  layers, or publishing behavior.
- Prefer explicit typed models, deterministic behavior, narrow side effects,
  and direct service boundaries over stringly or implicit behavior.
- Avoid hidden production panics. Use typed errors for expected failures.
- Avoid `unsafe` unless strictly necessary and document the local invariants.
- Do not expose secrets, private keys, credentials, tokens, private
  identifiers, sensitive user data, or sensitive event content in code, logs,
  tests, fixtures, docs, or examples.

## Architecture and generation rules

- `radroots_sdk` is the advanced front door. It owns host-neutral client
  semantics, not global runtimes, hidden workers, logging installation, UI
  state, Studio databases, or process lifecycle.
- `radroots` is a curated ordinary-user facade. It has no public `sdk`
  namespace and does not wildcard-reexport `radroots_sdk`.
- Cross-repository dependencies on the lower package family use registry
  versions in release candidates, never production sibling paths or Git
  overrides.
- No public package has a dependency on a private or unpublished Radroots
  package, including dev, build, optional, and target-specific edges.
- Own generated artifacts through checked-in schemas, generators, templates,
  and public contracts. Do not hand-edit generated output.
- Generated bindings remain reproducible and do not mechanically dictate the
  native Rust module layout.
- During migration, every package remains non-publishable until its
  package-realistic release gates pass and publication is explicitly
  authorized. Follow `docs/implementation/PUBLICATION_FREEZE.md`.

## Commits, deviations, and irreversible actions

- Format commits as `<scope>: <lower-case imperative summary>`.
- Keep commits focused and reviewable. Use a blank line before a multi-line
  body and `- ` bullets for notable changes and validation.
- If repository evidence proves a planned step obsolete or unsafe, record the
  evidence, affected specification anchor, disposition, and validation in
  `docs/implementation/deviations.toml`, following
  `docs/implementation/DEVIATIONS.md`. A normative change also requires an
  approved decision record.
- Do not publish crates or packages, create release tags, change registry
  ownership, merge or rename repositories, merge pull requests, rotate
  credentials, or mutate trusted-publisher configuration without explicit
  authorization.

## Definition of done

- The requested change is complete at the correct package boundary.
- Affected code, tests, contracts, generators, outputs, and docs agree.
- Relevant repository-owned validation passed or an exact blocker is reported.
- The final review records files changed, checks run, residual risks, and
  whether the next step is safe.
