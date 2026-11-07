# Finish version and delivery management

Status: implementation plan, producer-reviewed design; not runtime acceptance.
Input: Tig `1dfa6db6199a226661601e2e829dcbd9ad56c2f1`.
The [common delivery review](COMMON-ASSET-DELIVERY-REVIEW.md) supplies the detailed
delivery model. This plan closes the remaining revision/selection/storage seams.
It supplements existing VTRACE controls; it does not close their open requirements.

## Contract decisions

One real asset keeps one stable ID and one small `.tig` file across edits,
production runs, selections and deliveries. Its domain definition stays separate.
An independently editable/exported representation may be a separate asset; another
delivery destination alone is not another asset. Project status values such as
`golden` belong to the admitted project vocabulary, never to Tig's field names.

Keep three independent identities:

| Identity | What changes it | What does not |
| --- | --- | --- |
| Git record revision | Any committed authored/pointer/history change | An unrecorded external observation |
| Production revision | An explicitly registered production instance with exact result/input/provenance pins | Merely adding a receipt or changing a target policy |
| Delivery event | An actual attempt/observation against an exact target and representation revision | Merely selecting a new local output |

Two different production runs can have different revision identities even when
their payload bytes are identical. Payload objects deduplicate by digest/length;
revision manifests preserve run identity and evidence. An imported revision may
have unknown production correspondence; never fabricate a producer claim.

Use the existing bounded envelope and named stream pins. The proposed common
`metadata.revisions` capability names a versioned revision-history root and an
optional selected-revision reference. `metadata.delivery` names the policy and
delivery-history slots described in the companion review. Exact serialized field
names/version identifiers and limits are frozen in T1 before use. These labels
are not validated merely because today's metadata reader accepts them. No silent
top-level extension, permissive fallback or premature consumer project mass rewrite.

A production revision manifest pins exact result representations, input/recipe
witnesses, producer/run provenance and parent revision identities. It excludes
its own eventual record/history hash and delivery evidence. Selection and approval
decisions reference the immutable revision; they do not mutate its manifest or
retroactively certify its recipe. Selected-byte pins must agree with the selected
revision's explicit representation. Conflicts are errors, not inferred repairs.

The current authored source may advance while an earlier produced revision stays
selected and is reported stale. That truth may merge under project policy; a
delivery gate can separately require freshness/approval. No generic operation
automatically approves a draft or advances selection. A delivery can explicitly
use a retained revision without rolling back selection.

Both histories use bounded immutable asset-side pages/trees and small Git root
pins. Declare complete reference closures and coverage; missing pages or an
unimplemented adapter mean unknown, not empty history or zero references. Old
revision input witnesses must resolve frozen inputs without relying on today's
moving asset IDs or the continued presence of an old checkout. Reuse existing
archived-context/produced/publication contracts rather than inventing another
recipe, receipt parser, custody database or cache engine.

## The lifecycle and placement boundary

```text
edit source -> produce B -> register protected draft B -> select B -> deliver B
                    |                |                     |           |
                 exact run       A still selected       A retained   target event
                 + input pins    unless selected        as history   + receipt

current + authorized drafts + other branches/jobs -> working-place claims
retained revisions + their frozen input/evidence closures -> retention claims
all claims + capacity + fresh custody observations -> existing placement plan
explicit authority + revalidation + scoped fence -> existing apply/journal
```

Superseded, withdrawn, no longer needed in a working tier, replica-evictable and
finally disposable are different states. Historical edges require retained
recoverability without transitively demanding the fastest working tier. A new
selection only proposes a demand change. Shared inputs, parallel branches/jobs,
authorized drafts and retention policies remain in the union of obligations.

Before a temporary job/draft protection is released, persist and acknowledge its
replacement retained/working claims. Retry/recovery must survive a crash at each
handoff boundary; forgetting to release old working claims also fails acceptance
because it prevents reclamation. Unknown coverage blocks destructive planning.
No publication receipt proves independent custody. Existing admitted evidence
adapters may supply custody only under their explicit trust/freshness policy.

Capacity conflicts remain explicit unsatisfied constraints. An old revision can
leave a working place only after the existing engine verifies required retained
copies, current claims, scoped authority and fresh plan fences. Measure physical
reclaim separately from logical bytes. No global writer/render lock; unrelated
ingest must continue. Nothing in this plan authorizes live removal or final deletion.

## Three bounded Tig PRs

Owners below mean implementation responsibilities, not invented independent people.
Each PR records its actual input SHA, changed paths, exact test names/results and
review findings. Do not expand a PR into new UI, transport, renderer or provider work.

| PR | Owner and allowed scope | Entry | Exit |
| --- | --- | --- | --- |
| T1: common contracts | `tig-assets` maintainer; pure optional asset types/readers/history operations, specs, synthetic tests | This plan; current instance/recipe/produced/publication contracts; fixed schema/version/limit vectors | Image, audio and non-media document use one revision/delivery contract; bounded history closure, selection checks, event union/conflict and old-record compatibility pass |
| T2: explicit operations | Native CLI/Git maintainer; existing `tig-record` family and reusable proposal interfaces and synthetic fixtures | Qualified T1; explicit operation and expected-old-record contract | Validate/inspect, register revision, propose selection and record delivery create immutable bounded history plus sparse Git proposals; stale base refuses; retry is idempotent; no implicit network/Git mutation/cache work |
| T3: lifecycle bridge | Claim/engine maintainer; adapt classified closures to existing claim/handoff/planner/executor seams and temporary-store fixtures | Qualified T2; known closure/coverage and policy generations; explicit scoped authority | Current/draft/history demands map without holding all history hot; old claim handoff recovers; stale/unknown/custody failures refuse removal; unrelated additions continue; actual reclaim measured in temporary stores |

T1/T2 trace WP-010/011/012/015, REQ-001/006/010, SPEC-010/011/012/014/015,
IF-010/011/013, PKG-006/008; use RUST and INTEGRATION profiles. T3 additionally
traces WP-006/007/009, REQ-002/003/004/005/009, SPEC-008/009 and existing
claim/provider interfaces; use GIT/PROVIDER profiles at those seams. At entry,
resolve the exact existing interface/package IDs against the current baseline;
record any additional impacted controls before code, not after tests.

L0 for every PR: repository validator, whitespace and role/trace review; code also
fmt. L1: locked all-feature workspace tests and strict all-target Clippy, with
nonzero named new cases. Preserve standalone Git/LFS by checking `tig-git` without
default features. L2: real explicit CLI in isolated temporary Git repositories;
T3 adds temporary-store interruption, capacity, concurrency and physical reclaim
checks. No user media/stores or live provider needed to prove these mechanisms.
Required Windows/Linux CI must pass on the exact merge candidate; requalify
changed source and compare the merged tree with the qualified tree.

## Acceptance cases, not yet executed

| ID | Fixture/action | Required assertion / PR |
| --- | --- | --- |
| VD01 | Register two runs with identical bytes | Distinct revisions, one payload object; no invented production proof / T1 |
| VD02 | Receipt-only update | Delivery history advances, production recipe/revision remains identical / T1 |
| VD03 | New source, old selected output; draft then selection | Honest stale state, no automatic approval; exact selected pins checked / T1,T2 |
| VD04 | Different language/profile or retained subject | Exact representation required, no receipt substitution or selection rollback / T1 |
| VD05 | Concurrent target histories and retries | Union preserves independent events; retry identity stable; same-target ambiguity explicit / T1,T2 |
| VD06 | Missing/corrupt history or unknown closure | Bounded failure; no empty-history/refcount fallback / T1,T3 |
| VD07 | Thousands of events | Bounded page/record limits; small root-only Git change; no full-ledger rewrite / T1,T2 |
| VD08 | Image, cue and non-media document | Same common validator, original domain schemas retained / T1,T2 |
| VD09 | Stale proposal and mixed-stream Git merge | Expected-base refusal or valid domain-checked merge; never silently repaired / T2 |
| VD10 | Replace five of 1,000 inputs; one still shared | Only new bytes acquired; shared input remains working; four eligible demotions / T3 |
| VD11 | Another branch/job and authorized draft | Protections union; draft expiry is explicit policy/recovery, not assumed absence / T3 |
| VD12 | Job-to-retained handoff, crash and replay | No gap in custody; no permanent leaked temporary working claims / T3 |
| VD13 | Stale plan, bad retained copy, concurrent ingest | Removal refused safely; unrelated additions succeed / T3 |
| VD14 | Retained revision restoration after moving source IDs | Frozen input/result/evidence closure resolves without old checkout / T1,T3 |
| VD15 | Demotion and rollback | Physical reclaimed bytes measured; protected historical version restores exactly / T3 |
| VD16 | No delivery configured / legacy metadata | Optional capability; original records remain valid; no invented never-delivered assertion / T1,T2 |

## Six-role producer review

Method: manually apply all [role checklists](../.roles/ROLE.md) to this plan and
the companion common-delivery review, against MODEL, SAFETY, publication evidence
and VTRACE package/profile boundaries. Findings are design review, not independent
approval, executed cases or permission for live operations.

| Role / finding | Severity | Owner | Disposition and evidence still required |
| --- | --- | --- | --- |
| Identity: conflating Git, run and delivery revisions loses same-byte runs or creates cycles | High | T1 | Addressed in design above; VD01/02/04/14 still required |
| Placement: historical graph edges make every old input hot | High | T3 | Working and retained claim classes separated; VD10/11/15 still required |
| Custody: supersession/receipt treated as deletion authority; missing closures disappear | Critical | T1,T3 | Unknown blocks removal, frozen closure and fresh custody required; VD06/12/13/14 still required |
| Concurrency: last-writer history roots, stale selection and leaked/gapped job claims | High | T1,T2,T3 | Causal merge, expected-base proposals and acknowledged handoff; VD05/09/11/12/13 still required |
| Provider/privacy: metadata acceptance mistaken for trust or provider guarantees | High | T1,T2 | Admitted common validator; explicit capabilities; sanitized diagnostics, private raw evidence; negative CLI/provider fixtures remain required |
| Git/adoption: unbounded ledgers or converting every receipt into another asset | High | T1,T2 and consumer migration owner | Bounded roots and stable actual assets; VD07/08/16 and measured real migration/rollback still required |

Plan disposition: suitable to implement in the three bounded slices; runtime
closure remains open. No new safety guarantee is claimed by landing this prose.

## Consumer landing and completion

The consumer owner maintains the private mapping/coverage/cutover plan, not Tig's public fixtures:
Consumer-specific landing records remain with the project owner.
Pilot one actual episode/render with its cel/cue and one text asset after T2;
then batch by real object identity, preserving IDs, pins, rights and approval
subjects. No mass rerender or fresh remote census is a prerequisite to importing
retained truth; remote currentness remains unknown until separately observed.

Plan landed, code qualified, consumer project records converted, delivery coverage reconciled,
and each live lane accepted are separate milestones. Full migration is finished
only when required coverage is explicit and normal new-version/select/deliver/
restore operations plus source-independent lane operation/rollback pass. Optional
REEL features, UI, torrent transport and new remote providers are separate roadmaps.
