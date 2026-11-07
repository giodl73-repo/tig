# Explicit authored-schema execution

Entry1de926c3531ba19a5e49beec874553069f215bd0; depends on qualified PR33/34.
WP-010/011; REQ-001/006/009/010; SPEC-010/011; IF-010/011; PKG-004/006/008.
Allowed: optional Rust schema engine/feature, exact native validation wrapper,
explicit tig-record validate command, synthetic tests, locked dependencies/docs.
No change to existing bind/inspect semantics, defaults, schema bytes, source
selection, runtime installation, cache IO, renderer, hook or deletion authority.
RUST/GIT/INTEGRATION L0: structure/fmt/Clippy/diff/standalone-feature isolation;
L1: all supported explicit dialects, nested/composed schema constraints, pins,
false/invalid schemas, formats, malformed/remote refs, no mutation and redaction;
L2: actual CLI and native Git/archive validation; no source/cache writes.

Use jsonschema0.60.2 (MIT), exact Cargo pin, default features disabled and offline
options. Primary source: https://docs.rs/jsonschema/0.60.2/jsonschema/ and the
downloaded matching crate source. No HTTP/file retriever, implicit external schema
lookup or project-specific validator. Self-contained compound/internal references
are permitted; missing resources block compilation. Explicit dialect: Draft4,
Draft6, Draft7, Draft201909 or Draft202012. Root $schema, if present, must match
the admitted dialect's canonical URI (optional trailing #); no automatic fallback.
Assert known formats and reject unsupported ones. JSON Schema annotations and
unknown annotation keywords retain standard engine semantics; this is not an
assertion that arbitrary project keywords have executed.

Compile exact pinned authored schema under its exact definition; check only
instances bound to that same definition. Preserve Tig's strict64KiB/4,096-node/
depth32 integer-only JSON intake. Do not materialize defaults or rewrite source.
Regex limits:100,000 backtracks,1MiB compiled pattern,1MiB DFA cache; engine failure
is evaluation failure, never valid. These limits do NOT provide a general hard
CPU/wall-time/memory sandbox for arbitrary JSON Schema; trusted admitted schemas
and separate operational process/capacity policy are still required.

Successful checks bind exact instance, definition, schema and engine-policy pins.
No deserialization constructs a checked capability. Private errors expose category
only, never upstream validation messages containing instance values or schema
text. Debug/CLI omit authored values. Engine policy bytes are available for caller
retention and do not claim producer/runtime custody.

Optional tig-git schema feature checks every captured current/retained record,
reuses compiled schemas by exact definition and returns a borrowing immutable
AssetResolver wrapper only after all succeed. Expanding/mutating the snapshot
requires a new check. No flat proof inventory is added to Git. Original metadata
resolution remains available separately for evidence-only legacy intake.

Explicit CLI: tig-record validate INSTANCE DEFINITION SCHEMA DIALECT, built with
schema feature. Without that feature, refuse before file IO. An authored-schema
check is not stream_rules/semantic_rules/reference closure, native producer truth,
eligibility/approval, byte integrity, custody or complete_semantic_coverage.

Producer pre-code six-role review: identity requires exact definition/schema
pins and dialect agreement; Git/adoption requires opt-in verification without
source mutation; privacy requires category-only errors; recovery requires remote
and unresolved resources fail closed; placement/custody forbids safety authority.
Dependency intake increases Cargo.lock/source bytes within the existing2MiB
budget; low-level Git/LFS consumers must not build the optional engine.
