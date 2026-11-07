# Public source release gate

Input 1a871a580847f9273c944a0aaa33a32b57fb50e1. DOCS profile;
WP-015/016, REQ-006/008/010, SPEC-014, IF-010/013, PKG-008/009.
Scope: accurate public documentation, generic source, privacy/license review and
a reproducible candidate. This does not authorize visibility changes, force-push,
remote branch deletion, consumer migration or runtime/storage changes.

## Source preparation

- README, current architecture, roadmap and maintainer setup distinguish the
  implemented components from target lifecycle/transport/custody behavior.
- Private consumer notes were copied and hash-verified outside the repo before
  current summaries were generalized. Generic integration lessons replace raw
  private feedback. Original evidence remains private; summaries are not new proof.
- Original MIT license and donor/upstream provenance remain intact. Vendored
  checker licenses distinguish software from other upstream content. Distribution
  must retain applicable notices; publishing source does not publish dependencies
  or grant rights to consumer media.
- Current source, reachable history and public hosting metadata need separate
  checks. Secret scanning cannot prove the absence of private context.

## Publication boundary

The audit found private consumer context in historical notes and GitHub issue/PR
descriptions. Deleting current files does not remove it from old branches, tags,
PR refs, comments or cached hosting views. No history rewrite or hosting metadata
mutation has been applied by this preparation.

A clean-history source snapshot is the simplest reviewable publication candidate.
Keep the original private repository and verified full-history bundle as archive.
Historical evidence SHA references in public summaries describe original private
checkpoints; they are not reachable-commit promises in a clean-history release.
Retain source attribution and its original pins independently of rewritten history.

Before publishing, the owner must approve the exact candidate, destination/name,
history policy and treatment of existing hosting metadata. Changing the existing
repo's visibility is blocked until that surface is reviewed and addressed.
Use a fresh public destination if old private PR/history data must remain private;
do not assume force-pushing the default branch erases it from GitHub.
GitHub also makes historical Actions logs visible on a private-to-public change;
see its [visibility guidance](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/setting-repository-visibility).

## Validation and limitations

Run source/link/hash checks, fmt, appropriate locked Rust/Python checks and hosted
Windows/Linux CI. Exercise README commands in a clean exported tree without
adjacent private repositories. Scan that exact export and its Git history.
Record tool/version, scope, findings, reviewed false positives and artifact hash
with the private release evidence. Keep raw scanner/hosting reports external.

The current historical secret-scan exception is one reviewed Markdown task-table
phrase, pinned by exact fingerprint. It is not a credential and is not a broad
path/rule suppression. No public-ready claim follows merely from that exception.
Producer review is not independent acceptance or permission to publish.

## Producer role review

Identity: code/hash domains unchanged; candidate source must have an exact tree
and archive digest. Placement/custody: no policy, payload or removal authority
changes. Concurrency: no runtime behavior or cache coordination changes.
Privacy: generic current source and scoped scanner exception pass; historical and
hosting context require owner disposition. Adoption: preserve original history,
donor notices, external bindings and synthetic setup. No private fixture or
adjacent checkout is necessary to build. These are producer-applied checklists.

CI pins third-party action commits and Gitleaks 8.30.1 with its release archive
checksum. Its separate history scan uses redacted output and the narrow reviewed
fingerprint. Privacy-context review remains necessary even when that job passes.

Preparation checks: source/link/hash/diff validation, README build, ten synthetic
catalogue cases and three legacy-helper cases passed locally. Current-tree and
reachable-history Gitleaks scans passed with the exact prose exception; the new
hosted secret-scan job passed. The resolved all-feature Windows dependency graph
has 101 packages, none missing license metadata; this is an inventory, not legal
clearance or a bundled binary notice set. Original vendor pins/licenses passed.
Clean-export qualification and full hosted platform checks remain separate.
