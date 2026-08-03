# SDK native API migration

The Release V1 SDK intentionally makes a breaking cut from the predecessor
representation-shaped API. Native Rust callers must use module context,
constructors, builders, and accessors rather than compatibility aliases or
public field layout.

| Predecessor pattern | Release V1 API |
| --- | --- |
| `RadrootsClient` | `radroots_sdk::Client` |
| `RadrootsClientBuilder` | `radroots_sdk::ClientBuilder` |
| `RadrootsSdkError` | `radroots_sdk::Error` |
| `RadrootsSdk*` or `Sdk*` product wrappers | contextual types such as `farm::Plan`, `listing::PrepareRequest`, and `trade::Operations` |
| SDK copies of event, trade, storage, sync, or transport values | the canonical type from its owning lower crate |
| struct literals over SDK request, plan, receipt, status, or error fields | the type's constructor or builder plus stable accessors |

There are no deprecated prefixed aliases. Code that previously projected SDK
fields directly into Studio or another host must instead translate from stable
accessors at that host boundary. This prevents additive native fields from
causing field-skew failures in consumer struct patterns and keeps private
storage, transport, and signer representations replaceable.

The crate root exports only `Client`, `ClientBuilder`, `Error`, and `Result`.
Advanced types remain under their owning modules. Deliberately passive,
versioned wire DTOs remain owned by `radroots_protocol`; the SDK does not copy
their public fields into native wrapper structs.
