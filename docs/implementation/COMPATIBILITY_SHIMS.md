# Compatibility shim quarantine

Step 064 retires version-suffixed packages from the public crate architecture.
The remaining package is a private, non-publishable bridge only; it is not a
second protocol authority.

| Shim | Final owner | Remaining consumers | Final removal |
| --- | --- | --- | --- |
| `radroots_runtime_contract_v1` | `radroots_protocol::runtime::v1` | SDK CLI-host generator; standalone `oss/cli` runtime registry and command code | Step 270 |
| SDK signer provider façade and `adapters::signer` | `radroots_signing` plus host-owned `radroots_nostr_connect` adapters | SDK runtime/examples/tests; standalone `oss/cli` and `oss/studio_app` | Step 313, after SDK Step 248, downstream Steps 269-293, and matrix Step 294 |
| hidden `RadrootsSdkNip46ClientKey`, `RadrootsSdkNip46Transport`, and legacy constructors | package-owned `radroots_nostr_connect::client::{Client, Transport}` consumed by `RadrootsSdkMycNip46Signer::from_client` | standalone `oss/cli` | Step 313, after CLI NIP-46 cutover Step 271 and matrix Step 294 |

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

Step 109's first-party source search also found that immediate removal of the
SDK-prefixed signer provider types would break the standalone CLI and Studio
repositories. The SDK crate remains `publish = false`; the retained root
reexports and signer adapter module are hidden from generated documentation.
They must not gain new consumers or behavior. Step 248 owns the SDK-internal
cutover, Step 294 must prove the downstream cutovers, and Step 313 removes the
feature, dependency, module, reexports, and old names in full.
