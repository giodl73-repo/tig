# Explicit local record contracts

Entryd1f8712da01e908fb1462f841a65e4bcb679d028; parent PR34/35 must qualify.
WP-010/011; REQ-001/006/009/010; SPEC-010/011; IF-010/011; PKG-004/006/008.
Allowed: optional schema-layer local contracts, native borrowing verification,
explicit CLI mode, synthetic tests and bounded docs. No new dependency, provider,
renderer, media IO, hook, source mutation, approval or custody operation.
RUST/GIT/INTEGRATION L0: structure/fmt/Clippy/standalone isolation; L1: roles,
groups, cardinality/order, cross-field conditions, pins, strict declarations,
legacy refusal, no defaults/mutation/private diagnostics; L2: mixed merges,
temporary native Git/archive and actual media/nonmedia CLI use.

Do not silently interpret existing stream_rules or semantic_rules maps as JSON
Schema. This explicitly admitted wire declaration is required in EACH map:

```json
{
  "schema": "tig.local-json-schema-rules.experimental-1",
  "dialect": "2020-12",
  "assertion": {"type": "array", "minItems": 1}
}
```

Unknown/missing declaration fields or tags refuse. Empty/legacy maps mean
unspecified, not successful validation. To explicitly declare no local assertions,
use assertion:{} (or true in dialects that support Boolean schemas) under the
same versioned declaration. Each declaration's
dialect must equal caller admission; its assertion's $schema must agree as well.
Use the existing offline pinned interpreter and its unchanged limits/privacy
policy; never add an alternate expression language or executable rule hook.

stream_rules evaluates the parsed ordered streams array, so standard JSON Schema
can assert roles/groups, slot positions, unique values, cardinality and required
classes. semantic_rules evaluates the complete parsed native instance, allowing
authored/selected/streams/status/metadata conditions. They are LOCAL assertions;
they cannot prove referenced asset type, primary-input closure, actual producer
behavior, review authority, delivery eligibility, storage capacity or custody.
Graph/native adapters remain responsible for those independently admitted facts.
Do not add hardcoded media roles, status vocabulary, approval or golden meanings.

First execute the pinned authored schema, then both declared local assertions.
Successful private checked values bind exact record/definition/authored-schema and
local-interpreter policy pins; definitions already pin both inline assertions.
Policy describes projections/wire/engine semantics, not an editable dependency
list. No defaults are applied; no raw value/error escapes diagnostic categories.
complete_semantic_coverage remains false. A valid mixed-stream merge need not use
one producer attempt unless its declared local contract explicitly requires that.

Native validate_local_contracts(dialect) checks all captured current/retained
records and reuses compilation by definition. The immutable borrowing resolver
reports local_contracts_validated:true only after the complete scoped check;
the existing authored-only path reports false. Mutation/expansion needs recheck.
Explicit CLI: tig-record validate-contract INSTANCE DEFINITION SCHEMA DIALECT.
Without schema feature it refuses before IO; bind/inspect/validate stay unchanged.

Producer pre-code six-role review: identity requires exact definition and declared
wire/dialect; Git/adoption keeps legacy evidence readable and does not fabricate
contracts; privacy keeps assertions/values private; concurrency/recovery uses
immutable scoped checks without new locks; placement/custody forbids approval or
eviction authority. General engine hard CPU/memory isolation remains an operational
gap inherited from NATIVE-SCHEMA-VALIDATION.md. Full native acceptance stays open.
