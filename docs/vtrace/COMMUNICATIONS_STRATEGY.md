# Communications strategy

Explain accepted product behavior and honest gaps. Engineering package/gate
records are not product CLI features. Existing ASCII guides show targets; no
diagram or planned command implies a working mount/provider/daemon.

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| COMMS-001 | NEED-001/005; REQ-001/008/010 | Git/LFS and asset users | What is Tig, and do I need typed assets? | README.md, docs/THEORY.md, docs/UNIFIED-DESIGN.md | Each capability/boundary change | git-adoption | maintained target/current explanation |
| COMMS-002 | NEED-001/004; SPEC-006/007/008 | Creators and CI operators | How do edits, pointers and merges affect bytes? | docs/guides/BYTE-WORKTREE.md, BRANCHES.md, LOOKUP-AND-LFS.md, GIT-LIFECYCLE.md | Each interface/workflow change | git-adoption | target walkthroughs |
| COMMS-003 | NEED-002/003; REQ-003/005; TIG-P-09 | Storage operators | What can be evicted, and what protects my bytes? | docs/SAFETY-PROMISE.md, guides/PLACEMENT-AND-RECOVERY.md, TREES-AND-BUNDLES.md | Each custody/ack/resource change | storage-custody | target obligation with explicit proof limits |
| COMMS-004 | REQ-001..010; SPEC-001/015 | Implementers/reviewers | What is proven versus designed, and what is next? | docs/vtrace, specs/HANDOFF-PROTOTYPE.md, docs/scenarios/LEARNINGS.md | Each package closeout | identity-namespace | scoped mechanism evidence; full requirements OPEN |

Private consumer examples stay in their owning repositories/stores. Public or
shared exports omit private prompts/media/credentials and attribute any source
licenses. Native/browser reports must distinguish desired, planned and observed
state, unknown coverage and independent custody. Current implementation counts
are checked against real source/tests, not aspirational package totals.

## Stage review

Producer-applied privacy, identity and Git lenses: pass_with_risk. COM-01: users
could infer a live provider from a polished guide. Resolved by target labels and
explicit current evidence links; update user docs when each capability actually
ships rather than advertising work-package completion as product functionality.
