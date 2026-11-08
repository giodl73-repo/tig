# Preserve acknowledged bytes by default

This is Tig's governing design principle and acceptance obligation, not a claim
that production storage safety is already implemented. The pure core performs
no IO; a bounded local publication adapter is tested in explicit temporary roots.
An opt-in [managed local profile](../specs/MANAGED-REPLICA-RETIREMENT.md) adds
bounded ordinary replica eviction/cancellation and explicit reacquisition in
synthetic fixtures. Production adoption and full-system invariant TIG-I-01 remain OPEN.

**Tig must not silently overwrite or make acknowledged managed content
unrecoverable through its own publication, replacement, movement, cleanup or
recovery operations.** Retiring content requires explicit admitted retention/
deletion policy and current authority, not an inferred lack of references.

## What users should be able to rely on

| Operation | Required behavior |
| --- | --- |
| Create | Protect admitted ingest; acknowledge only the actual persistence level achieved |
| Replace V1 with V2 | Publish new immutable bytes; preserve V1 under its retention obligations |
| Move or demote | Verify destination obligations before releasing protected source copies |
| Read | Coordinate active use with conflicting eviction; no timeout-only loss of a valid reader |
| Crash or lose a reply | Recover durable intent/protection; partial data and uncertain completion are never silently treated as success |
| Lose metadata | Block destructive work until protection scope is adequately recovered |
| Reclaim cache | Remove only eligible replicas after fresh all-consumer and independent custody checks |
| Delete retained content | Require explicit admitted final-deletion authority/policy and disclose the affected exact objects |

Automatic cache eviction can run under an explicitly enabled bounded policy;
this does not imply permission for final archival deletion. A Git reset, branch
deletion, failed job, lease expiry or unregistered object never grants deletion
authority by itself. Cleanup of dirty private work needs its own explicit scope.

## Say what is durable

```text
written privately -> sealed identity -> durable local copy -> verified redundancy
                         |                    |                       |
                 bytes identified      provider durability     independent retained
                 is not persistence    is not replication      copies/keys verified
```

An API result must distinguish those levels and any pending obligations. A
producer finishing or a Git push succeeding does not establish independent
backup. Real durability requires provider-appropriate flush/commit behavior,
integrity, retention and recoverability; a catalog row alone is not proof.

Tig cannot promise survival of every device/site/key loss. It must state its
failure-domain assumptions and redundancy policy, verify what it can, and report
unsatisfied protection honestly. That limit must never be used to justify a
Tig-controlled last-copy loss or a misleading durability acknowledgement.

## Proof required before live removal

Use fault-injected provider tests: interrupted writes, copy corruption, lost
acknowledgements, stale backup restores, reader/removal races, failed retention
targets and reordered lifecycle events. Verify byte recoverability and measured
physical effects, not only successful return codes or planned set membership.
Every permitted final deletion must retain an auditable exact scope/authority
record without leaking private payloads. See [mitigations](scenarios/MITIGATIONS.md),
[invariants](doctrine/INVARIANTS.md) and [verification](vtrace/VERIFICATION.md).
