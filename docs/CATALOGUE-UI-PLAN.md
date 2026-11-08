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

Record Impact now has explicit relationship choices, 25-consumer pages and exact
expandable paths. Work Overview offers a source-record shortcut. Known counts,
graph gaps and missing definition-pin rules are visible; source/basis changes
refuse mixed pages. Selected consumers open their exact current record, with
return navigation; historical revisions are labelled without a newer-record
substitution. This metadata-only UI does not expand inventory bytes. Conditional
manifest impact remains available through the agent CLI. Browser qualification
is separate from controller and authenticated loopback API fixtures.

Integrated controller fixtures now load the shipped scripts in HTML order with
two synthetic projects and a bounded DOM/fetch double. They exercise work/source/
impact/consumer return flows, duplicate titles, 63-consumer paging, 128 missing
definition rules, cross-project stale replies and scoped zero results. Impact
shows stable IDs, the pinned snapshot and shown/known counts; truncated rule
samples are explicit. These fixtures supplement real Rust API tests and do not
claim browser layout, keyboard or live-project acceptance. Run them with:

```text
node --test crates/tig-git/tests/work-selection.cjs crates/tig-git/tests/impact.cjs crates/tig-git/tests/ux-flows.cjs crates/tig-git/tests/compare-flows.cjs crates/tig-git/tests/object-flows.cjs
```

Record Compare adds an in-memory exact baseline across project navigation and
explicit comparison through the existing Rust agent operation. Its workflow
fixtures cover equal bytes/different records, missing selections, changed
definitions/slots, cross-project snapshots, baseline races, retry and privacy.
Run them with `node --test crates/tig-git/tests/compare-flows.cjs`. The baseline
does not reserve storage or silently resolve an older revision after a project
changes snapshots. Full browser/provider acceptance remains separate.

Object workflows connect registered byte bindings to the declaring owner's
Overview, Compare and Impact tabs, preserving the captured object scope and
references tab on return. Integrated fixtures cover shared bytes across projects,
retained/nested evidence, explicit unknown owners, sampled bindings, source and
identity refusals, same-object request races, retry and late held-review replies.
These shortcuts inspect owner records; they do not infer production edges from
byte associations or authorize eviction. Synthetic controller/API checks do not
establish visual/browser acceptance.

Record-inspector fixtures additionally reproduce repeated-card response races,
stale errors, pending/failed action clearing, wrong response identities, enrollment
snapshot changes and dependency-tab reentry. Inspection and detail-render tokens
isolate those requests without locks or extra API calls. Escape and existing
work/object/consumer return flows remain covered.

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

Collection totals now toggle filters; Ctrl/Meta-click combines their configured
statuses. Shared assets compares two pinned projects through the same Rust
selected-object query used by the CLI. Direct object-reference lookup includes
selected and named streams. See [meta operations](META-OPERATIONS.md) for scope
and remaining nested/LFS/transitive query adapters. Browser qualification of
these new controls is separate from automated controller/API checks.

Storage now admits retained observations and explicit registered-object metadata
refresh. A colored size map, cache/family/work/status filters and work/version/
object pivots share unique summary accounting. Historical review estimates stay
separate from safe reclaim, which remains unknown. Source timestamp, incomplete
scope, absent observations and allocation alias gaps remain visible. Tests use
synthetic shared objects, aliases, selected/history overlap, corrupt inventory,
budget refusals and actual temporary Git/store refresh. No automatic eviction.

Planned bulk retirement: select blocks or filter admitted categories and age from
recorded lifecycle events, then freeze exact asset/version/object scope for a
shared CLI/UI plan. Preview eligible/held items, external sharing, affected stores,
loss of historical recoverability and estimated versus measured reclaim. Final
backup deletion needs distinct authority from cache eviction; stale scope and
unknown coverage hold removal. Partial batches and restart have explicit outcomes.
See [the storage sequence](../PRODUCT_PLAN.md). These controls are not implemented;
no selection or size-map gesture grants deletion authority by itself.
