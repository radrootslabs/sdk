# SDK lifecycle and safe-default test matrix

The Release V1 lifecycle contract is qualified at the advanced front door and
at each lower commit boundary.

| Scenario | Backend/configuration | Evidence |
| --- | --- | --- |
| clean construction starts no network, file, keyring, daemon, runtime, or worker | no-default/default | `lifecycle::clean_default_path_contains_no_implicit_resource_or_worker_authority` and the package-boundary worker guard |
| passive capability reporting | memory | `lifecycle::memory_client_is_passive_clone_shared_and_explicitly_closed` |
| concurrent clone shutdown and idempotent convergence | memory | integration lifecycle test plus `client::tests::concurrent_clones_converge_on_one_closed_state` |
| cancellation before close polling and after close begins | memory | integration lifecycle test plus `client::tests::close_cancellation_boundaries_are_explicit_and_retryable` |
| resources appear only after an explicit open; close is clone-shared | SQLite | `lifecycle::sqlite_resources_exist_only_after_explicit_open_and_close_across_clones` |
| cancellation before atomic enqueue leaves no committed outbox operation | memory canonical sync | farm, listing, and trade enqueue cancellation tests |
| cancellation after atomic enqueue cannot report rollback; replay is idempotent | memory canonical sync | farm, listing, and trade enqueue/replay tests |
| backend status, integrity, and lifecycle stay lower-owned | memory and SQLite | client, storage reliability, diagnostics, and package-boundary tests |

All resource-producing behavior is tied to an explicit asynchronous method.
Cargo features compile capabilities only; they do not open storage, contact a
transport, load a credential, or install scheduling.
