# Explicit final retirement of managed content

Input59174b1e31eeeb8e2fe0d29160233d7a0345bf05.
WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014;
IF-002/003/004/005/006/008/012/013; PKG-002/003/004/007/008/009.
RUST/PROVIDER profiles. This contract distinguishes current claim-clearance
implementation from the remaining final-deletion executor. No live operation,
retention release, payload deletion or provider upgrade is authorized here.

Entry for the current slice: privately admitted complete flat-binding namespace
roster, immutable namespace/policy identities, trusted journal minima, exact
object and finite aggregate metadata budgets. Exit: all-namespace live membership
clearance, truthful holds, races/process-exit proof and no removal grant.
Provider mutation has additional entry gates below. Implementation, verification,
synthetic validation, trace and six producer lenses close separately.

## Four separate decisions

1. Select exact eligible asset versions and admit their source/age/category basis.
2. Settle only the selected owners' obligations, preserving unselected/held siblings
   through [retained-subset handoff](RETAINED-SUBSET.md).
3. Explicitly release eligible provider retention and remove authorized exact
   placements, possibly ending recoverability in the declared managed scope.
4. Retire detailed metadata after audit/dependency retention and replay fencing.

None follows automatically from its predecessor. A pointer remaining in old Git
history is not a permanent retention obligation. Conversely, removing a pointer,
losing a label, observing age or reaching low free space grants no deletion power.
Project schemas/adapters define categories and versions; golden is not a core enum.

## Final protection is content-wide

Ordinary replica eviction checks whether each namespace still requires the target
place, then independently holds surviving custody. Final deletion instead needs
absence of the exact object from every admitted source obligation in the content
scope, including destinations other than the target store. A job needing F blocks
deletion of M even when M is not assigned to that job and F is currently absent.
Do not reuse target-place absence or disable the ordinary keeper requirement.

The private final authority admits a complete content-scope namespace roster,
including authorities that can protect another place. It must cover each target
store's namespace binding; that store binding alone need not cover the whole
content scope. Missing/extra/substituted handles, unknown scope, policy mismatch,
unavailable authorities or unresolved authority handoff hold. A caller-supplied
coverage digest identifies the admitted declaration; hashing it is not proof of
global completeness. Unmanaged writers/pruners require an explicit boundary.
The executor must match the scope digest to its independently enrolled final
policy; a request-provided digest cannot grant itself authority.
Each namespace pin includes an independently admitted trusted minimum checkpoint.
Refresh and verify exact checkpoint ancestry even when a supplied handle was
opened against a weaker minimum. Revalidate the observed checked checkpoint
before use, so known rollback or a changed chain holds. These minima detect
known rollback, not every globally unseen restore or uncooperative writer.

Only admitted complete flat binding closures qualify this first clearance port.
Native dependency/context roots need their admitted complete extraction before
joining the same mechanism; interpreting arbitrary metadata as payload closure
is forbidden. Missing/corrupt nodes, size conflict and exceeded aggregate bounds
are unknown, not absence. Check all current, pending and predecessor roots of all
registered Branch/Draft/Job/Retention sources, regardless of proximity precedence.

Acquire existing subject fences in canonical namespace order, then traverse under
one aggregate deadline/root/binding/association/read/byte budget. Mandatory source
mutation uses those same partitions. A new overlapping prepare/adopt/withdraw/
retire cannot invalidate clearance while held; unrelated partitions can advance.
Revalidate namespace markers/policy at entry and before use. No transaction spans
metadata IO and no global cache lock is introduced. Return a non-Clone, non-
serializable live guard; its serializable summary is observation only.
The deadline bounds acquisition cooperatively between/after provider calls; it
cannot interrupt blocked IO and is not a lease that expires protection. The
authority/coverage/member/format scope digest excludes per-call budgets, while
the full roster-profile digest includes them. Advancing trusted minima is also
excluded from the authority-scope digest and retained in the concrete profile.
A changed scope needs independent
admission; increasing a read budget cannot grant deletion authority.
Excluding minima from the scope hash must never let a request lower privately
admitted witnesses. The executor constructs the concrete roster from its trusted
profile/current witness source, then enforces any additional exact plan minima;
it does not accept request-chosen weaker minima merely because a scope hash
matches. Scope equality does not authenticate the concrete roster or make an
old private profile globally current. Missing current authority/witness admission
holds the destructive executor.
The aggregate counters cover tree metadata, not SQL journal replay. Each of at
most sixteen journals retains its existing 16,384-command / 16 MiB replay bound.

`final_retirement::claims` provides this bounded roster/clearance API. It opens no
payload provider and grants neither retention release nor removal. A future
executor must additionally hold the exact provider/incarnation guard and satisfy
every condition below through the destructive step. Dropping a guard releases
only mutual exclusion; it neither cancels durable intent nor authorizes IO.

## Reviewable batch and owner authority

A bounded batch names exact source snapshots/version IDs, logical binding paths,
object hash/length and provider store/binding generation plus physical incarnation.
Use definition/adaptor-owned categories and recorded supersession/retirement age
events under an explicit admitted clock. Unknown age holds an age filter. Git
commit/file modification times are not trusted retirement ages by default.

Before source settlement, persist the exact batch intent and owner consents under
admitted metadata capacity. A source subset receipt is an effect, not payload
deletion approval. Preview per-version and per-object outcomes, held reasons,
sharing outside the batch, scope gaps, stores, remaining protection and deliberate
loss of recoverability. Counts distinguish asset versions, unique objects, logical
bytes, copy bytes and allocated bytes. Shared bytes are eligible only after all
surviving admitted obligations end. Held items are excluded from deletion consent;
later becoming eligible requires a new admitted basis, not silent batch expansion.

Deletion consent binds the exact immutable plan, final authority/epoch and policy,
eligible object/store/incarnation set, releasable grant identities/owners,
claim/source/selection basis, clock policy and validity window. Private authority
is admitted independently of Git/workflow content. Wrong owner, non-releasable
hold, retention deadline, new use, changed selection/policy/authority, unknown
clock or expired consent holds affected pending actions. Unrelated source changes
need not invalidate an unchanged object predicate. Historical completed effects
remain historical. Renewal may not mutate or reinterpret an old operation.

## Local provider state and compatibility gate

The writable implementation must reuse LocalStore, per-object/incarnation fences,
existing accounted metadata publication and capacity ledger; no second cache or
unbounded new event engine. Existing ordinary retirement/handoff semantics stay
unchanged. The first final-capable local profile must be explicitly versioned and
must refuse old writers that do not understand final absence/released grants.
Design-7 roots are not silently upgraded. A new manifest version by itself does
not revoke an already admitted old writer; live upgrade/quiescence and authority
handoff need separate qualification. Initial executor fixtures use fresh isolated
final-capable roots only. Real adoption/migration remains a later gate.

Freeze canonical vectors for the manifest, plan, intent, release, completion and
cancellation before admitting any new provider format. A prospective design-8
profile is under development in [FINAL-PROVIDER](FINAL-PROVIDER.md), without
production acceptance. Its authority/epoch/policy,
complete roster and supported acknowledgement are immutable enrollment inputs.
Old-reader/writer refusal and unchanged legacy behavior need actual pinned artifacts.

Final intent binds the exact current birth/placement and all owned grant records.
The active grant inventory is bounded and complete; an unexpected grant holds,
including another owner's grant without its independently admitted consent.
Enumerating active grant files is not equivalent to enumerating known receipt
copies. Released grants retain exact immutable evidence and can never be revived
by replaying old retention/publication operations. Release is an explicit owner
decision, not a successor-free use of the ordinary handoff contract.

Required local action order, under live content-scope and provider guards:

```text
admit metadata/recovery peak and exact allocation/job binding
  -> revalidate authority/window, namespace scope, bytes/birth and exact grants
  -> persist immutable final intent
  -> revalidate pending-action authority/protection
  -> persist exact grant-release evidence
  -> remove only those active grant files (idempotent exact recovery)
  -> revalidate current authority, source absence, birth and provider retention
  -> remove the exact managed payload representation
  -> persist completion/tombstone and settle measured metadata allocation
```

Intent and completion/recovery capacity precede destructive IO. A full journal,
record inventory or provider metadata budget refuses before irreversible work.
Provider locks and readers remain effective; no final authority overrides an
uncooperative provider's declared retention capabilities. Finite birth/history,
grant, action and byte bounds must be named/measured in the implementation profile.
No prospective reclaim finances admitted work. Process-restart acknowledgement
does not imply power-loss, native quota or hostile-filesystem safety.

## Cancellation and uncertain completion

| Last durable effect | Cancellation/recovery behavior |
| --- | --- |
| Batch intent only, no source settlement | Stop pending work and record cancellation; no claim rollback is needed. |
| Owned source history settled | Record version retired but payload present/unknown and remaining provider protection, or perform separately owner-authorized source restoration. Never report unqualified rollback. |
| Final intent, no grant release | Cancel under the same object fence and preserve original grants/bytes. Exact cancellation fences the old delete retry. |
| Grant release durable, bytes still present | Stop removal, record retired-but-present and remaining protection. Released grants stay released; restoration requires fresh owner-authorized grants/operations. |
| Bytes absent under exact durable intent/release evidence | Recover observed authorized absence and historical effects; do not claim successful rollback, fresh deletion or complete current protection. A new claim may now be protected-but-unavailable. |
| Current birth/placement differs | Never act on the new incarnation. Recover only exact old historical outcomes, otherwise report stale/uncertain. |

Cancellation is an exact current-owner/control decision. Expired deletion consent
grants no new unlink and does not itself prevent safe current-owner cancellation.
Recovery of recorded historical effects is not fresh deletion approval.
Cancellation before release must not itself release grants. A durable release
marker cannot become valid after a pre-release cancellation. Interrupted grant-
file cleanup remains exactly resumable; missing evidence without an authorized
matching marker is an integrity incident. Recovering an already absent exact
placement is distinct from authorizing a fresh unlink: new use or expiry holds
new IO but cannot erase a completed historical loss. Reacquisition is a fresh
explicit operation with a new birth; old deletion retries cannot remove it.

## Deliberate absence and reconciliation

The placement retained-backup floor currently covers all known managed objects.
After authorized final retirement it must not silently restore released grants
or rehydrate deliberately removed historical content solely because it was once
known. Preserve and consult incarnation-bound retirement evidence; never suppress
an object's future legitimate identity globally. Active pending final intent,
retired-but-present cancellation, completed absence, unexplained absence and a
fresh selected demand must have distinct observations.

Fresh demand/revert for deleted bytes reports unavailable/reacquisition required
until explicitly authorized exact-byte acquisition succeeds. Ordinary configured
copy/reacquire permission is not automatically final-reacquisition authority.
Never silently substitute a different version, revive a released grant, repair
missing retirement evidence under spent resources or reinterpret an old receipt
as fresh protection. Unknown/offline stores are not reported deleted. Managed
namespace removal, provider versions/trash/snapshots and measured capacity release
are separate; unlink is neither secure erasure nor proof about unknown copies.

## Later metadata lifetime

Keep compact exact deletion evidence for an admitted audit/recovery window; large
batch inventories/receipts remain asset-side under separate budgets. Metadata
retirement needs dependency/audit expiry and an enforceable authority/epoch or
equivalent replay fence. Time alone cannot make old operations safe. An aggregate
trusted fence may outlive per-asset records. Old Git commits remain untouched;
historical lookup reports deliberate retirement only while evidence supports it,
otherwise unknown/unavailable. Old peers/restored backups must not revive retired
operations; new explicit ingestion of identical bytes remains possible.

## Required qualification matrix

| Case | Counterexample / required evidence | State |
| --- | --- | --- |
| FR-01 | M is locally surplus but a current job needs F; content-wide membership holds. | Current clearance fixture |
| FR-02 | Current/pending/predecessor roots of all four classes and another namespace retain the object. | Current clearance fixtures |
| FR-03 | Unknown/missing/extra/duplicate/policy-substituted roster, weaker/forged minimum, restored history or missing/corrupt closure never clears. | Current clearance fixtures |
| FR-04 | New target demand is blocked while clearance lives; another partition advances; child exit releases only the fence. | Current clearance fixtures |
| FR-05 | Aggregate metadata/deadline bounds span namespaces; partial acquisition failure releases earlier guards. | Current clearance fixtures |
| FR-06 | Physical store/birth/grants changed or another owner/non-releasable hold appears before release/unlink. | Bounded local grant/birth/reader fixtures; full provider gate open |
| FR-07 | Clock expiry/rollback, authority outage or new selection/use invalidates pending removal; old completed outcomes remain. | Synthetic clock/window fixtures; current authority adoption open |
| FR-08 | Exhaust metadata capacity before intent, release, unlink or completion recovery; no unrecorded destructive IO. | Initial metadata reservation refusal; full exhaustion matrix open |
| FR-09 | Actual child exits before/after every release/unlink/completion boundary recover exact states. | Nine local process-exit boundaries; full batch/provider matrix open |
| FR-10 | Cancel before/after source settlement and provider release, including late cancellation after unlink. | Local provider cancellation fixtures; source/bulk integration open |
| FR-11 | Shared/held mixed bulk selection preserves unselected siblings and scopes approval to eligible members only. | Bulk integration gate, unexecuted |
| FR-12 | Completed/cancelled final retirement survives ordinary reconciliation without resurrection; fresh same-hash birth survives old retries. | Local reacquisition/public-placement fixtures; full adoption open |
| FR-13 | Old real artifacts refuse the final-capable profile; legacy stores/receipts and ordinary keepers remain unchanged. | Pinned pre-final CLI refusal/legacy control; live upgrade open |
| FR-14 | Actual managed removal and provider retention/versions/trash report measured effects separately. | Local logical/pre-allocation observations; actual provider/scale gate open |
| FR-15 | Audit-window retirement, journal continuation, old peer/backup replay and live epoch handoff preserve fencing. | Lifetime gate, unexecuted |

L0: fmt, portable validation, reviewed targets and diff checks. L1: locked default/
all-feature workspace, strict Clippy, standalone Git and exact focused negatives.
L2 now: actual synthetic journals/metadata and cross-process scoped fencing;
L2 later: all executor/format/placement/provider/lifetime gates above. Designed
matrix rows are not execution evidence. Full REQ/VAL acceptance remains open.

[FINAL-PROVIDER](FINAL-PROVIDER.md) and EVID-099 qualify the named bounded local
parts above at implementation9f5e8c0. Full row scopes remain open where shown;
local process-restart fixtures do not establish batch, live authority/provider,
large-file or lifetime acceptance. Historical EVID-098 remains clearance-only.
