# Linked project handoff evidence

Entry `a35f48fb12eeedea36c430cb226f01a59762cf9d`; WP-005/011/015/016,
REQ-001/003/006/010, SPEC-004/007/011/012/015, IF-005/008/010/013,
PKG-003/004/008; RUST + INTEGRATION. Scope: bounded consumer-supplied evidence
links and explicit selector/destination reporting over the existing project
plan/result. No new receipt engine, payload list, provider IO, runtime install,
approval, retention or deletion. L0 format/foundation/lint; L1 strict admission
and plan identity; L2 real temporary Git/cache/CLI, aliases, frozen non-HEAD
source and interrupted/failed results. Exit requires local tests, separate
producer review and exact-source hosted qualification before consumer adoption.

The ignored local project profile may supply `consumer_evidence`, a map of at
most sixteen labels to exact Object pins. Labels are portable relative paths
of at most 128 UTF-8 bytes. Duplicate keys, malformed objects and unknown object
fields refuse under the existing strict reader. An omitted field means no links.
Pins may refer to large external runtime, recipe, environment or review records;
the map does not duplicate their contents or create a second asset selection list.

```json
"consumer_evidence": {
  "runtime/build": {
    "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "bytes": 1234
  }
}
```

These are declared links, not verified evidence. Tig does not resolve, open,
execute or interpret them. Missing local evidence bytes do not fail restoration;
consumers requiring those checks must perform them and retain their own results.
Labels carry no built-in authority: a link named approval or custody grants
neither. Evidence is not added to requested objects, byte budgets, dependency
closures or protection claims. Object identity remains independent of machine
paths and evidence labels. Exact raw profile identity already binds the links
to the plan; changing them refuses a previously expected plan before copying.

The result schema advances to `tig.project-restore-result.experimental-4`.
Each existing entry outcome now includes its explicit selector and relative
destination. Two destinations or two selectors can share the same byte object
while remaining distinct operations. The report retains original source commit,
checkpoint, manifest/profile pins, plan, phases/timings, unique/destination/copied
byte counts, verified outcomes and observed protection. It also echoes the
declared evidence map with `consumer_evidence_verified: false`, including on
structured preflight failures and interruption. It never asserts a successful
full batch solely from the presence of links. Old profiles/manifests still work.

Detailed reports contain private identities and destinations; keep them local
asset-side, outside Git/public logs. Existing stdout can be retained and enrolled
through the consumer's asset publication route. This extension does not itself
persist, authenticate, register or retain a report. Exact source pins refer to
the actual run, which can differ from checkout HEAD; later documentation or
comparison revisions cannot retroactively qualify that run. Existing opt-in
synced progress remains the interruption evidence mechanism, not automatic resume.

Source verification, destination readback, observed protection and independently
retained custody stay separate. Source guards are released at return. No complete
environment/recipe semantics, multi-file atomicity, orphan classification,
independent custody or eviction authority follows from a handoff report.
