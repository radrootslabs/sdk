# SDK superseded-surface audit

Step 248 closes the predecessor `radroots_sdk` source boundary without adding
a deprecation package or compatibility API.

## Reachability and manifest audit

The final crate root reaches only the private `adapters::radrootsd` module and
the eleven chartered public modules. Cargo metadata and the package manifest
register exactly three integration tests and two examples. The removed files
were not reachable from that module graph and were not registered targets
because the package deliberately uses `autotests = false` and `autoexamples =
false`.

The deletion covers the dormant actor JSON, GeoNames wrapper, idempotency
wrapper, identity/knowledge reexports and builders, privacy model, private SQL
store, product-client wrappers, workflow runtime, obsolete Nostr/signer
adapters, and all tests/support files that exercised only those sources. The
active daemon adapter and its unit tests remain private because the chartered
`transport::DaemonDelivery` implementation uses them.

The final SDK manifest contains only the nine required and seven optional
Radroots dependencies from the package charter. It has no predecessor private
package dependency, production sibling path, prefixed feature alias, or
unregistered compatibility target.

## First-party consumer search

The search covered executable/configuration source in the parent workspace and
all checked-out `oss/*` capsules, excluding historical baselines, handoff
evidence, generated API snapshots, build outputs, and the untracked historical
`.sdk_step064_worktree` recovery checkout. That checkout is not a canonical
repository input and was left untouched.

One canonical external consumer remains red: standalone `oss/cli` still uses
the sibling `../sdk/crates/sdk` path, retired feature names, and prefixed SDK
types. This does not justify restoring a shim because the final SDK surface is
already cut over and the CLI is not part of this standalone workspace. Its
ordered disposition is:

- Step 269 removes sibling paths and selects final package/features.
- Step 270 migrates product operations and error imports.
- Step 271 migrates signer and NIP-46 composition.
- Step 272 migrates inbound/outbound synchronization.
- Step 294 proves the complete downstream compatibility matrix.
- Step 313 rejects all remaining legacy public names before qualification.

No other checked-out first-party executable source imports prefixed SDK types.
Documentation, release contracts, architecture fixtures, and lower-package
READMEs that name the final `radroots_sdk` identity are intentional and are not
compatibility consumers.

## Release disposition

`radroots_sdk` remains `publish = false`. No deprecation placeholder exists.
The only retained compatibility package in this repository is the separately
classified, non-publishable `radroots_runtime_contract_v1` generator bridge;
it is not linked by the SDK library and its external CLI cutover is Step 270.
