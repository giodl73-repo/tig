# Agent queries

Use Rust `tig-query` to discover and query enrolled assets. No project-specific
script, media hashing, daemon or catalogue server is required. Build with:

```text
cargo build -p tig-git --features schema --bin tig-query --locked
tig-query describe
tig-query run /absolute/request.json
tig-query run /absolute/request.json --out /absolute/new-report.json
```

Describe emits request/result JSON Schemas, limits and exit meanings without
opening a project. Run emits one JSON document on stdout; stderr has no normal
diagnostics. With --out it atomically installs a new full report without
overwriting existing files and prints a compact JSON acknowledgement.

The library is `tig_git::catalogue::query`; existing captured catalogues can call
`Catalogue::agent_query`, reusing their loaded native metadata. The CLI loads only
the projects requested by the operation. It never chooses main or a dirty file.
Supply a machine-local [catalogue enrollment](../specs/CATALOGUE.md) with exact
resolver pins and owner-admitted graph profiles. Keep request/results outside
source Git. Windows paths can use forward slashes in JSON.

```json
{
  "schema": "tig.query.experimental-1",
  "enrollment": "/absolute/enrollment.json",
  "query": {
    "operation": "impact",
    "project": "example",
    "asset": "cel/b",
    "kinds": ["production", "composition", "publication"]
  }
}
```

| Operation | Query fields | Result |
| --- | --- | --- |
| projects | none | Enrolled IDs, exact commits and category summaries |
| inspect | project, asset | Exact record/definition pins, selected object, named streams and owner-admitted inspector fields |
| admission | project, kinds; optional definition ID filter | Used definition revisions, property/schema pins, admitted rules/classes and missing kind coverage |
| preview_admission | project, asset, kinds, profiles; required expected_basis on request | Simulated full profile-set replacement, baseline/candidate impact and paged consumer/witness changes |
| find | project, text; optional category | Registered IDs/titles/types; empty text lists the scope |
| works | project; optional collection, text, state, facet | Owner-admitted work IDs for explicit manifest scope selection |
| shared | left/right project IDs; optional category | Unique shared selected objects and their record bindings |
| references | project, sha256 | Direct selected and named-stream bindings, including retained streams |
| manifest_references | project, collection, work, sha256; required expected_basis on request | Byte associations inside explicitly admitted JSON manifests, with containing-document provenance |
| compare | left/right objects with project and id | Record, definition and selected-byte equality; stream additions/removals/changes; changed authored field names |
| impact / where_used | project, asset, kinds | Potential direct/transitive consumers with exact revision paths |

Find, references and admission accept project `all`. Other selectors name explicit projects.
Compare can span projects. Impact traverses one project's admitted namespace;
cross-project semantic links are excluded. Edge kinds are explicit so an agent
does not confuse production, composition, publication, evidence and authority.
Projects reports the admitted dependency-profile count. Partial impact results
list unadmitted definition pins and affected record counts. Admit profiles from
the project's authoritative schema into its external enrollment; do not infer
references from text or filenames. Re-query with a fresh basis after admission.
Admission reports separate revisions even when their definition IDs match. Exit
0 means valid profiles declare the requested coverage for used definitions;
instance edges remain unevaluated. Use the [graph contract](../specs/NATIVE-GRAPH.md)
when reviewing classes, pointers and binding modes, then run impact.
Only explicitly admitted display/inspector values are exposed by those views.
Asset IDs, inspector fields and structural dependency-map keys can themselves be
private. Inspect reuses the owner's existing inspector admission; raw authored
maps, definition defaults and schema examples are not exported.

An impact path runs from consumer to the queried selected record revision. A
diamond yields one consumer with a deterministic shortest witness. Frozen older
record pins remain distinct. A path expresses possible impact; production recipe
and delivery checks determine whether work is required. Decoded media equivalence
remains a renderer/adapter operation.

## Preview profile changes

```text
admission -> reviewed profile set -> preview_admission -> owner enrollment update
                                                           |
                                                     fresh impact query
```

Preview requires a current expected_basis from a query for the same project.
Profiles is the complete proposed set, including unchanged profiles. The tool
validates and simulates it against the captured native namespace without editing
the owner's enrollment. Invalid candidates refuse with invalid_proposal. An
invalid baseline can be repaired in simulation and remains explicitly unavailable.

The result names baseline/proposed admission digests, both coverage/gap reports,
known consumer counts and delta rows: newly_known, no_longer_known and
witness_changed. Witness fingerprints describe deterministic shortest dependency
paths, not complete graph/production recipe identity. Paths can disappear because
an interpretation lost coverage; even a complete comparison grants no retirement
or admission authority. A partial baseline with a complete candidate returns
exit 3 because the comparison remains incomplete; inspect candidate coverage.

Delta rows page across the full known result; baseline/candidate witness details
show their first 100 consumers. Keep candidate profiles, target and kinds fixed
across pages and check proposed_admission_sha256. Review before editing enrollment;
then query again with a fresh basis. Each of the two graphs uses the normal impact
limits. Preview creates no durable query index or configuration change.

## Explicit manifest references

Use works to obtain the real collection/work IDs and basis_sha256. Set that basis
as expected_basis on a manifest_references request. The query reads only named
metadata objects through the owner's existing nested adapters and admitted byte
sources, including Git LFS cache layouts. It never opens referenced media leaves.
This operation performs byte IO; native references remains metadata-only.

Rows identify the root asset/slot/object, containing document, adapter and exact
JSON pointer. They describe direct byte associations, not semantic production
consumers. Unknown scopes, missing adapters and failed documents cannot establish
absence. A failed root contributes no partial associations; successful roots can
still yield a partial report. Unrequested scopes remain unknown.

For another page, keep the query fixed, pass the same expected_basis and set
expected_content to data.content_sha256 from the previous result. This content
fence is required for offset pages. Changed extraction content or availability
refuses with stale_content after bounded reads; changed source/adapter admission
refuses with stale_basis before byte IO. Neither query adopts a persistent index.

Owner limits are further capped at 256 roots/documents, 4 MiB/document, 16 MiB
total declared metadata bytes, 10,000 references and depth 8. Verification uses
at most 5 seconds per reader operation, within the traversal's 60-second deadline.
Metadata read attempts and decoded-document counts are separate. They do not
measure exact physical handles or allocation. coverage.payloads_opened is null
for this operation because that count is unknown; media_leaves_opened stays zero.
Raw Git LFS pointer extraction and nested semantic-impact bridging remain separate
adapter work. No scan, hydration, cache creation, transfer or deletion is implied.

## Agent decision loop

1. Describe and validate the supported request/result schemas.
2. Query projects/find to obtain the real IDs; do not invent names or hash files.
   Use inspect for a record. Use admission to discover exact definition revisions
   and reviewed extraction rules before interpreting a missing-profile impact gap.
3. Compare records or ask impact with admitted edge kinds.
4. Inspect status, coverage, gaps and pagination before acting on a zero count.
5. For another page, set offset to paging.next_offset and expected_basis to the
   prior basis_sha256. A changed commit, captured namespace or interpretation
   refuses with stale_basis. Optional expected_snapshots also fences exact commits.
6. Treat impact as a triage list. Re-run after fixes; evaluate producer/delivery
   gates separately. These queries never grant deletion or publication authority.

Exit 0 means complete within the requested declared scope. Exit 3 means partial
declared coverage: use the returned known paths and inspect gaps. Exit 2 is a
structured refusal/error; no successful report was published. Complete project
coverage remains false even for exit 0: unregistered files, unadmitted nested
inventories, other LFS pointers, cross-project dependencies, jobs and custody
are excluded. Missing profiles, cycles and graph limits cannot establish absence.

Requests are at most 64 KiB; output is at most 16 MiB; pages contain at most 100
rows. Text/category limits are 256 UTF-8 bytes. Impact bounds are 10,000 nodes,
40,000 edge attempts, 64 levels and 16 MiB returned graph metadata. Gap details
are a bounded first-100 sample with a full gap count; truncated details remain
explicit. There is no gap-detail pagination or persisted query index.
