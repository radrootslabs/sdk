# Compatibility shim retirement

Step 313 removed the final version-suffixed compatibility package after every
standalone consumer cut over to the final protocol and facade surfaces. No
second runtime protocol authority remains.

| Retired shim | Final owner | Cutover evidence | Final removal |
| --- | --- | --- | --- |
| `radroots_runtime_contract_v1` | `radroots_protocol::runtime::v1` | CLI facade-only refactor and Step 313 source census | Step 313 |

The SDK consumes `radroots_protocol::runtime::v1` and
`radroots_protocol::radrootsd::transport_publish::v5` directly. Repository
publication policy has no retained compatibility classification, and SDK
code generation no longer owns CLI runtime-contract output.

Step 248 removed the inactive SDK signer adapter, Nostr adapter, prefixed
models, private store, workflow runtime, and their dormant tests. Step 313
rejects any returning legacy package, feature, module, generated host surface,
or event-index binding package before release qualification.
