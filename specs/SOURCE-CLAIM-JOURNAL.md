# Restart-persistent source claim transitions

Input 5411e2be0521aef0e3722e508f31ff6355b8a66f. WP-008/009; REQ-002/004/007/009;
SPEC-005/006/007/008; ARCH-002/003/004; IF-004/006/008; DES-004/005/007;
PKG-003 (tig-store module), GIT/PROVIDER profiles. Entry: exact Git extraction,
immutable tree identity and restart-persistent SQLite techniques. Exit: actual
temporary-store/source integration and child-exit tests preserve prepared/current/
predecessor roots, reject stale owners/generations and replay commands exactly.
L0 structure/fmt/VTRACE; L1 locked tests/Clippy; L2 Git-to-journal/restart fixtures.

The journal is runtime metadata outside ordinary Git. It records immutable root
references, not media payloads or another expanded inventory. Repository/source
IDs and owner authorization are explicitly enrolled by a trusted caller. Names,
remote URLs and worktree paths never automatically identify an owner. The library
does not authenticate an owner string or discover all repositories and jobs.

```text
prepare(expected revision, candidate root)
    -> current root remains + pending root is durable
    -> caller may now complete its Git/source handoff
adopt(exact prepare operation, same owner)
    -> candidate becomes current + former current becomes predecessor
abort(exact prepare operation, same owner)
    -> candidate obligation explicitly withdrawn; current remains
withdraw(expected revision, same owner, no unresolved pending handoff)
    -> current becomes predecessor; source identity/revision remains reserved
retire(exact predecessor revision, same owner)
    -> that owner explicitly withdraws that predecessor obligation
```

None of these operations deletes payloads or proves another replica exists.

`Namespace::select_predecessor` composes existing Prepare/Adopt/Withdraw commands
in one guarded transaction. It admits the before/final historical-root placement,
never exposing a transient current root that could suppress history proximity.
The three distinct operation IDs, source/owner and expected revisions remain exact;
all three command slots/bytes must fit. Exact replay recovers the final Withdraw
receipt; a partial prior request set holds rather than silently completing it.
No event encoding changes. Existing two-command select and its receipts remain
compatible. See [retained-subset handoff](RETAINED-SUBSET.md).
Current, pending and predecessor are generic claim roles; portable project policy
maps them to places. Other branches/jobs remain independent sources. A source's
absence or retirement cannot establish global claim completeness. Unregistered
drafts, unknown sources and external Git operations still need admitted coverage.

Each selection records its root Object and context digest. Context binds the
caller's exact source checkpoint, scope and interpretation/policy; the Git adapter
exports a deterministic selection digest for this purpose. Tree metadata must be
published/protected separately before acknowledgment. A referenced root is not
proof of complete or available closure: missing/corrupt nodes remain UNKNOWN.
The journal exposes root obligations and never a negative object-membership or
eviction decision. Scalable metadata storage and integrated custody remain open.

Per-source revisions fence concurrent writers. A prepare protects both roots
before a Git/ref change; an interrupted change leaves the pending claim intact.
The journal cannot make Git and SQLite one atomic transaction. Source inspection
and recovery must decide adopt/abort; no timer or old process death expires claims.
Adoption records selected truth even if output rendering or placement is pending.

Commands are canonical, hash-chained and operation-ID idempotent. Exact historical
retries return the original outcome without repeating a transition. Changed reuse,
foreign owner, stale revision, wrong pending reference and exceeded history bounds
reject atomically. State is adopted in memory only after database commit; uncertain
failures poison the handle until verified reopen. Short SQLite transactions use
nonblocking Busy; no transaction spans Git, hashing media or file transfer.

Open requires an external trusted minimum checkpoint. Hash-chain replay detects
known rollback, gaps and corruption; it cannot prove the minimum is globally
latest or repair restored authoritative protection. Process restart is the tested
acknowledgment scope, not power-loss/provider durability or hostile filesystem
security. Roots/manifest/database must be owned regular paths without symlinks.

Hard bounds: 1,024 sources; 64 predecessors/source; one pending transition/source;
16,384 commands and 16 MiB command bodies. Reaching a bound refuses changes,
never silently drops obligations. Metadata admission, checkpoint compaction,
complete source coverage, root closure recovery and native consumer cutover stay
OPEN. Explicit create/open only; fixtures never attach a live cache or Git hook.

## Read-only observation

`SourceJournal::observe(root, trusted_minimum, source_key)` returns copied source
state and checkpoint from the same replay using SQLite read-only access. It
exposes no writable handle and performs no creation, repair or transition. The
existing writer API and persistent wire format are unchanged. Busy remains
nonblocking. Observation can be stale immediately; all mutation fences remain.
The [explicit inspector](../docs/guides/SOURCE-OBSERVATION.md) writes bounded
private reports and does not infer liveness, root availability, global absence,
recovery permission or payload custody. Absent source state is scoped evidence.

## Executed scope

`SourceJournal::preview` uses read-only verified replay, shared exact-request
receipt lookup, shared growth admission and the same pure transition checks as
apply. It returns an observed checkpoint plus either a recorded historical
receipt or proposed before/after source state. Proposed state grants no lease or
mutation acknowledgment. The [explicit source transition CLI](../docs/guides/SOURCE-TRANSITIONS.md)
plans and applies only existing command kinds, saving an exact private request
before mutation and retaining owner/revision/operation fences. No journal wire
format changes, automatic source adoption or payload operations are introduced.

`SourceJournal::acknowledge(root, trusted_minimum, operation, original_command)`
recovers only an exact previously committed receipt through read-only access.
It reuses the canonical body comparison and verified event-chain receipt path.
Unknown IDs return Missing instead of applying a new command; changed requests
return Conflict. The receipt checkpoint/revision remain historical after later
transitions. This is acknowledgment recovery, not current-state observation,
producer replay or interrupted handoff reconciliation. The explicit inspector
guide documents its bounded private `tig-source acknowledge` CLI profile.

EVID-026: 19 entries including the subprocess helper, six actual exits during
prepare/adopt and 15 injected boundaries across five command variants. Real
fixtures fill 1,024 source slots and 64 predecessor slots, and replay a valid
16,384-command database before verifying growth refusal and historical retry.
These are command/source bounds, not a 500,000-object metadata-scale proof.

EVID-027: two real Git/context fixtures retain root metadata outside the repo,
replace one cel while reusing another cel/shared cue, restart a prepared handoff,
adopt main, retire its predecessor and preserve another branch's old inputs.
No media files are moved/deleted. Selection root sizes are bounded to 1..65,536
bytes; references themselves never prove tree validity/closure. Even an empty
ObligationSet explicitly returns complete_claim_coverage=false. The set includes
its exact journal checkpoint; it is not a live retirement fence.

The [batch observation extension](SOURCE-BATCH-OBSERVATION.md) reads 1..64 explicit
distinct sources at one verified replay checkpoint. It shares the existing
read-only authority path; no source discovery, mutation, liveness or complete
claim-coverage assertion is added.

Earlier total: 299 Windows / 300 Linux entries. Full requirements/VAL-001/004 and
native source/job coverage, metadata admission/compaction, storage providers,
backup-recovery authority and actual removal remain OPEN.
