# Explicit verified restoration pilot

Entry 9252a8a1822e99155e5d2bc9bdd4a96476cfe22f; controls, entry/exit and
L0/L1/L2 scope are in [the contract](../../specs/EXPLICIT-RESTORE.md).
The owner requested portable Git records and an external-drive byte store for
two consumers. Foundation tests remain synthetic; private consumer records,
media, machine profiles and operational receipts stay outside Tig source.

Implementation uses the existing exact Git resolver and ReadCache. Default is
read-only verification/preview. Apply writes a private sibling, syncs and rereads
its exact bytes, then hard-links without replacement. Source protection lasts
through publication. The Python operator helper delegates reads/copies rather
than adding a second cache implementation. It captures committed metadata and
checks all requested source objects before creating any consumer directories.

## Producer review

| Lens | Finding and disposition |
| --- | --- |
| Identity | Exact raw-byte pins and frozen Git views; uncommitted records do not select bytes. Existing different files refuse. |
| Placement | Explicit roots/paths only; bounded requested set. No claim census or automatic storage convergence. |
| Storage/custody | Copy verification is not backup custody. Sources are retained; no eviction API added. |
| Concurrency/recovery | Atomic no-replace publication; private partial cleanup on ordinary errors, retry after lost reply. Abrupt termination can leave an unselected partial. Parent namespace is trusted, not fenced against hostile swaps. |
| Provider/privacy | Existing CAS/LFS/flat readers and their declared protection. No private media/authored records in Tig. Windows reparse and portable traversal paths refuse. |
| Git/adoption | No branch/index/config modification. Consumer manifests and schema pins remain ordinary Git files; format is experimental and the consumer must pin the tool revision. |

Disposition: pass_with_risk for explicit single-object restoration and bounded
operator orchestration. This is producer checklist review, not independent or
human acceptance. Quota reservation, complete project closure, crash-directory
durability, global job protection and full consumer/custody adoption remain OPEN.
Verification and consumer rehearsal outcomes are recorded separately.
