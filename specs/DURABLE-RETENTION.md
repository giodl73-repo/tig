# Budgeted local retention commitments

Input 3970f2b2b4c30a91040a36eff8f22937f5a07b3a. Bounded WP-003/004/006/007/008:
REQ-002/003/004/005/007/009; SPEC-003/004/005/008/009; ARCH-002/003/007;
IF-003/004/005/006/012; DES-003/004/005/008; PKG-002/003; PROVIDER profile.
Entry: verified completed replicas, scoped OS guards and durable accounting.
Exit: admit metadata budget, verify the copy, install immutable owner-scoped
retention, measure/settle its footprint, reopen and enforce protection.

```text
reserve metadata peak -> durable bound retention allocation
    -> exact attempt fence -> verify completed target bytes (shared object guard)
    -> bounded per-object retention metadata fence
    -> immutable retention record -> measured metadata footprint -> settlement
                                                   |
process restart -> verify exact grant + ledger + bytes
                                                   |
exclusive object request -> check retained records -> refuse protected object
```

Local manifest design-4 adds a required retentions root. Older stores reject
without migration. Each object permits at most 16 immutable owner/operation
records; normal checks inspect only that exact object. No retention expiry,
automatic release, root scan, final-object removal or live cache enrollment.
Releasing a retention commitment will require a separate owned safe-handoff
protocol; ordinary session exit never removes it.

The installer holds the object's shared guard through verification, record
publication and settlement. Exclusive acquisition holds its guard while checking
retention. New retention cannot slip past a granted exclusive guard. Malformed
or unknown records fail closed. Losing or restoring provider metadata is not
proof of no claims: trusted root/ledger recovery authority is still required
before future destructive operations. An exclusive guard alone never authorizes
eviction. Raw filesystem actors and hostile namespace mutation remain unsupported.

The metadata write is admitted before provider mutation. The charged basis stays
conservative allocated-file-data footprint: binding, private partials and immutable
retention record, never the already-present target payload. Directory/namespace
and ledger overhead remain outside that basis under the existing headroom policy.
No DB transaction spans byte verification or file publication. Failed/uncertain
attempts retain in-flight capacity. Exact retry recovers a complete record;
unresolved private partials do not spawn unbounded staging copies.

A portable grant identifies the exact record, owner, store, publication and ledger
allocation. It is not self-authenticating or removal authority. Fresh verification
must match actual record, settled ledger binding and actual bytes. Retention now
persists across normal process restart; native failure-domain independence,
power-loss durability, stale-backup authority, complete claims, release/handoff
and final-source removal remain open. No legacy consumer runtime change is admitted here.

L0 structure/fmt/VTRACE/source hashes; L1 locked workspace/all-target Clippy;
L2 actual temporary retention/reopen, object races and process interruption.
Full requirements, package exits and VAL-002/003/004 remain OPEN.

## Implemented evidence

EVID-019: 21 new entries including a subprocess helper; workspace total 230.
Commands: cargo test -p tig-store --lib --locked retention::tests; locked workspace,
fmt and warnings-denied all-target Clippy. Temporary fixtures exercise exact retry,
reopen, metadata-only accounting with a larger target payload, owner/budget/type
mismatch, damaged/missing records, pending partials, per-object cap and concurrent
writers racing the last slot. Three actual child exits cover bound, installed and
settled phases. Installed-but-unsettled claims already block exclusive access.

A grant is checked against actual immutable record, its typed settled allocation
and current bytes before it becomes a RetainedReplica. Declared-domain coverage
can now establish this local retention profile, while eviction authority stays
false. A failed refresh invalidates the live retained view but never releases
its object guard or durable claim. Missing settled records are integrity incidents,
not permission to recreate data under a spent allocation.

[Owned handoff](RETENTION-HANDOFF.md) adds explicit local release to a verified
protected successor. Design-5 stores retain immutable handoff markers and retire
only the old active claim file. Session exit/expiry still never releases a claim;
full ownership/recovery authority and final-object eviction remain open.
