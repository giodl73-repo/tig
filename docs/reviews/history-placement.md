# Producer review: classified history placement

Public summary: consumer identifiers generalized; original notes retained privately.

Target/input/controls: [HISTORY-PLACEMENT](../../specs/HISTORY-PLACEMENT.md).
Six [roles](../../.roles/ROLE.md) applied as producer checklists, not independent
review, consumer approval or deletion authority.

| Role | Finding, severity and disposition |
| --- | --- |
| Identity | High: moving IDs must not rewrite old input identity. Only exact checked revision manifests and admitted immutable witness closures feed placement; source/record/policy pins stay explicit. Same-byte runs retain distinct manifests. |
| Placement | High: retaining history must not imply keeping it hot. Selected and explicitly authorized draft revision closures are working; all history is retained. Another branch/job composes as an independent ordinary Claim. No project status or drive letters are built in. |
| Custody | Critical: classification is not protection. Unknown/partial/unclassified closures refuse. Results deny complete claim coverage and eviction authority. Provider verification and independent custody remain later T3 gates. |
| Recovery | High: no claim retirement may follow pure classification. This slice performs no IO or journal changes. Durable successor acknowledgment, interrupted handoff and reclaim remain open, not inferred from planner candidates. |
| Provider/privacy | High: resolver is an explicitly admitted trust boundary with bounded acquisition, not implicit filesystem discovery. Deduplicate witness resolution; cap unique objects, logical bytes and cumulative references including repeated closures. Errors are static; demand inventories have no Debug/Serialize. |
| Git/adoption | High: a source-valid record can have an unclassified or old result. Refuse unclassified selection; use frozen selected provenance rather than latest authored IDs. No record writes or implicit selection occur. |

CC-HISTORY-SOURCE: prior ~3.14 MB ordinary code/docs leaves ~5.7 KiB, insufficient
for adapter plus meaningful fixtures. Approve producer-controlled source ceiling
of 3.25 MiB (+256 KiB), with unchanged 128 KiB per-file and runtime/media bans.
This accommodates code/tests/controls, never large asset inventories or receipts.
No consumer project or cache capacity is changed. Actual measured size and test results
must accompany qualification; this review does not waive the validator.

Disposition: suitable for isolated candidate qualification. Current proof scope
is classification/planning only. T3 durable bridge, physical reclaim/restore,
actual consumer mapping and cutover remain open.

Seven new real-Git fixture cases pass: five-of-1,000 replacement plus another job
leaves exactly four old input working placements eligible in the scoped pure
planner; unknown coverage or missing retained placement blocks that candidate.
Explicit draft admission keeps both versions working without changing selection.
Old/delivery witness gaps, partial/untracked/unclassified histories, byte/object
bounds, conflicting sizes and cumulative repeated-closure reference exhaustion
refuse. These are synthetic assignment observations, not actual custody or
physical-reclaim evidence.
PR77 follow-up: full local locked all-feature workspace and standalone Git checks
pass on b935170; strict Clippy and L0 pass. Exact Windows/Linux hosted checks are
separate from these results; this does not close the later T3 obligations above.
