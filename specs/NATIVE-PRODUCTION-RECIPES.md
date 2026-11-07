# Declared consumed-input recipe fingerprints

The original plan contract below remains unchanged. The separately admitted
[consumed payload projection](CONSUMED-PAYLOAD-PROJECTION.md) is an opt-in mode
that can retain captured controls without consuming their raw bytes. It requires
explicit payload policy and uses different identities; legacy plans do not infer it.

Entry e504cf327528ba9b1573b6f3efeff64e0784d77e. WP-010/011;
REQ-001/005/006/010; SPEC-010/011; ARCH-005; IF-010/011; DES-009;
PKG-001/006. RUST/INTEGRATION: L0 fmt/Clippy/portable/canonical VTRACE;
L1 pure native vectors, unknown/missing/substituted/budget cases; L2 synthetic
non-media and scene-like graphs, shared/frozen revisions and existing temporary
Git/archive replay. PKG-004 is included for that native integration test only.
Allowed: optional tig-assets recipe module, scoped tests and bounded docs.
No provider, cache IO, native consumer/runtime switch, automatic build/selection,
new instance format, source/job admission or eviction. Art donor
119c66446f2e95129d8d8f143d60ec6220d2813a (MIT) model.rs planning/freshness tests
provide semantic parity obligations; do not import its legacy record format.

Full source/context identity intentionally changes on annotation-only edits.
Rendering needs a separate declared consumed-input fingerprint. Begin with an
already complete Production-only Graph and its exact externally admitted graph
profiles; revalidate their canonical admission bytes before any additional lookup.
Require one explicit consumed-authored-field policy per visited definition and
an operation name, semantic digest, producer Object and required opaque parameter
document Object. Re-read exact bound native
records under the graph checkpoint with a separately bounded metadata budget.
Unknown/missing policy, fields, defaults or source identity refuse. No implicit
all-fields or no-fields fallback. Native envelope/schema execution remains its
existing scope; this does not make opaque producer coverage complete.

Normalize only consumed top-level authored properties using explicitly declared
definition defaults. Preserve omitted versus explicit null unless that exact
default is null. Never rewrite the record. Include exact admitted payload
selector semantics and each qualified selected/ordered named stream binding;
swapping two stream slots cannot disappear into a deduplicated object set.
Instance selected output is excluded only when the admitted graph selector says
so; a selected primary/control input remains consumed. Outputs cannot be canonical
production dependencies. This API describes consuming existing primary inputs,
not generation of a cel from its own prompt/selected result. Reference-valued
fields need not also be consumed as raw data: recursive graph bindings already
preserve their role/target meaning. Explicitly selecting such a field consumes
its raw value, including any exact pins.

Use iterative bottom-up DAG fingerprints: asset/definition IDs, class, consumed
fields, qualified
payload bindings and ordered outgoing Production edge pointers/child fingerprints.
Root fingerprints plus operation/semantics/producer/parameters determine the
recipe. Hash
framing is TIG-NUL-native-recipe-{base,node,root}-NUL-experimental-1-NUL with the
existing length-prefixed canonical item feed and counted collections. Never use
raw reference hashes, checkpoint, full definition/schema pins, status vocabulary,
metadata, stream provenance or frozen/selected addressing mode in the recipe.
Those remain exact provenance through the graph checkpoint/binding/admission and
consumption-policy digest. No implicit policy semantics: producers must declare
all actual consumed fields/code, and admission/eligibility gates still run even
when recipes match. Policy or producer changes can legitimately invalidate reuse.

Recursive child identity is essential: flattening projected vertices/edges can
confuse two frozen revisions of the same logical node wired to different children.
Sort final root fingerprints so raw-byte ordering cannot cause false invalidation.
Preserve duplicate roots and edge order. Stream fanout hashing rather than building
one unbounded canonical graph JSON document. Each canonical item keeps the 64 KiB
identity bound; graph nodes/edges and additional lookup bytes have explicit bounds.
Providers must bound their own memory/blocking IO; this is not capacity admission.

Returned plans are declared recipe identities with private exact provenance and
node fingerprints. Matching fingerprints are not execution, output correspondence,
schema/variant completeness, human approval, delivery currency or removal authority.
Produced-result claims, native consumer adoption, first-class translation/generation,
full model/variant parity and guarded catch-up scheduling remain separate work.

Exit tests must show metadata/unconsumed changes alter provenance without recipe
churn; consumed/default/payload/slot/order/wiring/producer changes alter recipes;
shared and frozen inputs remain exact; gaps/foreign profiles/provider substitutions
and bounds refuse; a non-media report and scene-like DAG exercise the public API.
Six producer role lenses must review identity, policy, custody, concurrency,
privacy and adoption findings separately from tests and human acceptance.

## Parameter-binding review amendment

Entry c9348e49154f235c36dede6a5eaddc74d6634eae, same WP/REQ/SPEC/IF/PKG scope.
L1 adds a language/profile/encoding matrix and conflicting byte-pin tests; L2
freezes two scene descriptors and uses their exact Objects as parameter pins.
Before produced-result comparison, bind caller parameters separately from static
operation semantics and producer code. Otherwise two language requests consuming
one bilingual scene can share a fingerprint incorrectly. Operation.parameters
is required even for an explicitly empty parameter document; no implicit default.
It is opaque byte identity, not parameter interpretation, profile resolution or
automatic semantic normalization. Callers must include all actual parameters and
retain/verify that document in their separately owned production context before
execution. The recipe API neither archives nor reads it, and matching pins do not
prove its availability, content semantics, custody or actual producer use. Producer
and parameter pins must have safe sizes consistent with each other and graph pins.
They are additional producer requirements, not silently added to Graph's primary/
control inventory. A native frozen descriptor can retain the parameter bytes;
the caller must verify its exact Object matches Operation.parameters before
admitting execution. Full tool/runtime closure remains separately required.
No native consumer/runtime changes or output-freshness claims are added here.
