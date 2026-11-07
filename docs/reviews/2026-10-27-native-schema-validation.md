# Authored-schema engine producer review

Entry1de926c3531ba19a5e49beec874553069f215bd0; depends on qualified PR33/34.
WP/REQ/SPEC/IF/PKG, allowed scope, L0/L1/L2 and exit requirements are recorded
before code in specs/NATIVE-SCHEMA-VALIDATION.md. This is optional authored JSON
Schema execution, not full native semantics, runtime adoption or final acceptance.

Dependency intake: exact jsonschema0.60.2, MIT, default features disabled; matching
registry source and official docs examined. Explicit offline() prevents retrieval
even if a downstream consumer unifies other engine features. No new HTTP/file
provider, media cache, hook or production Python. Cargo.lock records dependencies;
the standalone Git/LFS graph must remain free of the optional schema engine.
Primary documentation: https://docs.rs/jsonschema/0.60.2/jsonschema/ .

Producer-applied six-role post-code findings, owner Tig producer:

- Identity/namespace: compile under exact definition/schema/dialect and check only
  the same definition. Root dialect mismatch refuses; policy bytes bind engine,
  format and regex settings. Private checked types have no deserializing constructor.
- Git/adoption: opt-in validate/borrowing resolver; existing bind/inspect semantics,
  statuses, source bytes and defaults remain unchanged. Rechecking includes retained
  records; immutable borrowing prevents expansion under an earlier check.
- Concurrency/recovery: compiled schemas shared by exact definition; time budget
  checked between records and after evaluation. No mutable global inventory/job
  state or new lock. General hard CPU/memory isolation remains an operational gap.
- Provider/privacy: disable default retrieval features and use offline independently;
  external refs refuse. Upstream error strings may contain private values, so only
  categories escape. CLI/Debug omit authored/schema text and private metadata.
- Placement: JSON Schema success is not stream/native semantics or actual producer
  consumption and cannot become implicit approval, current-delivery or retention.
- Storage/custody: metadata-only checks, no provider/media reads or writes except
  caller-explicit CLI files. Tests use synthetic temporary Git/archive providers;
  no cache initialization, enrollment, capacity admission or deletion is inferred.

Implementation findings: the first compile used a nonexistent upstream error-kind
helper; inspected the pinned API and replaced it with explicit category matching.
A targeted regex-under-negation case passes: unknown bounded matching must not
become success. Initial six pure and three native cases pass, including all five
explicit dialects and1,001 records sharing two compiled definitions. Additional
full-suite cases cover actual CLI behavior, default-feature refusal, explicit
external-resource nonretrieval and schema-validated resolver/freshness composition.

Disposition: component pass_with_risk pending final local/hosted qualification.
No independent/human review or full intended-use acceptance is claimed. Medium
residuals (owner Tig producer): operational hard CPU/memory isolation, custom
stream/native semantics, full variants/consumption and actual private adoption
remain open. Unknown annotation keywords retain standard JSON Schema behavior;
no project extension is claimed executed merely because the schema passes.

Final local verification: full workspace all-feature suite passes513 entries;
default-feature tig-assets passes99 entries (including early no-engine refusal).
Clippy then requests one equivalent nested-if collapse in the dialect guard;
fixed without changing validation behavior. Final affected package runs pass106
asset entries and55 native-source entries; workspace all-feature/all-target
warnings-denied Clippy, standalone Git/LFS check, absence of jsonschema from that
active dependency graph, and format pass. These targeted final runs are not a
second full workspace run. Exact-head hosted full Windows/Linux qualification
and parent33/34 qualification remain required before merge.

Dependency/structure audit:60 new locked packages; no existing package removed
or replaced; no reqwest/rustls client in the locked graph. New engine dependencies
are optional for standalone low-level consumers. The source remains under2MiB.
No source/runtime binding, approval, production cache or renderer changed.

Final review refinement: enforce the64KiB schema length before hashing supplied
bytes, not merely when the strict parser runs. The existing pin/privacy case now
asserts this early refusal. After this change, all106 asset and55 native-source
entries, workspace warnings-denied Clippy and fmt pass again. No broader rerun is
claimed; exact committed-head full hosted qualification remains mandatory.
