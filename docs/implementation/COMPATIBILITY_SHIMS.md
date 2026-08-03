# Compatibility shim quarantine

Step 064 retires version-suffixed packages from the public crate architecture.
The remaining package is a private, non-publishable bridge only; it is not a
second protocol authority.

| Shim | Final owner | Remaining consumers | Final removal |
| --- | --- | --- | --- |
| `radroots_runtime_contract_v1` | `radroots_protocol::runtime::v1` | SDK CLI-host generator; standalone `oss/cli` runtime registry and command code | Step 270 |

The `radroots_sdk` library no longer depends on or reexports the runtime shim.
Its radrootsd execution path also consumes
`radroots_protocol::radrootsd::transport_publish::v5` directly instead of the
separate transport-publish package. Repository publication policy classifies
the retained runtime shim as `retired`, and its manifest keeps
`publish = false`.

Source searches at this checkpoint also found the transport-publish shim in
standalone `oss/radrootsd` and the runtime/protocol shims in standalone
`oss/cli`; those consumers are assigned to Steps 286 and 270 respectively.
No new consumer may be added before those cutovers.

Step 248 removed the inactive SDK signer adapter, Nostr adapter, prefixed
models, private store, workflow runtime, and their dormant tests. No SDK-local
compatibility alias, feature, module, or dependency remains for them. The
standalone CLI still names predecessor SDK types and features, but that is an
external red consumer rather than authority to retain a second SDK surface:
Steps 269-272 own its dependency, product API, signing, and sync migrations;
Step 294 proves the downstream matrix; Step 313 rejects any surviving legacy
name before release qualification.
