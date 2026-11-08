# Manifest association to semantic impact

Input 954af659c8250067c73f3f0304924db5c0f33020; WP-008/010/015;
REQ-001/005/006/007/010; SPEC-005/010/014; IF-005/008/009/010/013;
PKG-002/004/006/008. RUST/INTEGRATION/PROVIDER reader profiles.

Entry: bounded explicit manifest references and admitted native graph queries.
Scope: manifest_impact composes these existing engines; one captured project and
one work scope. Refactor reverse impact to accept several exact selected roots
without resolving the native graph once per association. Preserve single-target
impact and preview fingerprints. No new semantic extraction rules or cache IO.

The association hop is evidence, not a production edge. Report potential consumers
conditional on a manifest-bearing record changing, including retained bindings;
do not infer actual production use, stale renders or retirement. Missing manifest
or graph coverage stays partial. Failed roots contribute no associations. Use
source and extraction-content fences before paging; graph limits stay unchanged.
All matching roots enter traversal before consumer pagination. Full association
provenance remains available via manifest_references with the same content token.

Exit/L0: source/schema/docs/hash checks and unchanged generic/privacy boundaries.
L1: focused agent tests, full locked all-feature workspace, strict Clippy and
standalone check. L2: direct/transitive/shared consumers, mixed edge kinds,
frozen old pins, missing graph/manifest coverage, several roots, over-100
associations, content/source fences, paging and preserved single-target behavior.
Producer lenses: identity, privacy/provider, Git/adoption and concurrency. This
does not close full project/job/custody coverage or independent acceptance.

Producer review: four named lenses applied to query/manifest_impact, shared
reverse traversal, manifest matching and request/result schemas. Identity keeps
exact selected owner pins distinct from missing frozen revisions; associations
cannot substitute another record. Privacy/provider reuses bounded named readers
and omits raw document fields/physical paths. Git/adoption adds no project rules,
registry, scanner or persisted index. Concurrency uses the existing source fence
before reads and content fence before paged results; all associations seed the
graph before pagination. No blocking finding within this scope; conditional
meaning and unadmitted coverage remain explicit limits. Owner: query maintainer.

CR-004 justification: the existing bounded impact resolver retains reverse
traversal, deterministic witness framing and coverage assembly together, now
parameterized by exact roots. Splitting its framing across a second engine would
increase semantic drift. Shared-consumer, depth/cycle, frozen-pin and preview
fixtures cover this retained function; new composition stays a small wrapper.

Verification: 39 focused agent scenarios pass, including seven new integration
cases: association/native chaining, independent coverage gaps, missing frozen
pins, 111 associations over two roots/shared consumers, 122-consumer paging and
stale fences, retained binding semantics and strict request admission. The
initial frozen-pin fixture incorrectly expected complete coverage; inspection
confirmed the admitted snapshot correctly reports the old revision Missing.
The fixture now requires that exact gap while preserving known current consumers.
Full locked all-feature workspace tests, strict all-target Clippy, standalone
no-default-feature check, explicit CLI build/capability smoke and portable
source/diff checks pass. Hosted Windows/Linux qualification is separate.
Validation is synthetic; no live cache/enrollment or browser is touched. Trace
closes this bounded composition only. Producer review is not independent review
or human acceptance; hosted cross-platform qualification remains required.
