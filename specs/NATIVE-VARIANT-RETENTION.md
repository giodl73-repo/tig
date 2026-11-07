# Retained native variant interpretation

Entry1095b6f (merged sparse variants). WP-010/011; REQ-001/006/008/009/010;
SPEC-005/010/011; IF-004/010/011; PKG-004/006. Scope: bounded metadata-only packet,
optional job linkage, synthetic Git/store/replay tests and controlled docs.
No cache initialization, media IO, daemon, rendering, deletion or runtime adoption.
L0: structure/fmt/Clippy/standalone; L1: strict pin, policy, coverage, privacy,
bounds and foreign-context refusals; L2: native Git plus persistent metadata/job
reopen with source unavailable. Exit requires exact-source cross-platform checks.

Packet policies are independently supplied per exact definition: original asset
space, current project space, semantics and consumed axes. Requests pin exact
records, indexes and context documents. Resolve the production graph again under
the context's admitted profiles/checkpoint/roots. Every primary node covered by
a supplied variant policy requires exactly one request. Reject extra requests,
non-primary owners, duplicate definitions/records and graphs whose selected primary
objects differ from the resolved variant. Uncovered definitions retain the graph's
existing interpretation; this is not complete native semantic coverage.

Read bounded exact space/index/context documents through NodeStore. Re-admit full
context and resolve against the exact source record (including retained frozen
records at the current checkpoint). Generate one bounded witness per record and
one policy document per definition; a linked Tree retains those witnesses, all
deduplicated interpretation controls and the existing production-context root.
No duplicate authored payload inventory or Cartesian matrix. Preflight all pins,
limits and conflicting sizes before writes. Candidate finish/readback and explicit
owner journal adoption remain external.

Restore recomputes every selection under external policies/requests and exact
context, reads verified retained documents and compares the entire packet tree
and witnesses. Stored policy bytes cannot grant admission. An optional version-2
job envelope links this sealed packet and adds its references to Interpretation,
never Primary. Version-1 job calls/bytes stay unchanged. A packet from another
production context is refused. Full unused context changes provenance, while a
producer using projected parameters may retain its recipe identity.

Producer checklist before implementation: identity requires exact replay and
size consistency; privacy forbids raw parameters in Debug/errors; Git keeps
growing witness data asset-side; concurrency uses immutable roots without global
locks; custody makes no durability/availability/removal claim; placement reports
declared references separately from disk allocation. Dynamic per-owner graph
projection and produced-claim variant replay remain separate open obligations.

```text
Git checkpoint + admitted graph profiles
  -> production context root (source archive, policy, declared inputs)
       ^
       | source/context
  variant packet root
       +-- policy/<definition hash> -> independent policy witness
       +-- witness/<record hash>    -> exact choice / projected subject
       +-- control/<hash>           -> index, spaces, full context
       ^
       | source/variants
  job version2 root -> production context + recipe/runtime references
       ^
       | explicit finish / reopen / replay / owner adoption
  existing source journal (an enrolled obligation, not global custody)
```

Packet restore receives the original external policies and request pins; it does
not infer current policy from the stored witness. Callers may inspect the linked
immutable data separately, but a stored declaration is never its own authority.
Read and write budgets are distinct; restore accepts zero writes. Packet control
reads, regenerated witness reads and tree traversal share the configured packet
read budget. Generated documents and tree writes share its write budget. Source
graph resolution uses the separate explicit graph limits. Tree-write failures
may leave bounded unadopted candidate metadata; no success/adoption is implied.
