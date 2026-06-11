# radroots_sdk - code directives

- this repo owns the Radroots `sdk` workspace, including Rust SDK APIs, generated language bindings, FFI layers, WebAssembly surfaces, package metadata, and SDK validation flows
- own generated SDK artifacts through their source generators, schemas, templates, and public contracts, not by hand-editing generated output
- do not make this repo responsible for downstream app repos, private layout, platform deployment, publication policy outside this repo's public contract, or compatibility packages unless explicitly represented here
- work spec-first for public SDK behavior; do not invent packages, bindings, exports, compatibility layers, or publishing behavior
- prefer the smallest coherent change that fully addresses the request; do not mix unrelated cleanup, speculative refactors, compatibility scaffolding, or roadmap work into the same change
- inspect the relevant implementation, tests, manifests, specs, package metadata, and docs before changing behavior
- do not depend on private repositories, unpublished artifacts, local machine layouts, absolute paths, or internal monorepo context
- keep generated bindings reproducible from checked-in source contracts
- preserve root imports, package boundaries, and public API shapes unless the task explicitly changes the SDK contract
- when behavior changes affect generated outputs, update the source contract and regenerate through repo-owned tooling rather than hand-editing artifacts
- prefer explicit typed models, deterministic behavior, narrow side effects, and direct service boundaries over stringly or implicit behavior
- avoid hidden production panics; use typed errors for expected failure modes
- avoid `unsafe` unless it is strictly necessary, locally justified, and documented with nearby invariants
- do not expose secrets, private keys, credentials, tokens, invite codes, private identifiers, sensitive user data, or sensitive event content in code, logs, tests, fixtures, docs, or examples
- use checked-in, repo-owned validation first; prefer narrow contract tests plus repo-wide validation for generated-code or package-surface changes
- if validation cannot run, report exactly what was skipped and why; never claim validation passed unless it actually ran
- keep commits focused and reviewable, using `<scope>: <imperative summary>` unless a repo convention overrides it
