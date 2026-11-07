# Explicit project host and space preflight

Entry `93e57ee8c4eb15b0f0ea7fdc47cb83b094d76874`. WP-005/006/011/015,
REQ-001/003/004/006/007/010, SPEC-004/006/007/011/012/015,
IF-005/008/010/012/013, PKG-003/004/008. RUST + PROVIDER + INTEGRATION profiles.
Scope: optional explicitly declared host-file identity checks and advisory space
observations before project copying, using existing restore checks and fs2.
No tool execution/discovery, new cache, media interpretation, capacity reservation,
quota, production adoption or removal. L0 format/foundation/lint; L1 bounded
declaration/error/space arithmetic; L2 synthetic Git profiles, missing/wrong host,
view/render separation, unavailable probe and no-write capacity refusal. Exit:
tested scope, producer review and exact-source hosted qualification kept separate.

Optional committed manifest `requirements` contains `host_files` (up to sixteen
unique IDs with exact Object pins) and optional `storage`. Local profile
`host_files` maps IDs to explicit absolute paths; unused bindings are not read.
A view manifest need not declare render-only files. No PATH/font/system discovery
or executable invocation is allowed. Requirements bind to the existing exact
manifest/plan; local path changes alter the plan, not immutable object identity.

Host verification uses the existing ordinary-parent/file integrity checks,
bounded to 256 MiB per file and 512 MiB total. Local limit
`limits.host_verification_bytes` defaults to 64 MiB and cannot exceed 512 MiB.
Unbound, missing or differing/unreadable files report distinct outcomes; all
declared host results are reported before source verification or mkdir. These
are observations, not guards retained through a later render or license checks.

Storage declares `minimum_available_bytes` as an observed free-space threshold
and `scratch_estimate_bytes` as an advisory consumer estimate. Local profile
`capacity_root` explicitly identifies an existing ordinary ancestor of the
destination root; no provider/drive discovery or fallback from an unavailable
enrolled root. Scoped ancestor checks find the deepest existing destination
directory. Unix device identity must match the enrolled root; Windows ordinary
parent checks refuse mount/junction reparses. The existing fs2 sampler reports
user-available bytes on that checked destination ancestry. An unexpected
filesystem refuses instead of sampling another drive and claiming sufficiency.
Missing/offline/unrelated/link roots refuse before copying, without initialization.

Report separately: new logical destination bytes after checking existing files,
the declared threshold, scratch estimate, observed available bytes, logical
restore sufficiency and estimated-peak sufficiency. Restore requires the observed
space to cover max(new logical destination bytes, declared threshold). An estimate
shortfall is a warning observation, not a secretly mandatory requirement. Checked
arithmetic refuses overflow. Existing verified targets require no new logical
copy space. Allocated blocks/metadata and other writers can change real usage.

No probe grants capacity admission or enforces quota; observation sufficiency is
not a guarantee of completion. The caller still owns admitted copy/render budget
and actual scratch. Apply repeats the checks on every explicit invocation; an
earlier preview never holds a capacity reservation. Reports keep protection,
custody, semantic completeness and eviction authority separate. Host file paths
remain local and are omitted from fixed errors and results. Operational reports
stay asset-side. The project result schema advances to experimental-3 for these
additional observations; old manifests/profiles without requirements still work.

## Small declaration example

An illustrative committed manifest extension (the hash is a placeholder identity,
not a supplied font or license):

```json
"requirements": {
  "host_files": [{
    "id": "fonts/body",
    "object": {"sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "bytes": 1045956}
  }],
  "storage": {
    "minimum_available_bytes": 1073741824,
    "scratch_estimate_bytes": 536870912
  }
}
```

The local profile provides `host_files: {"fonts/body": "/owned/fonts/body.ttf"}`
and `capacity_root: "/owned/runs"`, using platform-appropriate absolute paths;
`destination_root` is at or below the enrolled runs root. These directories/files
must already exist where required. No path is inferred from the requirement ID.
Input numbers retain Tig's existing strict safe-integer JSON rules. Tool/version
semantics, font licenses and render capacity admission stay consumer-owned.
