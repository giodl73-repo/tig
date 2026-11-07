# Producer review: classified demand archive

Input/controls/exit: [PLACEMENT-ARCHIVE](../../specs/PLACEMENT-ARCHIVE.md).
Apply all six [roles](../../.roles/ROLE.md) as producer checklists; no independent
review, live-operation authority or consumer acceptance is implied.

| Role | Finding and disposition |
| --- | --- |
| Identity | High: restoring a different root/policy could rewrite history. Header context binds exact source/record/policy/places/claim/counts; raw canonical bytes and complete path/object membership are checked. Demand identity fields now have read-only getters. Old roots stay immutable. |
| Placement | High: separate working/retained trees could duplicate inventories. Store each object once, with working membership implying retained membership. Reconstructed ordinary Claim preserves both place assignments and planner limits. |
| Custody | Critical: metadata finish is not payload protection. Candidate Selection and restored Demand grant no claim completeness/custody/removal authority. Real successor protection acknowledgment and physical reclaim remain open. |
| Recovery | High: partial writes must not replace the old root. Injected 0/1/3-write failures preserve old replay; exact retry produces the same root as clean construction. SQLite finish/reopen and source pending/adopt/replay leave independent peer roots unchanged. |
| Provider/privacy | High: per-stage limits could exceed operation limits. Header read/write consumes the same count/byte budget as tree traversal/build. No payload copies or discovery; static mapped errors; private header stays asset-side through existing NodeStore. |
| Git/adoption | High: restored assignments could be mistaken for live source truth. Replay is explicitly historical desired state; exact pin comparison, current policy admission and protected owner handoff remain caller responsibilities. Fixture detaches Git before reopening. |

Five new archive cases pass with the seven classification cases (12 total):
actual SQLite finish/reopen + journal restart and historical receipt replay;
deterministic shared metadata with absent payload objects; unsupported/noncanonical/
foreign/missing/count/path/header refusal; total read/write budgets; interrupted
write and exact retry. The fixture's explicit predecessor journal retirement is
not a byte-protection or removal proof. Full workspace/Clippy/hosted qualification
remains separately recorded. Disposition pass_with_risk for desired-state metadata.
