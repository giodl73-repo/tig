# Exact sparse native variants

Entry2f0c2dc0d0bffd88483dc6442a4de84cb240929f; parent35/36 qualification required.
WP-010/011; REQ-001/006/008/009/010; SPEC-010/011; IF-010/011; PKG-004/006.
Allowed: pure optional typed variant API, synthetic/native Git/recipe fixtures and
bounded docs. No provider/cache initialization, producer, renderer, new dependency,
hook, selection mutation, authority or eviction. RUST/GIT/INTEGRATION L0: structure,
fmt/Clippy/standalone; L1:14 adapted donor behaviors plus native pin/privacy/bounds;
L2: actual native Git edits and projected primary/recipe requirements.

Design/source donor: Art119c66446f2e95129d8d8f143d60ec6220d2813a,
crates/art-core/src/variants.rs and tests/variants.rs (MIT; same copyright/notice
as root LICENSE). Preserve sparse exact selection, consumed-axis projection,
owner/definition/space/semantics locks and no fallback. Tig uses NEW experimental
wire/subject identities, native raw Object pins and slot references, not Art hashes.

Space document: schema:tig.variant-space.experimental-1, axes:{name:[values]}.
Index document: schema:tig.variant-index.experimental-1, asset:<logical ID>,
definition:<exact Reference>, space:<raw Space Object>, semantics:<Digest>,
uses:[axes], entries:[{parameters:{axis:value},slot:<native stream slot>}].
Documents are strict externally pinned64KiB JSON, immutable after parse. Bounds:
16axes,1..64 values/axis,1..256 entries (strict JSON node/byte limits also apply).
Axis/value tokens1..64 ASCII alphanumeric/_.-, starting alphanumeric. Slots follow
native1..128 byte/no-control rules, never provider paths. Reject unknown fields,
duplicate keys/sets/contexts, conflicting sizes, malformed or unconsumed entry axes.

Index entries name existing ordered stream slots: they do NOT duplicate payload
Objects. One cue ID may expose en/es streams; a cel may expose visual styles;
neutral assets have one explicit zero-axis entry. No Cartesian expansion or
automatic fallback. Every supplied context axis/value must exist even when unused.
Required axes must be supplied. Missing combinations, streams or selected index
refuse, not planned defaults or matches to a nearby language/profile.

Caller admission supplies expected definition, space Object, semantics and consumed
axes independently of document claims. Require the exact record's selected Object
to equal the index pin and owner/definition to match. Resolve against immutable
Resolved metadata, without provider IO. Selection retains source checkpoint,
record/index/space provenance and returns exact slot/payload/projected parameters.
Its domain-separated subject binds owner, definition, admitted semantics, projected
parameters, slot/role/group and payload; unrelated axes/stream edits preserve that subject.
Source provenance still changes. Definition/slot/semantics changes cannot masquerade
as the same selection. Debug reports counts/pins, never raw parameters/private IDs.

Project extension: Space.admit validates the entire project context into a sealed
Context before projection. resolve_in_context additionally requires its externally
admitted current project-space pin; an asset's original pinned space still checks
all consumed parameters/entries. A new project language can therefore reuse old
language-neutral cels without rewriting their record/index/space. Old cues lacking
that language still refuse. The convenience resolve validates full context against
the asset space as before. Current context-space provenance/control pins change;
unused project dimensions do not change the selection subject. No implicit
cross-schema migration, unchecked context or language fallback is introduced.

Use projected context/selection subject in producer parameters when appropriate;
retain full context as separate provenance. Index/space are interpretation control
objects, NOT audio/image payloads. controls() names them explicitly for caller
admission/retention via existing context ports. Returned selections are not recipes,
freshness, native consumption proof, approval, byte integrity or custody authority.
No implicit graph/REEL variant execution or complete native replay is asserted.
Actual family mapping and minimal private bilingual producer closure remain open.

Producer six-role pre-code review: identity checks exact pins and owner/admission;
privacy requires redacted diagnostics; Git/adoption avoids duplicate payload lists
and per-combination files; recovery forbids silent fallback; placement/custody
keeps interpretation, payload and global claims separate. Native tests must show
unrelated Spanish/style edits keep English audio subject/recipe while provenance
changes; zero-test donor claims are forbidden.

Context provenance refinement, before implementation: a sealed Context also owns
strict canonical tig.variant-context.experimental-1 bytes (space Object + full
parameters), an Object pin and explicit read-only getters. Selection retains that
pin separately from its projected subject. controls() includes the context document
alongside index/asset-space/current-project-space; callers retain these assets
separately from primary bytes. Unused dimension edits change context provenance
without changing selected audio/recipe. No parsed context can bypass Space.admit.

CC-VARIANT-SOURCE: the source is already2,087,508 bytes before final controls/tests
and review records. Raise only the ordinary source/docs total from2MiB to3MiB for
actual Rust schema/variant/native integration and locked dependency/test coverage.
Keep128KiB per-file, all runtime/media/binary exclusions, all checker commands and
both CI platforms unchanged. This is development-source capacity, not cache or
artifact-repo storage. Producer Git/privacy/custody review accepts this scoped
budget change; do not omit required coverage or hide source as ignored runtime
records to satisfy the former scaffold ceiling.
