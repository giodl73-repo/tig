# Project restore progress: producer review

Controls and entry identity: [PROJECT-PROGRESS](../../specs/PROJECT-PROGRESS.md),
WP-005/011/015, REQ-001/003/006/010, IF-005/008/010/013, PKG-003/004/008.
Allowed changes: opt-in bounded local evidence writer/reader, CLI flags and
synthetic recovery fixtures over the unchanged shared copy primitive. No new
cache, daemon, hooks, live provider write, production installation or asset edit.

Seven additional project harness cases (one child driver) cover explicit logged
apply/retry, verified-existing reuse, conflicting changed destinations, observer
stop, actual child exit after a synced row, missing/torn completion, raw evidence
pinning, stale profile/plan, reordered/unknown/duplicate JSON, oversized logs,
fresh outside-cache paths and real CLI flag/inspection behavior. Existing project
fixtures remain present. Final project20, restore8 and library4 suites pass,
including a new read-only evidence-sink refusal unit. Strict Clippy and standalone
Git/LFS check pass; the earlier full workspace pass precedes final attempt-ID/
failure-unit additions. Subsequent exact-head Windows/Linux full qualification
passed in run 41226010966; merged main `eb482ee` preserves the tested whole tree
`a5f98058963ed9d225d7768bfac95b9ac9c73988`. No consumer runtime was installed.

Producer-applied lenses, not independent review or human acceptance:

- Identity/namespace: exact source/profile/manifest/plan and per-entry identity
  checks; no use of checkout HEAD or log contents as fresh selection authority.
- Concurrency/recovery: exclusive fresh log paths, per-row sync, no global lock,
  explicit stop/retry, no append after possibly torn writes. A copied but unlogged
  file is unknown until retry verifies it; a logged file can later change.
- Provider/privacy: strict bounded JSON, no raw paths in fixed CLI errors,
  private asset-side identities, ordinary parent checks, inherited Windows ACLs,
  trusted namespace boundary and no automatic traversal/cleanup.
- Storage/custody: observations of a completed copy do not retain source guards,
  establish independent custody or authorize eviction. No portable power-loss,
  directory persistence or multi-file transaction claim.
- Git/adoption: small authored manifests stay authoritative; expanding operational
  JSONL remains outside Git. A fresh log cannot replace or silently rewrite prior
  evidence. No consumer runtime is switched by this patch.

Disposition: pass_with_risk for explicit progress evidence, subject to local and
hosted qualification. Open integration work: full consumer runtime/environment
handoff, orphan classification, source enrollment/custody completeness and actual
reversible adoption. Disk-full/power-loss fault matrices and hostile namespace
security are not accepted by the current synthetic process-exit evidence.
