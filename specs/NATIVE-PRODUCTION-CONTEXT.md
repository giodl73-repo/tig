# Frozen declared production context

Entry e7d2bde2f309c6f8a78d259e496b2ae59849dafb. WP-008/009/010/011;
REQ-001/003/004/006/007/008/010; SPEC-005/008/010/011; ARCH-003/004/005;
IF-002/005/008/010/012; DES-001/004/005/007/009; PKG-004/006. RUST/GIT/PROVIDER
profiles: L0 fmt/Clippy/repository/canonical VTRACE; L1 missing/corrupt/substituted
source/policy/roots/roles, descriptor and resource bounds; L2 real temporary Git,
metadata-store and source-journal reopen with independent main/job obligations.
Allowed scope: graph admission-document access, native context adapter, archive
IO helper reuse, synthetic tests and bounded docs. No media IO, cache creation,
new provider, automatic job enrollment, daemon, removal or runtime switch.

A source archive and a loose input list are insufficient to reconstruct an owned
job. Freeze from an actual AssetSnapshot, exact Production query, caller-admitted
profiles and an opaque job descriptor (at most 65,536 bytes). Resolve against that
snapshot again before writes; do not accept externally constructed object sets.
The descriptor can carry producer/request identity; Tig stores exact bytes but
neither executes it nor infers dependencies hidden inside it. The producer must
account for every external input in its admitted query and validate its semantics.
Credentials remain outside descriptors. Canonical source dependencies and optional
derived-cache reuse/physical execution leases are separate producer concerns;
rendered outputs cannot be relabelled primary dependencies to obtain reuse.

Reuse NodeStore/Tree with an experimental context header:
`tig.native-production-context.experimental-1`. Persist the source archive,
the exact already-canonical admission document, descriptor bytes, exact query
roots and declared primary/control/interpretation pins in one immutable tree.
The Selection root is that Tree Object; its context digest pins the header bytes.
The header pins checkpoint, admission/binding digests, archive/policy/descriptor
Objects and role/root counts. Metadata index namespaces are explicit:

| Entry | Value |
| --- | --- |
| context/identity | Exact header Object |
| source/archive | Exact native source archive Object |
| source/policy | Exact canonical admission document Object |
| operation/descriptor | Exact opaque descriptor Object |
| roots/record-SHA256 | Bound root instance Object |
| primary/payload-SHA256 | Declared primary Object |
| controls/payload-SHA256 | Declared container control Object |
| interpretation/document-SHA256 | Exact instance/definition/schema Object |

These are metadata entries, not physical media copies or another authored list.
Delivered/selected outputs keep independent custody claims. This context stores
source interpretation and descriptor bytes, not media/control payload bytes or
undeclared producer/runtime dependencies. It is not a consumed-property recipe,
complete semantic proof, producer execution proof or public/sanitized receipt.
Context changes are provenance changes, not automatic render invalidation: a
separate consumed-property recipe and eligibility policy must decide reuse.

Restore requires an externally admitted Selection and profiles plus explicit
native/tree/query bounds. Verify the complete tree/header, policy, source archive,
descriptor and root documents; restore the exact source view, replay the query
and compare admission/binding identities, counts and every typed entry. Refuse
missing, corrupt, extra or substituted entries and changed policies. Never load
policy authority from the archive itself or resolve host-global latest values.
An identity-only descriptor reader supports choosing the caller's decoder/policy
before full replay. It verifies the header/descriptor pins but grants no semantic,
availability or execution authority; it must not be treated as restored context.

Freeze returns a candidate Selection compatible with the existing SourceJournal.
The caller must explicitly finish provider writes, reopen/readback, then perform
owner-authorized prepare/adopt before its job/ref handoff. No atomic transaction
spans those steps. A failed build can leave metadata prefixes but preserves old
roots. Global job/source enrollment, admission/capacity, full crash/restore authority,
metadata compaction, payload availability/custody and fresh eviction fencing remain
separate gates. No timeout, completed descriptor or absent root authorizes removal.
Native archive, context-tree and graph limits apply to their own stages; these
logical budgets are not a shared capacity reservation or total measured memory/IO.
Root documents are recovered from the already verified archive, not reread as an
additional unbounded provider stream. Providers must bound their own allocation
and blocking IO. This API covers complete Production queries; incomplete draft
authoring and other job purposes need separately admitted protection semantics.

The optional [job resource context](NATIVE-JOB-CONTEXT.md) links this retained
source context with the original recipe, explicit runtime pins and checked
unsigned result evidence. It reuses the same metadata and owner-journal ports;
native execution, complete enrollment and custody remain separate obligations.

Exit: an actual context replaces the earlier synthetic test helper, replays after
Git/working metadata are moved aside and SQLite/journal reopen, and an independent
job keeps its old context after main advances/retires its own predecessor. Negative
tests must refuse missing source/policy/descriptor, wrong witness/policy/roles/roots,
limits and interrupted writes; unchanged document/tree sharing remains possible.
