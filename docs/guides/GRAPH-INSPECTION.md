# Inspect declared dependencies and compare byte demand

Build the experimental read-only command with:

```text
cargo build --locked -p tig-git --features assets --bin tig-graph
tig-graph inspect PRIVATE_PROFILE NEW_PRIVATE_REPORT
tig-graph compare BEFORE_PROFILE AFTER_PROFILE NEW_PRIVATE_REPORT
```

Inspection and comparison configure no payload provider. They capture committed
record/definition/schema metadata through the existing local Git adapter. They
never change Git refs, authoring records, caches, claims or installed hooks.
The same graph resolver/production requirements/primary comparison APIs are used
by Rust callers. It does not implement a separate dependency engine.

## Verify the selected input objects in existing stores

```text
tig-graph verify GRAPH_PROFILE PRIVATE_BYTE_PROFILE NEW_PRIVATE_REPORT
```

This explicit mode captures Git once, extracts complete Production requirements,
and uses the existing verified reader for deduplicated objects. It never creates,
fills, repairs or removes a cache. Output-only, incomplete and non-Production
queries refuse rather than report an empty successful input check. Delivery
output lookup remains a separate asset-ID/stream operation.

The strict, at-most-64-KiB byte profile contains:

| Field | Meaning |
| --- | --- |
| `schema` | `tig.graph-byte-verification-profile.experimental-1` |
| `graph_profile` | Exact SHA-256/size Object of the graph profile's raw bytes |
| `selection` | `primary` or `primary-and-controls`; Git interpretation documents are already bound metadata, not cache payloads |
| `byte_source` | Existing ordered provider places/layouts, required protection and explicit reader limits |

The whole object count, maximum object size, total declared bytes and bounded
private report are admitted before payload IO. ReadCache keeps its original
platform/protection rules and refusal semantics. Missing, busy, denied and corrupt
observations stay distinct; present corruption is never hidden by a later valid
copy. History/evidence do not become current inputs merely by appearing in a
record. Each immutable object is checked once even when several IDs share it.

```text
exact Git source -> one captured graph -> admitted object set
                                              |
                                 open / hash / release one object
                                              |
                                    bounded private observations
```

Reports identify each selected Object, outcome, successful provider index and
effective protection during its read. No physical provider paths or source text
are added. Stdout contains counts/booleans only. Guards are released between
objects and before publication; this is not a simultaneous snapshot or retained
protection. `selected_objects_verified` applies only to the explicit selection;
`complete_declared_payload_verification` also requires primary-and-controls scope.
Neither field proves schema/semantic execution, producer currency, independent
custody, all physical replicas, global protection or eviction authority. Logical
verified bytes are not allocated disk space or predicted reclaim.

## Graph inspection profile

The profile uses schema `tig.graph-inspection-profile.experimental-1` and fields:

| Field | Meaning |
| --- | --- |
| `git_executable`, `repository`, `commit` | Explicit absolute executable/repository and exact full commit ID |
| `records`, `definitions` | Bounded logical Git scopes, as for the native resolver |
| `limits` | Optional existing native capture limits |
| `roots` | 1–128 unique current logical asset IDs in that exact snapshot |
| `kinds` | Requested edge kinds, e.g. `["production"]` |
| `profiles` | Exact caller-admitted graph interpretation for each relevant definition |
| `query_limits` | Explicit `nodes`, `edges`, `depth`, `bytes`, under existing graph ceilings |
| `max_report_bytes` | Positive private report allowance, at most 16 MiB |

Each interpretation admits `definition_id`, exact definition Object, `class`,
`covered` kinds, `rules` and `payloads`. Classification comes from this trusted
caller policy, never an instance's own status. Existing serialized API spelling
is preserved: classes/kinds/collections are lowercase; binding and payload
variants use their Rust names. For a primary text definition:

```json
{
  "definition_id": "project/text",
  "definition": {"sha256": "<exact definition digest>", "bytes": 512},
  "class": "primary",
  "covered": ["production"],
  "rules": [{
    "pointer": "/translation_sources",
    "kind": "production",
    "collection": "array",
    "optional": false,
    "binding": "SnapshotSelected"
  }],
  "payloads": "SelectedObject"
}
```

Replace placeholders with actual admitted definition bytes. An original text
can declare an empty array; its translation declares source-ID references in
that array. Null is not an empty dependency list. A container can use
`ObjectValues` via collection `object_values` over a reference map and payloads
`None`. Those policies do not automatically admit other reference fields.
Output definitions must have no production payload selection; a production
edge targeting an output is a gap. History streams are excluded when only the
current selected object is admitted. Additional evidence/authority/composition/
publication relationships remain separately admitted kinds.

```text
commit A: English --production--> Spanish v1
commit B: English --production--> Spanish v2  (only Spanish record changed)

compare(A, B):
  added                     Spanish v2 bytes
  shared                    unchanged English bytes
  unreferenced_by_new_query  Spanish v1 bytes
```

The old commit still resolves v1. Another branch/root may still require those
bytes. The comparison is scoped to two admitted graphs: it is neither global
ref counting, a freshness/approval verdict, a physical-space measurement nor
permission to evict. Build currency requires producer/consumption evidence;
storage removal also needs complete current claims, verified retained custody
and fresh fences. All claim/eviction-authority flags remain false.

Reports contain exact source/profile/binding identities, roots, classified nodes,
edges and gaps, without authored text, status or private metadata values. With
complete Production-only coverage they also separate primary bytes, control
bytes and interpretation metadata. Exact definition/schema pins are bound;
authored JSON Schemas are not executed here (`authored_schemas_validated:false`).
Run the project's schema/semantic/eligibility gates separately.

Missing children, malformed references, unsupported meaning and exceeded query
bounds remain explicit gaps. Inspect writes that partial report and returns
failure. Missing roots or invalid profile/capture fail before a graph report can
be formed. Comparison returns failure with `change:null` when coverage,
requirements or interpretation compatibility is insufficient; no empty successful
candidate list is fabricated. Non-Production inspection is supported, but only
Production-only graphs can produce executable input requirements/comparisons.

Stdout contains counts and booleans only. Detailed IDs, hashes and structural
pointers belong in the bounded private report; keep profiles/reports outside
ordinary Git (for example private runtime storage or ignored `.local/`). The
output must be new and absolute with an existing trusted parent. Serialization
stops at its byte budget before creating a temporary file; publication uses a
synced owned temporary plus same-parent no-overwrite hard link. Unsupported
filesystems refuse. Handled failures clean owned temporary files; abrupt crashes
can leave a bounded private temporary. No power-loss or hostile-filesystem
durability/containment claim is made. Compare uses the smaller report budget of
its two explicit input profiles.
