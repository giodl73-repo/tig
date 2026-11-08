# Six-role storage roadmap and development handoff review

Input 44b7dabf3b7bd913a5d9e448e20623001697a0f6. DOCS profile;
WP-003/004/005/006/007/008/009/011/012/013/015/016;
REQ-001/002/003/004/005/006/007/008/009/010; SPEC-002..014.
Targets: PRODUCT_PLAN.md next sequence and fixture design; AGENTS.md, CONTEXT.md,
docs/DEVELOPMENT.md and context/RESUME.md for self-contained future startup.
No runtime/provider mutation or new implementation claim.

Method: producer-applied checklists from .roles/ROLE.md and all six parliament
roles, against specs/MODEL.md, specs/SAFETY.md and foundation scenarios. This is
one producer's review, not six independent agents or approval for live operations.
Entry: the copy/verify/retain/evict roadmap and existing component evidence.
Exit/L0: explicit dispositions, local links, source budget, foundation hashes and
diff checks. L1/L2: not applicable to prose; all new lifecycle tests remain planned.

| Finding / role | Severity | Required correction / design disposition | Implementation proof still required |
| --- | --- | --- | --- |
| SR-01 Identity and Namespace | High | Bind plan and retry IDs to exact object hash/length, store instance/generation, source/policy/claim basis and provider contract; conflicting reuse refuses. Added to priority 2 and contract. | Changed store root/generation, operation-ID collision and source drift refuse without destructive effects. |
| SR-02 Placement Policy | High | Eviction cannot precede complete scoped branch/draft/job/retention admission merely because automatic Git reconciliation is later. Put minimum admission in priority 2, require it in 4; incomplete scope permits copy-only progress. Add explicit precedence, infeasibility and peak reservation. | Shared branch/active job, conflicting demands, unknown coverage and insufficient peak capacity preserve protection; no anticipated reclaim funds admission. |
| SR-03 Storage and Custody | High | Verification is not retained independent custody. Require declared replica count/domain provenance and alias refusal; first executor excludes final-archive deletion. Separate logical/file/allocation and capacity observations. | Destination loss, aliases, shared packs and restore after eviction demonstrate surviving bytes and honest reclaim. |
| SR-04 Concurrency and Recovery | High | Fresh checks must remain protected through removal; a check/delete race is insufficient. Define scoped fencing, durable idempotent operations, cancellation and uncertain restart states, with unrelated writers progressing. | Claim arrival at final boundary, interrupted copy/removal, lost acknowledgement, cancellation and fresh-process recovery; no blind destructive retry. |
| SR-05 Provider and Privacy | High | Boolean capabilities alone hide acknowledgement/retention strength. Admit provider-specific guarantees; hold unsupported removal. Bound apply/default dry run, credential handling, path safety and external-writer assumptions. | Credential denial, replaced roots, path/symlink escape, unsupported fence and untrusted remote acknowledgement refuse safely; separately qualify real providers. |
| SR-06 Git and Adoption | Medium | Make startup independent of old chat/consumer checkout. Link a bounded handoff and startup path; derive live branch/PR state, use external build/temp roots, preserve other lanes and running tools. Keep journals/large plans machine-side. | Documentation links/checks now; next fresh session verifies source/PR state. No deployment, cache init or automatic hook authority follows from reading files. |

All six corrections are incorporated in the revised plan/startup documentation.
Owner: Tig maintainer, using the named role as the review lens. Disposition:
pass_with_risk for the revised design only. No unresolved design blocker within
this scope; runtime milestones remain open until their specified tests execute.
This does not close full work packages, independent review or human acceptance.

The prior roadmap checkpoint reran 20 replica entries, 16 handoff entries and six
real LFS client cases successfully. These scoped component results are unchanged;
they do not execute the proposed copy-to-evict lifecycle. No Cargo rerun is
required for this documentation-only revision. Protected CI and portable L0
checks remain distinct from those historical component results.

Startup walkthrough: resolve the actual checkout; inspect branch/dirty state/
worktrees/PRs; read resume state and ordered roadmap; select an unowned bounded
slice; configure external build/temp roots; run its checks; update handoff when
priorities change. Machine-local checkout paths are deliberately not in public
source. Canonical source can therefore be relocated without rewriting this guide.
