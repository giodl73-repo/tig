# Explicit staged-index protection

Input195bd0a5fee0b431e53bd3b4f071b5605d4ef783.
WP-004/005/008/009/015; REQ-001/002/004/005/007/008/009;
SPEC-002/004/005/006/007/008/009; IF-002/004/005/006/008/009/013;
PKG-002/003/004/008/009. GIT/PROVIDER profiles. Allowed scope: explicit bounded
index capture, draft-only source adoption, existing metadata/journal/placement
composition, private CLI profiles and synthetic fixtures. No live enrollment,
hook installation, index/worktree edits, payload discovery or final deletion.
Entry: enrolled draft source, exact pointer scope and existing guarded metadata/
source/provider contracts. Exit: plan/status/apply/resume/reconcile protect staged
bytes including conflict alternatives, reject stale/incomplete capture, preserve
historical receipts and reuse ordinary placement. Implementation and verification/
validation/trace/six producer lenses close separately.

An index is a draft obligation, not a committed version or a successful merged
tree. Private enrollment pins logical repository/worktree IDs, source owner,
namespace/policy, exact pointer prefixes and budgets. The operator binds that
enrollment to an existing repository/worktree root; logical IDs are assigned
authority, not inferred from a remote URL or path. Copies/relocations require
explicit enrollment review. New sources must already be registered as Draft.

Capture reuses Repository::index/extract and immutable pointer-sized Git blob
reads; it never opens working payloads. Every supported stage-0/1/2/3 pointer
contributes to a unique-object obligation tree keyed by payload digest. Nonzero
stages are allowed only for this protection set and remain explicitly unresolved.
The existing complete merged-tree/transfer extraction API is not relaxed. Any
issue other than Unmerged (missing blob, unsupported mode/path/pointer, conflicting
size or budget) holds adoption rather than shrinking unknown protection.

Plan identity binds enrollment, exact index listing, obligation root/context,
conflict/association counts and prior source revision/state. Capture, metadata
finish and independent readback precede atomic existing Prepare/Adopt. Recheck
the index listing before admission and sample it after adoption; source drift is
reported separately from the historical acknowledgement. Git and SQLite do not
share a transaction. Exact retries remain historical, no-op adoption writes no
commands, stale plans cannot replace a newer draft, and status needs no Git or
metadata provider. No new journal/event encoding is introduced.

Unstaged/private producer writes and future Git index mutations are not covered
by a past capture; admit ingest/job protection before managing those bytes.
This explicit command is not an automatic clean hook or universal worktree scan.
Previous draft roots remain protected under the enrolled predecessor policy.
Commit success alone does not withdraw the index source: independently adopt the
branch and retain draft protection until owner-authorized source settlement.
Automatic branch withdrawal/rename, commit handoff, arbitrary private overlay and
native opaque closure remain separate priority-6 work; no full exit is claimed.

Physical reconcile uses the existing private placement profile, default-false
copy/eviction/reacquisition/cancellation flags and one bounded page. Current index,
admitted draft state, page convergence and full storage convergence remain
distinct. Every ordinary removal still uses current all-namespace/keeper fences.
No retained backup is deleted or new cache engine created.

## Operator interface

```text
tig-lifecycle index plan PROFILE
tig-lifecycle index status PROFILE PLAN
tig-lifecycle index apply PROFILE PLAN
tig-lifecycle index resume PROFILE PLAN
tig-lifecycle index reconcile PROFILE PLACEMENT_PROFILE [CURSOR]
```

`describe` advertises index_profile_schema and index_operations separately from
committed-ref commands. PROFILE is strict `tig.git-index-profile.design-1`:
enrollment, git_executable, repository_root, source_limits, namespace_root,
namespace_minimum, metadata_root, metadata_minimum and default-false
allow_selection. Enrollment contains repository/worktree/authority/namespace/
policy digests, source/owner, scope and the existing lifecycle Budget. Documents
are bounded regular absolute JSON files. Persist canonical plan bytes without
the CLI's trailing newline; existing command plans/encodings remain unchanged.

Reconcile reuses `tig.placement-profile.design-1` and its separate permissions.
Validate owner/authority/policy, canonical namespace root and complete provider/
ledger bindings before changing the draft. Later provider failures preserve
the durable selection result with pending/held storage status. Each invocation
processes one page; cursors bind exact desired basis. Source drift or unavailable
post-work index observations cannot assert convergence. Root/provider absence
never initializes or replaces storage. All expanding operational data stays
outside Git; no per-stage payload duplication is implied.

The current source journal still has 64 predecessors/source and finite command/
byte lifetime. Repeated index changes can exhaust that bound and must hold;
no automatic predecessor discard/reset is added. Qualified owner settlement and
journal continuation are required before sustained unattended adoption.

L0: fmt, validator, pins and diff. L1: locked default/all-feature workspace,
strict Clippy, standalone Git and focused canonical/authority/stale/coverage cases.
L2: real temporary staged/unstaged/conflicted/linked-worktree Git fixtures, source
drift at handoff, actual CLI -> existing three stores, no-op retry, missing source
status and interrupted metadata/adoption recovery. Small fixtures do not qualify
large-file custody, authority migration, lifetime, final deletion or user adoption.
