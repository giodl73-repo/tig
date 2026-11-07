# Architecture baseline

Target component IDs control packages; they are not claims that crates exist.
The implemented tig-core, tig-store, tig-assets and tig-git workspace is mapped
in [current architecture](../ARCHITECTURE.md). EVIDENCE.md and
PACKAGE_BOUNDARIES.md distinguish their proof from the full target architecture.

```text
Git / native producer / optional typed records
             |                    |
       source snapshot      semantic resolution
             +---------+----------+
                       v
              exact objects and claims
                       |
        catalog <-> durable engine/journal <-> byte worktree
                       |
                 scoped plans/fences
                       |
                admitted provider ports
                       |
             local / remote / optional transport
```

| ID | Responsibility | Parent specs | Key risk / disposition |
| --- | --- | --- | --- |
| ARCH-001 | Pure identity, trees, assigned claims, plans and coverage types | SPEC-001/002/008 | Do not treat pure assertions as provider truth |
| ARCH-002 | Durable admission, operations, recovery and active-use/removal fencing | SPEC-003/004/009 | Lost protection or ineffective fencing blocks removal |
| ARCH-003 | Derived catalog, observation cache and immutable-node indexes | SPEC-005 | Rebuild does not erase authoritative durable protection |
| ARCH-004 | Private byte views, seal/adoption and exact Git/LFS source adapter | SPEC-006/007 | No silent Git commit, pointer dialect or dirty-file overwrite |
| ARCH-005 | Optional typed assets, native adapters, fingerprints and profiles | SPEC-010/011 | No duplicate semantic inventories or media-specific core |
| ARCH-006 | Review/issue/delivery reconciliation and trusted transition policy | SPEC-012 | Private evidence and human authority remain separate |
| ARCH-007 | Capability-declared store/transport ports and implementations | SPEC-009/013 | No universal deletion/fencing assumption across providers |
| ARCH-008 | CLI/read-only inspector and optional browser/WASM view | SPEC-014 | No private payload/credential artifacts or implicit daemon |

Portable authored records and policy stay small in Git. Immutable objects and
large indexes/reports live in admitted storage. Durable claims/journals and local
place credentials live outside Git; derived indexes are rebuildable. Explicit
configuration binds place identity/capabilities/failure domains to locations.
No branch name or drive letter is an immutable object/claim identity.

No global transaction spans Git, local files and remote hosts. The engine uses
scoped durable intent, safe acknowledgement ordering and idempotent recovery;
destructive work additionally needs effective provider-specific revalidation.
External pruning is not automatically coordinated by the LFS transfer agent.

See [unified design](../UNIFIED-DESIGN.md), [theory](../THEORY.md),
[byte worktrees](../../specs/BYTE-WORKTREE.md) and package/interface stages.

## Stage review

Producer-applied six-role lenses: pass_with_risk. A-01: catalog and authoritative
protection could be conflated in one database abstraction. Resolved by distinct
owners/recovery semantics even if a backend co-locates rows. Choice and crash
proof of a concrete backend remain discovery/package outputs, not assumed here.
