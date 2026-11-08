# Local catalog application

SPEC-014 / WP-015, IF-010/011/013, PKG-004/006/008. Native Rust application
service and bundled frontend, gated by tig-git/schema. No daemon installation.

` tig-catalogue check ABSOLUTE_ENROLLMENT ` validates metadata without media IO.
` tig-catalogue serve ABSOLUTE_ENROLLMENT [PORT] ` explicitly serves loopback
(default 8799). Stop the process to stop the viewer.

The record inspector's Impact tab traces potential consumers using owner-admitted
relationship rules. A work's Overview links directly to its source record's
Impact tab. Choose relationship kinds and explicitly Trace consumers. Counts,
exact witness paths, unresolved relationships and missing definition-pin rules
remain separate. Current record links preserve return navigation; a historical
revision is displayed without silently opening its newer selected record.

The authenticated `/api/impact` GET accepts only project, id, snapshot, kinds,
offset and basis. One exact project/snapshot and a nonempty comma-separated set
of distinct production/composition/publication/evidence/authority kinds are
required. It reuses native agent impact and source-basis framing. Pages contain
at most 25 consumers; nonzero offsets require the previous basis_sha256 as basis.
Snapshot/basis drift refuses before graph traversal. The response reports scoped
status, coverage, paging and native data; navigation cards are supplied only for
the exact selected revision. HTTP response limits remain 4 MiB. No general agent
query, profile admission or manifest/byte reading is exposed through this route.
Missing coverage is unknown, never absence, regeneration or eviction authority.

Impact progress names both shown and known counts and the exact pinned snapshot.
Consumer labels include stable IDs even when display titles collide. Missing-rule
samples disclose omitted definitions. Work inspection, work lists, search and
metadata refresh discard stale successes/errors rather than overriding a newer
selection. Controller fixtures check the shipped scripts together; their DOM
double makes no browser-layout or provider-behavior claim.

Every record inspection has a unique request token even when reopening the same
card. Pending or failed inspections expose no earlier details/actions. Responses
must match the requested logical ID, project and captured enrolled snapshot
before admission; missing enrollment or source changes require refresh. Dependency
responses also belong to one detail render, so leaving and reentering the same
tab cannot append an earlier request's relationships to the newer view.

## Record comparison

Open a record's Compare tab, choose Use this record as baseline, then navigate
to another record and Compare with baseline. Work Overview has a source-record
shortcut. The in-memory baseline captures project, asset ID, exact record pin
and snapshot; it is a UI selection, not a retention claim or historical resolver.
Both source snapshots must still be captured by their enrolled projects.

The fixed authenticated `/api/compare` GET accepts left_project, left_id,
left_snapshot, right_project, right_id, right_snapshot and optional basis. Both
snapshots are required; project all, unknown fields and stale source/basis refuse.
The endpoint uses the agent's compare_assets and basis framing, returns snapshots,
coverage and data under tig.catalogue-compare.experimental-1, and performs no
payload IO. Existing HTTP limits and host/origin/session protections apply.

The UI independently checks both selectors, record pins and snapshot identities.
Changing/clearing the baseline or leaving the selection prevents late results
from being displayed. Equal selected bytes, record bytes and definitions are
reported separately. A missing selected object is unavailable, including when
both records lack one. Authored diffs include names only. Named streams report
additions/removals and before/after role/group/object changes, baseline to current.
Decoded media, transitive dependencies, producer currency and retained custody
remain separate; equality never establishes regeneration or eviction authority.

`tig-catalogue shared ENROLLMENT LEFT RIGHT NEW_REPORT [CATEGORY]` compares
selected object identities in two explicit pinned projects. Only those project
profiles are opened. Identity is SHA-256 plus length; conflicting lengths refuse.
Shared bytes count unique objects once; each side lists all matching asset IDs,
definitions and admitted categories. Categories are external presentation rules,
not inferred MIME or media semantics. The create-new absolute report path is
operator-owned, capped at 16 MiB; stdout contains totals and coverage flags only.
Reports name commits, record scopes, category counts and absent selections.
Named streams, nested inventories, unregistered files and other LFS pointers
are excluded. Payload presence, decoded equivalence, custody and eviction are
not established. No media is opened, rehashed or moved.

The authenticated `/api/shared?left=ID&right=ID&category=LABEL&offset=N`
uses those same captured snapshots. It pages 100 objects and at most 100
references per side/object, retaining full reference totals. The Shared assets
view names both pinned commits, excluded scopes and missing selections.
Matching categories need not represent the same media type across projects;
their meaning remains externally admitted. A zero result is scoped evidence.

`tig-catalogue references ENROLLMENT SHA256 NEW_REPORT [PROJECT] [OFFSET]`
lists direct selected and named-stream bindings, 100/page, across all enrolled
projects or one explicit project. `/api/references` accepts sha/project/offset
with the same calculation. Reports distinguish selected bindings and stream
slot/role/group, include captured commits, and reject invalid hashes, unknown
projects, conflicting lengths or offset above 1,000,000. These are byte bindings,
not transitive semantic consumers. Neither operation scans caches or infers
safe deletion from missing references. Private CLI report rules above apply.

## Enrollment and presentation

Machine-local schema tig.catalogue-enrollment.experimental-1 has projects
(1–32) and optional default_project. Each project has unique id, label, absolute
profile (existing tig.asset-resolver-profile.experimental-1), optional display,
fields, graph_profiles and collections. Keep bindings outside Git. Intake uses
exact Git commits, enrolled scopes and authored-schema validation. Invalid
configuration/intake refuses, rather than displaying an empty successful project.

Display rules are first-match id_prefix/definition conditions, with category or
category_pointer, title_pointers, optional basename and icon. Admitted fields
have label, pointer and source/delivery section; raw authored metadata is not
served. Unenrolled graph semantics remain unknown, not zero dependencies.

A collection has id, label, icon and roots; optional members, stages, facets,
count_state/count_label, version_pointer, version_role, version_slot_prefix and
delivery_pattern. Matchers have pointer (default /id), anchored Rust-regex
pattern, capture-expanded key and optional title. Explicit pointers address id,
authored or status strings. Root matches establish groups; member matches can
join existing groups but cannot create works. IDs remain project-scoped;
group keys are presentation identities, not new Tig instances or dependencies.

A stage supplies label, pointer, pattern and optional selected_slot_prefix.
It requires selected bytes; a configured slot prefix must resolve exactly to
those bytes using the matched field value. This is a registered selection
interpretation, not inferred creative approval. Facets name patterns over group
keys. Count-state filters do not count languages/versions/files as extra works.
Versions match the configured role; current selection requires both the exact
configured slot/version and selected bytes. Equal hashes do not merge runs.

## Boundaries and limits

GET /api/projects, /api/works, /api/work and technical /api/assets, /api/asset,
/api/dependencies, /api/snapshot, /api/verify use exact captured metadata.
Snapshot changes never checkout/fetch; failed capture preserves the prior view.
Details/checks require the displayed commit. Pages include snapshot identities;
refresh redraws captured data. Restart to enroll newer refs/profiles.

Summary totals are toggle buttons. Ordinary click selects one collection's
configured count-state; clicking the only active total again clears selection.
Ctrl/Meta-click toggles collections together. The works endpoint accepts a
comma-separated `collections` union (1–32 unique admitted IDs); each retains
its own count-state. Combining that parameter with collection/state/facet refuses.
The union preserves search, deterministic order, snapshot fencing and 100-row
paging. One collection's status must not be applied to another collection.

Exact loopback Host, foreign-Origin refusal and per-process unpredictable
X-Tig-Session admission protect APIs. Bootstrap supplies the token in its page,
never a URL. No-store responses, CSP, textContent and no cookies/CORS. This is
not hosted multi-user authentication; do not expose through a reverse proxy.

Budgets: 256 KiB enrollment, 64 KiB resolver profile, 32 collections/project,
16 root/member/state rules per collection, 32 facets, 512-byte anchored patterns
and 1 MiB regex compilation limits; 4,096 refs; 100 rows/page; 256-byte search;
4 MiB response; 16 KiB/3-second header; eight HTTP connections. Existing native
and guarded-reader limits still apply. Admitted detail fields truncate at 4,096
characters. Related-file lists page without exporting a second giant registry.

Technical byte totals describe selected objects, deduplicated per project, not
physical allocation. Selected subjects may be manifests. Targets/retained
receipts do not prove current cloud content. One verified guarded read proves
neither all replicas nor independent custody. Browse never scans caches, opens
payloads, hydrates, transfers, uploads or deletes. No removal route exists.

Remaining milestones and acceptance scope: [catalog plan](../docs/CATALOGUE-UI-PLAN.md).

## Nested catalog navigation and search

Collections may set top_level=false and admit a parent Matcher. These grouped
items remain discoverable through paged /api/children and /api/search without
inflating deliverable counters. Optional Matcher.definition restricts projection
to an admitted native definition ID. Parent fields are presentation relationships,
not production/retention dependency authority; missing parents remain unresolved.
Work details expose bounded ancestry (16, stopping cycles) and indexed children.
Child lists require the expected commit and page/filter independently of files.

Search scopes one project or all explicitly enrolled projects, with optional
collection/type filter and 100-item paging. It searches registered IDs, labels,
types and selected hashes, including grouped members, under existing query and
response bounds. Results retain project/snapshot identity. Package/media contents
are not indexed, and no cache traversal or payload IO is implied. Browser scope
controls and ancestry links expose the distinction from collection-only search.

## Read-only storage pivots

Optional enrollment `storage` (tig.catalogue-storage.experimental-1) admits one
exact metadata inventory Object, json/gzip encoding, decoded-byte budget,
cache-label/place-index/report-tier bindings and anchored owner projections.
The adapter accepts retained reel.asset-cache-report.v1 or equivalent
tig.cache-observation.experimental-1 observations without a REEL dependency.
Only an explicit storage request reads that named, guarded, hash-verified
metadata object. It never enumerates media directories or reads payloads.

GET /api/storage requires the project snapshot and supports cache, family,
work, version, reference-status and hash/version search. Pivots group by work,
version or object; 50 largest groups and 100 objects/page are returned. Size
sorting accepts recorded_file_bytes, measured_allocated_bytes (known physical
identities deduplicated) or observed_allocated_bytes (unknown aliases can
double-count). Unmeasured allocation stays unknown. Shared work/version totals
are not additive; the summary counts objects once and known physical IDs once.
Color is a presentation projection, not a retention decision.

/api/storage/retained restores the exact enrolled baseline after a successful
metadata load; failed restore preserves the current view.

An explicit /api/storage/observe request refreshes only named registered subjects
and streams from admitted top-level work members plus other registered records,
optionally one family. It
checks existing enrolled locations with live_probe=true; offline locations
remain unobserved. No directory traversal, hashing, media reads, copies or
deletion. Metadata source, commit, scope, time and unobserved registered count
remain visible. Limits: 100,000 unique objects; probe_millis defaults to 30 seconds and is capped at 120 seconds. Refusal preserves the
old observation. A source snapshot change rebuilds its view. Captured metadata
can immediately become stale. This is not complete dependency/job coverage.
Safe Windows APIs currently leave physical-identity and link coverage unknown;
observed file allocation must not be presented as deduplicated physical reclaim.

/api/space explicitly probes enrolled live volume capacity, separate from cache
allocation. Retained reports admit at most 64 MiB encoded / 512 MiB decoded,
500,000 objects, 16 places/copies, 64 owner rules and 65,536 refs/object. Filters
are bounded; accounting overflow refuses. No fresh census runs on startup.

Historical single-cache review estimates exclude known selected subjects,
current selections, pins/leases, metadata, unknown links/allocation, aliases
and missing independently identified copies. They are candidates to recheck,
never safe reclaimable bytes or removal authority. Fresh registered observations
cannot establish custody by file presence/size. No eviction endpoint is added.

## Storage across projects and drives

Optional enrollment storage_domains names up to 32 explicit capacity domains
(id, label, absolute root, live_probe). The global Storage view lists these
drives and cache roots from every enrolled resolver profile. It is available
independently of the currently selected project. A domain binding is owner
configuration, not proof that distinct roots or volumes are independent.

/api/storage/global accepts project-reference and domain filters and 100-copy
paging. It reads captured registry metadata and loaded observations; it does
not scan caches. Explicit measure=true probes allowed volume capacity only.
Identical normalized root/layout/object locators are counted once across projects.
Different roots can alias physical bytes; file allocation remains qualified.
Project reference totals overlap. OS volume usage is never summed with or
subtracted from logical cache-file totals. Missing observations stay unknown.
Limits are 500,000 registered objects and 1,000,000 observed copy locators;
overflow/filter/budget errors refuse. Root namespaces remain project independent.

A storage profile may omit inventory for registered-only projects. Its explicit
refresh uses existing byte locations, including standard Git LFS. An asset-only
projection covers records not grouped by creative-work rules. It changes no
authoring IDs, definitions or deliverable counters. Immutable observations are
retained in memory by source kind/family; changing one family does not erase
other loaded scopes. Restart does not persist these observations.

/api/storage/scope reuses a matching captured registered-family observation at
the same Git commit, or explicitly probes its named registered objects. The
family control clears dependent work/version filters, broadens the underlying
observation when necessary, shows progress and restores the prior view on refusal.
All families therefore selects the all-registered scope rather than merely
clearing a filter over journey-only data. Observation time remains visible.
Equal-height ranked labels precede a compact proportional map so smaller groups
remain readable without inflating their byte-area representation.

## Named objects and placement review

The primary management view is one immutable byte object per row, with named
direct asset/stream references, declared revisions, project placement preferences,
observed cache copies and review flags. Project-dashboard actions and sidebar
navigation preserve project context. Capacity/location summaries are secondary.
No catalog work-membership grouping is treated as a complete storage footprint.

/api/storage/objects accepts project, kind, usage, anomaly, primary-cache ID,
observed-cache ID, text/hash search and content/stored/name sorting. Pages contain
100 unique byte objects. Selecting an observed cache filters objects and counts
copy bytes for that cache, while exposing their other copies for comparison.
Each row exposes up to six bindings, or 100 for exact-object detail, with the
actual binding count; truncation never implies complete consumer coverage.
Current/previous revision interpretations reuse the explicitly admitted root
version role, slot prefix and exact selected-object/version correspondence.
Other selections stay unclassified for use. A nonselected revision is a review candidate, not proof that it expired or is not an authorized draft. Missing current-version or slot correspondence stays unclassified. Direct references are not ref-count
or production/dependency closure authority. Unobserved locations stay unknown.

Optional project placement_rules declare id/label, anchored asset-ID pattern,
usages, place_index and priority (0..1000). A field/field_pattern pair optionally
matches an admitted authored/status string; private metadata is excluded. Regex
and rule budgets match existing catalog bounds (64 rules, 512-byte patterns,
1 MiB compilation). Cache targets must be enrolled. Highest priority wins per
binding; ties with different targets flag a conflict. Shared binding preferences
remain visible. A current revision suppresses that same project's historical
primary preference for unchanged bytes, while preserving all historical refs.
Preferences do not establish approval, demand, custody or deletion authority.

/api/storage/preview binds an exact object to captured Git snapshots, policy
fingerprints and copy observations. Stale bases refuse. Alignment and older-copy
review show the intended checks and holds; no executor or mutation is exposed.
/api/storage/verify-copy separately performs an explicit guarded existing-byte
read for one enrolled live cache. It releases the catalog metadata mutex before
hashing and inherits provider limits/protection. A successful point-in-time check
does not acknowledge independent durable custody or authorize eviction.

Acceptance tasks: find a named large object in its project; compare preferred
and observed caches; inspect all exposed refs/revisions and coverage gaps;
preview alignment/old-copy review; verify an exact retained copy without
changing bytes. Synthetic shared-current/history and stale/corrupt-copy cases
precede browser task qualification. No move, upload, cache census or deletion.

Batch review preserves up to 50 selected object identities across filters/pages,
shows hidden-selection counts and clears selection when changing project scope.
/api/storage/batch-preview accepts project, selection (comma-separated sha:basis
pairs, at most 6500 bytes), observed-cache ID and align/evict-review action. It
captures one inventory and refuses the whole response on duplicate, missing or
stale identities. Cache scope restricts older-copy candidates and alignment
sources. Totals separate unique content, proposed copy bytes and older-copy file
bytes; physical reclaim stays unknown. Filters select review context, never
retention authority. No batch executes transfers or removals.

Work inspection links directly to object management with project, collection and
work IDs. /api/storage/objects optionally accepts collection/work together in a
named project. Membership comes from that exact snapshot's admitted catalog
rules, including the root. Unknown/partial scope refuses. A matching object's
other bindings and placement preferences remain visible across works/projects.
Scope is registered related records, never inferred dependency closure.
copy_summary counts all observed copies of all matching objects before pagination,
per cache; an observed-cache filter chooses the matching objects, while this
summary still shows their other copies. These file bytes can overlap physical
aliases and do not establish reclaimable allocation or verified custody.
The work view explicitly warns that manifest documents count as documents;
nested file references are not expanded. Different registration coverage means
these totals must not be presented as comparable complete work footprints.

## Explicit nested metadata adapters

Optional storage.nested_content enrolls bounded JSON adapters, not directory or
URI crawlers. No document read occurs on startup, snapshot change or object-list
requests. The work view's explicit Expand/Refresh nested metadata action calls
/api/storage/expand with project, exact snapshot, collection and work IDs.

The profile sets max_documents (1..2048), max_document_bytes (1..4 MiB),
max_total_bytes (document limit..64 MiB), max_references (1..100000), max_depth
(1..16), and up to 32 adapters. Each adapter has a unique id, exact schema string,
root rules (definition ID plus anchored asset_pattern/slot_pattern), objects
selectors and optional documents selectors naming a child adapter. The root slot
`selected` means the exact native selected object; other slots match native stream
names. Ambiguous root adapters refuse before IO. Only enrolled live byte places
are admitted; original reader limits are narrowed, never enlarged automatically.

Selectors have pointer and optional (default false). JSON Pointer tokens support
explicit `*` traversal of arrays/map values; literal slash/tilde keys use ~1/~0.
Leaves must contain exact lowercase sha256 and unsigned bytes. Other fields,
including cache_uri, URLs, prompts and unselected hashes, are ignored. A documents
entry contains selector and adapter; it names a verified metadata document to
recurse into, never an arbitrary file locator. Ordinary leaf payloads are not read.

JSON reads are hash/length verified and guarded by the existing ReadCache. JSON
duplicate keys refuse; unselected floating-point fields are allowed. Documents
cache once per operation, with a 500000-node parsing bound, selector-step budget
and 60-second traversal check. Attempts and bytes are charged even on failures.
Malformed/missing/schema-refused roots contribute no partial associations and are
reported unknown; hard budgets refuse the whole adoption. An incomplete refresh
preserves a previously loaded scope. Results are adopted only if snapshot, roots
and profile pins still match; reads happen outside the catalog metadata mutex.

Associations retain native asset/root slot, root object, document object, adapter
and descriptor pointer. Scope results and object inspection expose this provenance.
Exact hashes deduplicate bytes; all root associations remain visible and inherit
the root's declared use/preference without claiming producer dependency semantics.
Loaded-content pins invalidate old previews even when Git is unchanged. Explicit
named-copy refresh includes discovered objects; presence is not verified custody.
Loaded scopes are in-memory (max_loaded_scopes defaults to 16, configurable 1..256
per project, with 500000 associations total/project), cleared
when the project snapshot changes. No expanded inventories are written to Git.
Adapter success establishes the configured paths only: complete production graph,
unconfigured documents, active jobs, durable custody and removal authority remain
unknown. The historical unexpanded warning above applies until adapters are loaded.

Example adapter (inside nested_content.adapters, with the budgets above):

```json
{
  "id": "bundle", "schema": "example.bundle.v1",
  "roots": [{"definition": "type/bundle", "asset_pattern": "^bundle/", "slot_pattern": "^selected$"}],
  "objects": [{"pointer": "/assets/*"}],
  "documents": [{"selector": {"pointer": "/child", "optional": true}, "adapter": "bundle"}]
}
```

Each assets map value is a hash/size descriptor. The optional child descriptor
names another document with the admitted schema; ordinary assets are counted but
never opened. Adapters may select other explicit array/map paths and child schemas.

Work scope responses include nested_roots: null when unconfigured, zero for no
matching definition/asset/slot roots, or the admitted count. Counting roots reads
captured native metadata only. Eligibility and expansion share the same compiled
selector path; the UI hides expansion for a work with no matching roots. Neither
absence of a matching adapter nor successful expansion proves dependency closure.

## Coverage and version review

/api/storage/coverage reports top-level registered works across enrolled projects.
Filters: project, collection, state, q and offset (100 rows/page). State counters
precede the status filter. Expanded means configured paths expanded, not complete
dependencies; partial, ready-to-expand, no adapter, no matching roots and
configuration-error remain distinct. This report reads captured metadata only.
Pagination carries a coverage basis; changed loaded state refuses mixed pages.
Project names, groupings, definitions and storage mappings are external enrollment.

/api/storage/compare takes project, snapshot, collection, work, from and to. Both
slots must belong to the root record's project-admitted collection version_role;
arbitrary streams, evidence and client-provided hashes cannot substitute. A root
matching its metadata adapter can explicitly compare those version documents
under that same schema/definition admission; ordinary work expansion still uses
its configured slot selectors. Hash/length/schema and existing read budgets apply.
Without an admitted root adapter only the directly declared version objects are
compared, without payload reads. Missing metadata remains visible and incomplete.

Comparison returns added/shared/removed known declaration sets, byte totals and
the largest 100 objects/set. Metadata documents themselves count as stored bytes.
Equal hashes with conflicting sizes refuse. Configured-path completeness is
separate from full dependency closure; partial set differences are inconclusive
about unexpanded content. Metadata IO occurs outside the catalog mutex, with
snapshot refusal on return. Results carry source pins and a deterministic basis.

Removed objects expose known records outside the compared root and loaded cache
observations. Those references include history and are not an authoritative
current refcount; unexpanded consumers, other retained versions of the same root
and active jobs remain unknown. Zero loaded references is not zero demand.
Observed copies are not independent custody or measured reclaim. Comparison
adopts no expanded state and never transfers or deletes bytes; executable and
removal_authorized remain false, safe_reclaimable_bytes is null.

Deployment-specific readiness history and private qualification belong with the
consumer owner. Generic gates are in [consumer adoption](../docs/CONSUMER-ADOPTION.md).

Storage work grouping applies family and work filters to the same membership
pair. Nested objects inherit the admitted work memberships of their referencing
records. Shared bytes retain multiple owners; membership grants no lifecycle
approval or removal authority.

The object References & versions tab offers explicit Open, Compare and Impact
shortcuts to each declaring owner at the captured project snapshot. Retained
slots and nested document/pointer associations remain evidence; opening the owner
does not resolve a historical record or establish current production use. Missing
owners and unindexed consumers remain unknown. Sampled bindings are disclosed.

Navigation checks the captured owner snapshot and returned record identity.
Back to object restores the references tab and captured scope. Each object load
has a unique selection token, clears previous actions while pending and checks
returned hash, length and project snapshots before exposing actions. Empty,
failed and changed-source responses offer explicit recovery. Late object or
held-review responses cannot replace another inspector or tab. Reviews use the
captured scope; none of these shortcuts grants movement or removal authority.
