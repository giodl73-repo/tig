# Explicit manifest queries

Input 0f292ba8aa24e7fd652872f9cda866e9cea31d0c; WP-008/010/015;
REQ-001/005/006/007/010; SPEC-005/010/014; IF-005/008/009/010/013;
PKG-002/004/006/008. RUST/INTEGRATION/PROVIDER reader profiles.

Entry: native works, owner-admitted nested JSON adapters and existing verified
readers. Scope: works discovery and explicit manifest_references agent queries.
One enrolled project/collection/work per manifest query; require a current basis
before metadata byte IO. Reuse named traversal with stricter query ceilings:
256 roots/documents, 4 MiB/document, 16 MiB total, 10,000 references, depth 8.
No media-leaf reads, discovery scan, hydration, cache creation, writes or new index.

Exit: provenance includes root/slot, exact containing document, adapter and JSON
pointer. Failed roots remain partial and publish no partial associations. Content
fences include coverage so recovered/missing/corrupt documents cannot mix pages;
offset pages require expected_content. Hash storage/adapters into the source basis
without exporting physical paths. Report metadata read attempts separately;
payload-open count is unknown for named byte reads, media-leaf reads stay zero.
Associations are not semantic production consumers or retirement authority.
Existing Git LFS layouts reuse the reader; arbitrary LFS source extraction and
nested semantic-graph bridging remain outside this slice.

L0: source/format/hash/privacy; L1: focused CLI/reader/schema tests, Clippy and
standalone build; L2: shared-child provenance, paging/recovery fences, missing/
corrupt/duplicate-key/schema failures, query ceilings and flat/LFS layout parity.
Producer roles: identity, provider/privacy, Git/adoption and concurrency. No
independent review or complete project/job/custody claim.

Producer review of query/manifests, nested reader boundaries, request/result
schemas and synthetic agent fixtures used the four named role checklists.
Identity: exact native snapshot plus adapter/source basis; document provenance
retains both root and containing-object identity. Provider/privacy: only admitted
named JSON objects are read; path-bearing policy is hashed, never returned;
ignored prompt/URI fields remain private. Adoption: no second registry, scanner
or persistence layer. Concurrency: mandatory source fence precedes IO; required
content fences reject mixed offset pages after availability changes. No open
blocking finding within this scope; raw LFS extraction, semantic bridging and
full project/job/custody coverage remain open. Owner: query maintainer.

Verification: 32 agent-query scenarios pass, including seven new manifest cases
covering flat/Git LFS layouts, shared-child provenance, missing-child recovery,
corruption/schema failures, unadmitted scopes, paging and 257-root refusal.
Strict all-feature/all-target workspace Clippy and no-default-feature check pass.
The full locked all-feature workspace suite, explicit CLI build, portable
repository checks and diff checks also pass. Hosted cross-platform qualification
remains required before protected merge.
Validation uses synthetic fixtures only; no live cache or enrollment is changed.
Trace closes this bounded agent slice only, not the parent full-system controls.
Producer review is neither independent review nor human acceptance.
