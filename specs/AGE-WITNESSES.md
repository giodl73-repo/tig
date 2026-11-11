# Recorded age of exact historical occurrences

Input0dbc41f318b5acffacb3ed6562df839ae1ffe9d9.
WP-003/004/006/007/008/009/010/013/015;
REQ-001/002/003/004/005/006/007/009/010;
SPEC-002/003/004/005/008/009/010/013/014;
IF-002/004/005/006/008/010/012/013; PKG-002/003/004/006/008/009.
RUST/PROVIDER/GIT profiles. Allowed: metadata-only age capture/status/assessment,
explicit initialization of an already enrolled registry source, verified owned
subset inheritance, explicit private-profile CLI/native age selection and owned
synthetic fixtures. Reuse existing MetadataStore/Tree, Namespace/SourceJournal and
Clock ports; no new SQL/event/cache engine. No payload IO/removal, live root,
daemon, implicit source enrollment, migration, final approval or deployment.
Entry: independently admitted current source/registry/metadata/clock identities
and minima, private writer boundary and existing roots. Exit: actual observation
before clock sampling, acknowledged immutable facts across restart/retry/races,
unknown age held, inherited age proved, bounded exhaustion and actual CLI/native
selection. Close implementation, verification, synthetic validation, trace and
six producer lenses separately. L0: fmt, pins, validator/diff; L1: independent
framing, focused/default/all-feature/Clippy/standalone; L2: actual temporary
metadata/journals, native clock, process exits, peer races and native selection.

## Authority and identity

An Enrollment binds authority/epoch, metadata provider, registry namespace/policy,
fixed registry source/owner, clock identity and sorted admitted observer namespace/
policy/owner/class scopes. Only Branch/Draft predecessors qualify for observation;
Job/Retention protection cannot be aged away. Runtime budgets and advancing trusted
minima are separate from that immutable identity. All namespaces/providers are
opened with independently current private minima; a portable request cannot lower
them. Missing authority/clock/coverage never becomes age zero.

The registry is one explicitly initialized Retention-class source over an existing
asset-side immutable Tree. Its identity leaf and Selection context bind enrollment;
each age key hashes namespace/source/predecessor revision. A document also binds
the exact root/context, owner/class and observed source checkpoint. It is an exact
historical occurrence, not a permanent content-hash label. Reuse/reactivation
produces independent current/recent obligations and cannot reuse an old occurrence's
receipt. Metadata documents and root history stay outside project Git.

Only the privately admitted capture/inheritance writer may adopt registry roots.
Registry custody/admission is the provenance boundary: a document/hash or a clock
digest does not authenticate its own timestamp. CLI capture accepts no timestamp
or precomputed age document. Existing generic namespace APIs are trusted substrate,
not authentication of an arbitrary owner string. Qualify the actual writer/OS
boundary before live adoption; malicious administrators and globally unseen
restoration remain outside the bounded local guarantee.

## Capture, retry and bounded registry lifetime

Explicit init requires an already enrolled, empty Retention source of the admitted
owner; no source/store is discovered or created. Metadata identity is finished and
read back before existing atomic Prepare/Adopt. Repeated init preserves subsequent
captures and never resets the ledger.

Capture reads the actual requested predecessor, verifies observer enrollment and
source owner/class, then samples the exact admitted Clock bounds. The sampled
latest value is a conservative upper bound on when that occurrence was already
historical. It is an observation time, never a fabricated original merge date.
Persist the immutable document and edited registry root through MetadataStore,
finish/read back, then use existing atomic Namespace::select with stable keys.
No transaction spans clock/metadata IO. Competing captures use scoped existing
admission and optimistic source revisions; no global cache lock.
Nonblocking contention may leave every initial caller unacknowledged. Callers
retry explicitly after contention clears; no per-round winner or scheduling
fairness is promised. An already adopted fact must replay unchanged without a
new clock sample. If none was adopted, retry samples later, conservatively.

An acknowledged same-occurrence retry returns the strongest retained verified
observation without sampling a new clock or replacing any acknowledged document.
Captured facts keep their original primary occurrence key. Inherited facts append
under a domain-separated key binding occurrence, parent evidence and subset plan;
legacy primary inherited facts remain accepted. A later observer's direct capture
cannot suppress an older proved inherited observation. Assessment selects the
smallest verified latest bound, then earliest bound and evidence digest for ties.
Search and ancestry share the operation budget; missing/corrupt records or
exhaustion hold rather than silently selecting incomplete evidence. Record/event
encodings and enrollment identities remain unchanged. Legacy readers may miss an
alternate proof and conservatively report unknown or younger age.
A crash before registry adoption may
leave unacknowledged metadata; retry can only observe later, conservatively. A
crash after atomic adoption recovers the exact acknowledged document. Conflicting
pending/foreign state holds. Older registry predecessors retire only after the
current root demonstrably retains every old binding; interrupted cleanup resumes
without resetting source identities or dropping known facts. Status never cleans,
repairs, initializes or appends a command.

Bound metadata reads/writes/bytes, record count, source admission work, lineage
depth and cooperative elapsed time. One shared metadata counter/time boundary spans
the whole call, including cleanup, construction, independent persisted readback,
source-admission tree reads and final status/lineage verification. Changing phases
does not renew that budget. Existing journal command/byte bounds remain;
safe exhaustion holds without resetting history. Continuation and eventual audit/
dependency-aware registry-entry expiry remain required before sustained use.

## Inheritance and assessment

Owned subset inheritance requires an existing admitted parent record, exact
original occurrence, recomputed unchanged survivor projection, and actual source
Enroll/Prepare/Adopt/Withdraw/Retire receipts for the declared subset plan. Header/hash
equality alone is insufficient. Store parent and plan references; verify bounded
lineage and source-checkpoint ancestry on assessment. New independent occurrences
have no inherited age. Current eligibility still requires the leaf occurrence to
exist; historical receipts never recreate retired claims.
Inheritance may acknowledge this historical fact after a later owned page retires
the survivor, provided the complete historical enrollment/transition/projection
proof remains available. That cannot make a missing occurrence eligible again.

Assessment consumes a current admitted clock interval. Reject invalid/rollback
bounds; old enough requires now.earliest >= observed.latest + required_age with
checked canonical-number arithmetic. Overlapping intervals prove no positive age.
Unknown record, missing occurrence, corrupt/missing evidence and unavailable clock
stay distinct. Reports grant neither claim-release nor final-removal authority.
Native age selection must bind this exact occurrence to its existing native
header/path proof and retain independent final plan/policy/window approval.

Metadata GC must preserve registry identity, record/parent/plan and referenced
source-root dependencies or hold expiry. Full native source/writer admission,
current authority/host trust/restore, UI/Action, real providers/large files,
journal continuation, metadata retirement and human acceptance remain open.

NB-07 remains a separate gate: paged final retirement needs exact global consent
and separately authorized historical-claim-only settlement. Age evidence does not
change current held-member semantics or make repeated bounded final batches a
solution for shared content spanning more than MAX_ROOTS historical roots.
