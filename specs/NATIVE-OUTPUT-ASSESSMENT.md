# Native output assessment

Entry36286fb12a815e19d51253c9260e2ce4f551967e (PR33 must qualify first).
WP-010/011/012; REQ-001/006/009/010; SPEC-010/011/012; IF-010/011;
PKG-004/006. Allowed: exact resolver metadata access, optional native read-only
assessment adapter, synthetic tests and bounded docs. No new provider, renderer,
job enrollment, executable hook, media IO, approval or custody operation.
RUST/INTEGRATION L0: structure/fmt/Clippy/diff. L1: missing versus malformed
evidence, exact checkpoint/definition/selector, bounded pins/paths and redaction.
L2: temporary real Git revisions and retained metadata replay, no provider writes;
logical source edits stale outputs, annotation edits retain recipe matches.

The caller supplies an already computed desired Plan and an exact output key,
admitted output definition, explicit metadata claim location and separately
admitted original producer/context rules. Resolve the current record once at the
desired checkpoint. A missing record is blocked, not planned. A present record
with no selected/named bytes can be Unbuilt only when evidence is also absent.

The resolver exposes immutable metadata from the same bound record as authored
and status fields; Debug and CLI continue omitting those values. A claim location
has 1..8 literal object-key segments of 1..128 bytes. It is neither a filesystem
path nor an implicit project field. Missing first segment means no claim. Once
the first segment exists, missing/non-object intermediates or null/invalid pins
are blocked. This supports both a top-level pin and REEL's production/claim
metadata without hardcoding project vocabulary or duplicating dependency lists.

Read only the exact externally pinned claim (1..65,536 bytes) through the existing
metadata NodeStore. Missing/provider-error/corrupt bytes remain blocked. Parse the
claim, replay its original context under separate admission, then use the existing
scoped assessment with the current subject and desired recipe. Never trust a
serialized claim subject as current source. Return typed bounded issue categories,
current record and claim pins; Debug reveals only state/presence/category.

Old producer admission remains separate from desired producer choice: a permitted
old claim can be stale under a new desired producer. Failure to admit/replay old
semantics is blocked, not proof that rerendering or replacing approval is correct.
Claims remain unsigned declared correspondence, not execution/integrity/custody
proof. No complete project discovery, native consumption/variant completeness,
delivery eligibility, source/job retirement or reversible cutover is closed.

Producer pre-code role review: identity requires exact current checkpoint and
definition before evidence IO; privacy forbids raw metadata in diagnostics;
Git/adoption preserves one record as authority; recovery forbids fallback after
missing/corrupt evidence; placement/custody forbids treating triage as eviction.
Native fixture exit and exact hosted qualification remain required.
