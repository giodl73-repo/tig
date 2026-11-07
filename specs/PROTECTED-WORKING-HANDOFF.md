# Acknowledged working-claim handoff

Input bd572083489c4be0da81736cb6aaf12960a0e5cc (PR79 candidate).
WP-006/007/008/009/010/012; REQ-002/003/004/005/006/007/009;
SPEC-003/004/005/006/007/008/009/010/012;
IF-002/003/004/005/006/010/012; PKG-001/002/003/004/006.
RUST/INTEGRATION/PROVIDER profiles. Scope: exact owner handoff over existing
SourceJournal commands and immutable demand roots, compiled protection-provider
port, existing LocalStore retention verification adapter and synthetic fixtures.
No new journal/format/cache engine, implicit copy, payload eviction or live use.

Entry: classified roots and role projection candidates. Exit: old working root
is withdrawn only after successor protection acknowledges all new assignments
and old retained assignments. Changes/retries remain bound to one exact request;
crashes do not lose old working protection or leave completed temporary roots
hot. L0 validator/fmt/diff/roles/trace; L1 locked all-feature tests/Clippy and
standalone compatibility; L2 actual retained-byte verification, journal reopen,
phase failures, stale/refusal and unrelated-owner progression in temporary roots.

Request binds the admitted journal genesis, operation ID, required AckLevel and exact old/successor source,
owner, revision and Selection. Sources must differ; both current roots must match.
Prepare a metadata-only intent Demand from the successor, with claim ID equal to
a domain-separated canonical request hash. Finish and independently read back its
metadata through the admitted provider before executing. Changed operation reuse
therefore changes the pending selection and conflicts in the existing journal.
The host must retain the exact Request and intent Selection in its existing
private job/operation record before executing any journal transition. The library
does not reconstruct an operation from a hash alone or discover lost host jobs.

```text
prepare exact intent on old source (old current remains working)
  -> verify permanent successor assignments + all old retained assignments
  -> revalidate held provider guards and successor source stamp
  -> abort exact pending intent (old current still working)
  -> withdraw exact old current (predecessor remains retained-only)
```

Phase operation keys derive only from caller operation namespace and phase.
Existing journal replay binds complete command bodies. Intent prepare, exact
abort and withdraw use sequential source revisions; no SQLite transaction spans
provider IO. Another source can progress. Changed owner/root/target, pending
foreign work, stale revisions, missing grants/bytes, weak acknowledgments and
budget failures refuse. A failed guard verification keeps old current/pending
claims. Failure after abort keeps old current. Exact retry completes without
repeating a state transition. Historical completed acknowledgment is explicitly
not fresh protection; it returns no live provider guard or custody claim.
Archive reads share one IO budget. Held guards have an explicit admission ceiling
(default 4,096, maximum 65,536); an insufficient budget refuses before any journal
transition or protection acquisition. Raising a limit does not alter the request.

The compiled provider port must verify durable retention and return held guards;
it may not substitute path existence or a publication receipt. The provided
LocalStore adapter reuses settled RetentionGrant verification/revalidation and
acknowledgment strength checks. It enrolls existing places, ledgers and grants;
the requested place ID must equal the native store's enrolled place ID, so a
working store cannot be relabelled as a separate retained destination.
it creates no caches or claims. Other engines must provide equally admitted
adapters; LocalStore limits do not expand for media migration.

Global coverage, native independence, all-cache authority and eviction remain
false/unproved. Withdrawal changes desired working ownership, not physical bytes
or old store grants. Existing scoped custody/handoff/executor machinery still
must retire surplus store protections and verify/measurably reclaim/restore bytes.

Existing source-journal command/source/predecessor bounds are unchanged. This
protocol does not reserve multi-command journal growth against peer writers or
provide service-lifetime continuation. A later capacity refusal preserves current
or pending protection and requires explicit recovery; sustained-service admission/
continuation remains a separate gate before bulk deployment (KRY-02).
