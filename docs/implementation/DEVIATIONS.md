# Implementation deviations

The machine-readable authority is [`deviations.toml`](deviations.toml).
Repository checks validate it on every architecture and full check lane. This
ledger records evidence-based changes to implementation planning; it does not
silently change `radroots.crates.release.v1`.

## Active records

| ID | Affected steps | Approved disposition |
| --- | --- | --- |
| `RCRV1-DEV-001` | 015-023 | Preserve the existing standalone `lib` and `sdk` repositories; replace repository import/unification with independent qualification. |
| `RCRV1-DEV-005` | 013, 019-026, 226, 247, 249-268, 305 | Pin every Rust crate and internal Radroots dependency in `radrootslabs/sdk` to exactly `0.1.0-alpha` until further explicit authority. |

## Closed records

| ID | Closure |
| --- | --- |
| `RCRV1-DEV-002` | The facade remained in `oss/sdk`, completed Steps 249-260, and entered validation-only publication staging at Step 305. |
| `RCRV1-DEV-006` | Steps 261-268 replaced authenticated predecessor snapshots with protocol- and codec-owned generation. |
| `RCRV1-DEV-007` | Step 235 removed SDK-local mappings; Step 313 removed the last daemon/lib transport aliases and helpers. |
| `RCRV1-DEV-008` | Step 313 confirmed every predecessor secrets/storage package and downstream source edge is absent. |
| `RCRV1-DEV-012` | Steps 282-283 qualified the final shared SDK engine, app_rt bindings, and iOS host lifecycle boundary. |

## Record template

Add one `[[deviation]]` table to `deviations.toml`:

```toml
[[deviation]]
id = "RCRV1-DEV-NNN"
date = "YYYY-MM-DD"
status = "active" # active | closed | superseded
approval = "Explicit approving decision."
affected_steps = ["NNN"]
spec_anchors = ["docs/specs/<durable-spec>#<anchor>"]
source_evidence = ["Committed source evidence."]
replacement_action = "Smallest safe disposition."
verification = ["Command or review evidence."]
unresolved_risk = "none, or a concrete bounded risk"
normative_architecture_change = false
adr_required = false
closure_evidence = [] # omit while active; required when closed or superseded
```

Every field is mandatory except `closure_evidence` on active records. Spec
anchors must resolve inside `docs/specs/`; affected steps must be three-digit
IDs in 001-315. A normative architecture change needs explicit approval and
the appropriate ADR decision before the record can be accepted.

Do not silently skip, merge, reorder, or broaden implementation steps. Keep a
red checkpoint uncommitted and mark the next step blocked until its evidence or
approval is complete.
