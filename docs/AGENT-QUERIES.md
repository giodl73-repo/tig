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
| find | project, text; optional category | Registered IDs/titles/types; empty text lists the scope |
| shared | left/right project IDs; optional category | Unique shared selected objects and their record bindings |
| references | project, sha256 | Direct selected and named-stream bindings, including retained streams |
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
