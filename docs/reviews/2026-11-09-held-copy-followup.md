# Held-copy follow-up review

Input: 124642985af1a1b358611de6015b9d90c993c536; branch
feat/held-copy-followup. Controls, allowed scope, entry/exit and RUST/PROVIDER
L0/L1/L2 are in [the contract](../../specs/HELD-COPY-FOLLOWUP.md).
Targets: copy-only candidate/runtime, explicit operator, independent vectors,
temporary-root/process/artifact fixtures and CI qualification. No source command,
new cache/journal engine, live root, initialization, migration or deployment.

## Implementation

Fresh exact-copy decisions retain original object scope and actual sealed
original witness bindings. New provider facts, current complete protection basis,
window, operation and approval stay separate from old authority. A changed store
or birth can be reviewed within that object scope; the old action set never grows.
Metadata is finished and independently read back through one Session. Apply uses
a reader and existing content-wide clearance/final-provider fences. Each attempted
provider action admits its resources within the run limit.

Existing fixed accounted witnesses, debits and provider intent/release/unlink/
cancellation are reused. Stop precedes optional cleanup; unknown counts stay null.
Completed removals stay historical. No old source history is reconstructed.
Expiry must preserve original proof dependencies or separately admit a bounded
successor proof; this component does not implement that lifetime.

## Verification

Chain69506 passed nine focused entries (one process helper, five actual child
exits), fresh CLI qualification and strict store Clippy. It covers ended foreign
job, recovered provider, fixed exclusions, old-token refusal, aged original proof,
transient Prepare/Abort invalidation and offline cancellation. Two independent
Python/hashlib cases pin canonical headers and three witness records for both
completed and cancelled original provenance.

Expanded focus in83712 passes12 entries, one additional ignored artifact entry.
New cases reject outside-scope objects, conflicting size/owner and substituted
original witness jobs. The final73066 focus strengthens unavailable proof to an
actual corrupted acknowledged SQLite node with unchanged size/counters; fresh
hash verification refuses apply while durable stop reports unknown cleanup.
A cancelled partial origin keeps protection:
remaining historical obligations hold every requested copy. An initial expected
one-copy removal was wrong; observed all-claim protection corrected the test,
without weakening runtime checks. Source checkpoints and protected bytes survive.
Five actual exits exercise fresh intent, provider release, removal, completion
and stop. Ignored entries are not execution evidence; CI explicitly invokes the
fresh executable workflow.

Default83712 and all-feature51119 workspace/strict Clippy chains terminate zero,
including both500k metadata runs and standalone Git. Their test compilation
precedes only the stronger corruption fixture;73066 executes that final focused
case. Final73066 also passes six explicit fresh CLI qualifiers (ordinary cancel,
bulk inspection/execution, manual/aged paged and follow-up) and strict workspace
Clippy. Final L0/pins/fmt/diff pass. Hosted checks remain pending. Changed-source
Gitleaks8.30.1 reports no findings. Whole-directory scanning finds only two
inherited public synthetic paged-control digests; history-specific exclusions
are unchanged. Available-history scanning also passes (100 scanned commits).
An initial HEAD~100 range could not resolve; its zero-commit scan is not evidence.
No file/rule exclusion was added.

## Validation and trace

Synthetic roots exercise actual source/provider/ledger effects and an enrolled
LocalClock CLI. Independent current native authority, continuous project inactivity
after reactivation, real provider durability, large-file latency, stale restore,
metadata expiry and F/D/M intended-use acceptance are not established. Full
agent/UI/Action, native writer/authority and lifetime gates remain. No requirement,
invariant or whole priority closes. PO-07's local mechanism advances; complete
block-retirement acceptance remains separate.

## Producer role review

One producer applies all six [.roles](../../.roles/ROLE.md) checklists, not
independent reviewers or human approval. Owners: maintainers through the named
lenses/work packages. Decision: pass_with_risk for the locally qualified candidate,
pending exact-head hosted checks. This does not close full product acceptance.

| Finding / lens | Severity | Disposition and remaining evidence |
| --- | --- | --- |
| HF-01 Identity/namespace | High | Original scope constrains digest/size; new copy/store/birth and approval are exact. Scope/size/owner/job substitution refuses. Native admission and lifetime remain open. |
| HF-02 Placement policy | Critical | Content-wide claims hold across places; cancelled provenance releases no remaining claims. Held rows stay fixed. Current native/job coverage and inactivity policy remain open. |
| HF-03 Storage/custody | Critical | Recovered-provider/ended-job fixtures remove only eligible copies; shared/current bytes survive. Peak names witness-only basis. Provider/volume/large-file/intended-use gates remain open. |
| HF-04 Concurrency/recovery | Critical | Source transitions, including Prepare/Abort, invalidate remaining consent under live clearance. Five exits and offline stop preserve truthful effects. Restore/authority/lifetime gates remain open. |
| HF-05 Provider/privacy | High | Private minima and separate original/new jobs; no discovery, credentials, writer on apply or initialization. OS/provider isolation and revocation remain open. |
| HF-06 Git/adoption | High | Default-disabled CLI/new approval; original operator wire and source commands stay unchanged. Full native/agent/UI/Action and acceptance remain open. |
| HF-07 Resource admission, custody/recovery | Major | Review found eager all-action admission before a bounded call. Existing provider execution now admits each attempted action; a one-action regression observes exactly one admission. Representative resource/latency qualification remains open. |
