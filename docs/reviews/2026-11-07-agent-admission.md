# Agent admission diagnostics

Input 37b77c8a5d97a663875ae573c6ef9f40888f82a7; WP-010/015,
REQ-001/006/007/010; SPEC-010/014; IF-008/010/013; PKG-004/006/008.
RUST/INTEGRATION profiles. Entry: versioned agent query interface and exact native
metadata. Scope: inspect through the existing admitted inspector and paged
definition/admission discovery with explicit requested edge kinds.

Exit: agents discover exact used definition pins, property names, schema pins,
admitted classes/rules and missing kind coverage without raw instance values,
definition defaults or schema examples. Definition names cannot coalesce revisions.
Missing admission stays a diagnostic; it does not infer dependencies or mutate
owner configuration. Reuse native metadata lookup/schema parsing. No cache/media
reads, new databases, daemon or browser action. Source/docs cap expands from
3.6 to 4 MiB for generic code, fixtures and contracts; 128 KiB/file and payload
exclusions remain. No storage safety or runtime admission limit changes.

L0: source/format/hash/privacy; L1: focused native/CLI/schema tests, Clippy and
standalone feature check; L2: missing/partial/complete admission, same-ID definition
revisions, exact source, paging and stale interpretation. Roles: identity,
provider/privacy, Git/adoption and concurrency, applied as producer checklists.
Independent acceptance and full semantic coverage remain open.

Producer findings: one shared profile validator preserves traversal admission
identity (existing graph vectors and direct fingerprint parity pass). Same-ID
definition revisions remain separate. Invalid profile sets request repair before
admission; covered kinds do not certify evaluated instance edges. Inspector
field admission enters the stale basis. Contract output omits defaults/examples;
inspector values remain owner-admitted and can be private. Seven new scenarios
bring agent tests to 18; 90 graph, 12 schema and 13 catalogue tests also passed.
Strict Clippy, standalone/source checks and hosted qualification are recorded
separately; no independent reviewer or full consumer coverage is claimed.
