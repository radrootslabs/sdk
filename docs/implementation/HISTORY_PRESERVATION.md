# Standalone history preservation

Step 015 is satisfied under `RCRV1-DEV-001` without importing or combining
repository history. This repository remains the independent source authority
for `radroots-sdk`, the new `radroots` facade, bindings, and SDK tooling.

## Verified checkpoint

- repository: `git@github.com:radrootslabs/sdk.git`
- reviewed baseline: `fd8384aee348034e0c8ea17a868fe7f094770050`
- verified candidate parent: `7c0177cd5b2261e2e5bea054907aa6147d52aae7`
- baseline relationship: the reviewed baseline is an ancestor of the verified
  candidate parent
- submodules: none
- import, subtree, filter-repo, history merge, repository rename, or archive:
  not required and not performed

`git log --follow` retains representative history for
`crates/sdk/Cargo.toml`; `crates/radroots/Cargo.toml` begins at the approved
facade scaffold commit. `git fsck --full` completed successfully with no
corrupt or missing reachable objects. It reported only unreachable dangling
objects retained by Git; those are not part of the release candidate and were
not pruned or modified.

The next workspace steps must preserve this repository, its lockfile, and its
release boundary independently from `radrootslabs/lib`.
