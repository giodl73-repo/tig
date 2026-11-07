# Experimental native record intake

WP-010 bounded native slice; REQ-001/006/010, SPEC-010, ARCH-005, IF-010,
DES-009, PKG-001/006; CR-001..008, RUST profile. Its explicit diagnostic CLI is
the bounded WP-015 / IF-013 / PKG-008 subset, not a full inspector. Entry input is Tig
e2b944f5d03513c1bc6702519a29a91c4de0e3a7 and the existing design-1 schemas.
Art donor 119c66446f2e95129d8d8f143d60ec6220d2813a remains read-only; its
model/graph/variant tests have not been ported and are not this slice's evidence.

Scope: optional tig-assets pure reader plus explicit read-only tig-record CLI.
Exit: exact definition/schema binding, bounded malformed/conflicting negatives,
ordered mixed streams and two synthetic consumer families. No store/REEL changes.
L0 fmt/structure/Clippy; L1 native integration tests; L2 actual CLI with synthetic
separate instance/definition/schema files. Full WP-010 and consumer cutover stay open.

The reader accepts the design-1 envelopes using tig-core's strict 64 KiB JSON
subset: 4,096 nodes, depth 32, safe integers, no floats, duplicate keys rejected.
Raw byte SHA-256 identifies the received file, including whitespace. Parsed
values have read-only access; changing source requires parsing a new revision.
Media/native documents with fractional numbers remain separately pinned opaque
bytes. They must not be rounded or rewritten to fit this intake subset.

Instances pin exact definitions; definitions pin exact authored-schema bytes.
Bind verifies all three identities/IDs and declared/required authoring fields.
It does not execute JSON Schema, stream_rules or semantic_rules. Defaults stay
declarations and do not mutate source. Unknown semantic meaning always leaves
complete_semantic_coverage false. A successful bind cannot assert primary-input
closure, freshness, human approval, delivery, protection coverage or custody.

At most 256 streams, each with a unique stable slot and ordered role/group/pin.
Unusual mixed selections are preserved. Same-digest/different-size pins reject.
Selected may point to an external result manifest rather than a stream; planned
instances require explicit selected:null. Status is generic project vocabulary.
There is no schema field or built-in policy named golden.

The CLI reads only its three explicit files, each at most 64 KiB; it initializes
no cache, provider, Git config or daemon. inspect emits bounded identity/counts
and incomplete coverage; stream emits one selected slot's byte pin. Neither emits
authored text, status contents or provenance, nor reads the media bytes. File paths
are caller-trusted local inputs, not an OS sandbox. This is not yet the native
consumer resolver, graph extractor, producer adapter or final transfer interface.

Optional schema execution is specified separately in NATIVE-SCHEMA-VALIDATION.md.
Existing bind/inspect retain their envelope-only contract. Explicit validation
checks the exact authored schema under an admitted dialect/engine policy, without
materializing defaults, retrieving external resources or executing stream/native
semantic rules. Its checked result still reports incomplete semantic coverage.
