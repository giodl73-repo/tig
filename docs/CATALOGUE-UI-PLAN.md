# One catalog across Tig projects

WP-015 / REQ-006/007/010, SPEC-014, IF-010/011/013, PKG-004/006/008;
UI and INTEGRATION profiles. See [the contract](../specs/CATALOGUE.md) and
[producer review](reviews/2026-11-05-catalogue-ui.md).

The first screen presents works, not every record in an internal registry.
A project admits root selectors, grouping keys, member selectors and status
interpretations. One episode remains one work when it has two languages,
multiple versions, captions, evidence and hundreds of primary inputs. Project
names, categories and values such as golden remain project data.

```text
Project -> collection -> work -> versions / deliveries / related files
                           -> technical record inspector when requested
```

Default enrollment can select one project. Collection counters distinguish
all registered works from a configured selected state. Facets support season,
volume or other project groupings. Plain records stay in Technical records.
Work grouping is a presentation projection; it never changes asset identity,
authoritative dependencies, selected bytes, approval or placement claims.

| Milestone | Product | Status |
| --- | --- | --- |
| U1 | Native multi-project intake, search, paging, exact refs and inspector | Implemented; initial flat view retained for technical inspection |
| Work view | Owner-admitted collections, current-state counts, facets and nested files | Implemented; generic fixtures and producer-run browser tasks qualified, not independent acceptance |
| U2 | Full production/delivery histories, profile explanations and snapshot comparisons | Partial: registered versions/files and bounded declared-version comparison implemented; full production/delivery/profile explanations remain open |
| U3 | Inventory/delta, working-set, custody and measured allocation views | Named object/preferences/copy review, guarded verification and global capacity implemented; coverage and work/version comparison implemented; full custody and eviction remain open |
| U4 | Fenced handoff to enrolled authoring tools | Open |

Also open: previews/playback, interactive enrollment, native/WASM parity and
portable Git/LFS-only presentation adapters. External authoring editors remain
separate tools. Do not turn them into another asset resolver or runtime registry.

Use synthetic media/non-media fixtures for duplicate IDs, languages, same-byte
runs, orphan members, missing payloads, stale requests, schema failures and
paging. Explicit private acceptance may inspect metadata; no private assets,
prompts, voices, enrollment or screenshots enter source or public examples.
A grouped catalog count is not proof of delivery, freshness, custody or eviction.

Storage now admits retained observations and explicit registered-object metadata
refresh. A colored size map, cache/family/work/status filters and work/version/
object pivots share unique summary accounting. Historical review estimates stay
separate from safe reclaim, which remains unknown. Source timestamp, incomplete
scope, absent observations and allocation alias gaps remain visible. Tests use
synthetic shared objects, aliases, selected/history overlap, corrupt inventory,
budget refusals and actual temporary Git/store refresh. No automatic eviction.
