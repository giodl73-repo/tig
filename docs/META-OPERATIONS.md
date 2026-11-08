# Comparing and finding assets

Tig compares immutable byte identities and declared references at exact snapshots.
Project adapters define the meaning of an episode, photograph or dependency.
Cache paths and filenames do not establish equality.

| Question | Operation | Meaning |
| --- | --- | --- |
| Do these projects share selected bytes? | Catalogue: Shared assets; CLI: shared | Same recorded SHA-256 and exact length, grouped with each project's asset IDs |
| Which records bind this object? | CLI: references | Direct selected and named-stream references, including retained streams |
| Where is it referenced inside an inventory? | CLI: works then manifest_references | Explicit admitted JSON scope; root, document and JSON-pointer provenance |
| What changed in declared version content? | Work -> Versions -> Compare declared bytes | Added/shared/removed declarations in admitted version manifests; coverage remains explicit |
| What changed in declared dependencies? | tig-graph inspect/compare | Caller-admitted graph/profile semantics; see [graph inspection](../specs/NATIVE-GRAPH.md) |
| Are two videos perceptually equivalent? | Renderer/media adapter | Requires decoded media semantics; equal filenames or similar previews are insufficient |

Use a machine-local catalogue enrollment with exact commit pins and externally
admitted category rules. Both CLI operations run without the catalogue server:

```text
tig-catalogue shared ENROLLMENT project-a project-b NEW_REPORT [CATEGORY]
tig-catalogue references ENROLLMENT SHA256 NEW_REPORT [PROJECT] [OFFSET]
```

ENROLLMENT and NEW_REPORT are absolute paths. Reports are create-new JSON files
outside source Git, capped at 16 MiB; stdout contains totals, not the private
record list. Reference lookup defaults to all enrolled projects and offset zero;
it returns 100 bindings/page. Increment OFFSET by the returned row count to
continue; preserve the same snapshot pins while paging. A missing object has
zero bindings within the enrolled scope, not proof of absence everywhere.

The UI's Shared assets navigation offers two project selectors and an optional
category filter. Each object opens its registered asset records. The view shows
unique shared bytes once, reference totals and both pinned Git commits. It does
not sum duplicate cache copies or claim bytes can be reclaimed. Refresh redraws
captured snapshots; changing a profile or enrolling newer refs requires restarting
the explicitly launched server.

These queries open Git metadata, not media. Selected-object comparison excludes
other streams, nested inventories and unregistered files. Reference lookup also
includes named streams but does not traverse semantic dependencies. Git LFS
pointers outside enrolled native records require a separate source extraction
adapter; none is silently substituted. Query coverage and custody are separate.
Changing categories or observing zero known references never authorizes eviction.

The [agent interface](AGENT-QUERIES.md) unifies find, shared, references, record/
stream comparison and explained transitive impact through admitted graph profiles.
Explicit manifest_references reuses admitted nested adapters and verified byte
readers, including Git LFS layouts, with source and content fences for paging.
It reads named metadata documents; media leaves remain unopened. It does not
turn byte associations into production edges or authorize retirement.
Remaining work includes raw LFS source extraction, nested semantic-impact bridging, cross-project semantic
dependencies and UI access to full graph comparison. Reuse the
native graph and source extraction mechanisms rather than adding project scripts
or a second hash database. Persisted indexes must be derived from exact snapshot
identities and invalidated when their source or admitted rules change.
