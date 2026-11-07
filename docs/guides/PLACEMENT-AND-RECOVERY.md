# Placement, demotion and recovery

Target walkthrough. Tig currently computes pure plans; no file operation below
is executed by the prototype.

## Safe demotion is copy, verify, then eligible release

```text
desired: object K leaves working, remains required in history + archive

working       history       archive
   K             ?             ?       inspect admitted observations
   K             K             ?       copy/verify history if needed
   K             K             K       ensure required archive custody
   |             |             |
   +---- fresh protection/authority/custody recheck
   |
eligible release at working             only if all conditions still hold
```

An existing verified destination copy can eliminate a transfer. Multiple drive
letters do not prove multiple independent disks. Retained-copy policy considers
failure domains, freshness and actual retention obligations. Unreachable places
remain unknown unless a declared evidence policy admits appropriate observations.

## A dry run must expose costs and blockers

Illustrative report shape, not current CLI output:

```text
source checkpoint: exact Git/tree + claim/policy generations
ensure:             5 objects at history
reuse:              995 working objects unchanged
held:               1 old object still used by another job
release candidates: 4 working copies, subject to final checks
space:              transfer bytes / peak scratch / possible physical reclaim
blockers:           missing coverage, offline places or unsupported capabilities
```

Net savings do not pay the peak scratch cost in advance. A plan must fit admitted
resources or use safe bounded batches. Metadata, shared packs, duplicate LFS stores
and editable materializations can make logical bytes differ from allocated bytes.
Only a completed provider operation and measurements establish reclaimed space.

## Crash and concurrent work

```text
persist intent -> reserve -> transfer -> verify -> publish observation
      |              |          |           |              |
      +--------------+----------+-----------+--------------+
                     durable recovery checkpoints
                                   |
                     refresh claims / custody / policy
                                   |
                     fenced release -> measure result
```

A crash before completion leaves partial work, not a verified replica. Recovery
reconciles durable intent and physical evidence, without discarding useful complete
copies. Catalog loss cannot erase active-job protection; loss of authoritative
protection history blocks destructive work until adequate recovery.

Another session can publish a new object or start using an old one while a transfer
runs. Tig must fence conflicting removals at the relevant scope. No global cache
lock should pause all rendering. Transfers must not hold catalog transactions for
their duration. An earlier passing merge gate cannot replace the final safety check.

## Eviction is different from deletion of history

```text
evict working copy:       remove an eligible replica, preserve retained content
delete archived content: change retention obligations and authorize final removal
```

Final archival deletion is a separate explicit policy action. An unregistered
object has unknown obligations; it needs an admitted classification/retention plan,
not immediate garbage collection. Current pure tests conservatively hold it.

See [safety contract](../../specs/SAFETY.md), [handoff prototype](../../specs/HANDOFF-PROTOTYPE.md)
and [evidence limits](../vtrace/VERIFICATION.md) before treating a plan as executable.
