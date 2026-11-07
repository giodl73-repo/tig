# Public experimental source baseline

Tig's canonical public repository is https://github.com/giodl73-repo/tig.
It began with a clean root snapshot; the original private Git/hosting history
was preserved separately. It was not made public by changing its visibility.
Current public history and current source are the release surface.

Original evidence SHAs in historical summaries identify their original source
checkpoints. They are not promises that those commits exist in public history.
Keep donor attribution and pins; port reviewed generic patches rather than
importing private ancestry.

## Current scope

The [README](../README.md), [architecture](ARCHITECTURE.md) and
[roadmap](../PRODUCT_PLAN.md) distinguish implemented experimental components from
target behavior. Publication is not a stable release, production certification,
complete dependency/job coverage or safe payload-eviction acceptance.

The [six-role baseline review](reviews/2026-11-06-public-baseline.md) records
findings, dispositions and verification scope. Roles are producer-applied
checklists, not independent people or publication authority.

## Release and contribution checks

- Small source, synthetic fixtures and generic contracts belong in Git. Machine
  enrollment, credentials, private prompts/assets and growing evidence stay external.
- Identity/schema/provider changes require focused negative/integration fixtures.
  Preserve unknown coverage, independent custody and explicit removal authority.
- Run source/link/hash/budget validation, formatter, appropriate locked tests and
  strict Clippy. Check the standalone Git/LFS build separately from all features.
- Exercise documented build commands without an adjacent private checkout.
  Windows/Linux CI and the separate redacted secret scan are required on PRs.
- Scan current source and reachable history. A secret scanner does not detect
  every private operational fact; review example/config paths and hosting artifacts.
- Retain MIT, donor and vendored licenses. Cargo license metadata is an inventory,
  not legal clearance or the notices required for a distributed binary.
- Keep raw audit/scanner/hosting reports outside source. Record bounded generic
  conclusions, exact inputs and remaining limits in the review.

Main requires an up-to-date PR and passing GitHub Actions checks, including the
secret scan. Force pushes and branch deletion are disabled. No second reviewer is
fabricated when a sole maintainer applies the role checklists. GitHub secret
scanning/push protection and private vulnerability reporting complement CI;
they do not prove the absence of secrets.

The public snapshot needs no historical scanner exception. The original private
history's exact prose false positive remains a private audit concern, not a public
path/rule suppression. Security reporting is described in [SECURITY.md](../SECURITY.md).

## Future publication boundaries

Before moving a private source/hosting surface into public view, obtain explicit
owner approval of its contents, exact destination and history policy. Removing a
current file or force-pushing main does not erase old refs, PR context or logs.
GitHub also exposes old Actions logs on a private-to-public visibility change;
see [GitHub visibility guidance](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/setting-repository-visibility).
Preserve recovery evidence privately and verify anonymous access to the approved
public tree. Never substitute a branch name or delivery receipt for byte custody.
