# Verified-cache Git proposal CLI

Experimental explicit commands, without installation or repository setup:

```text
tig-propose check profile.json     verify records and exact-base preflight
tig-propose create profile.json    repeat checks, publish separate proposal commit/ref
git show <proposal-reference>      review with ordinary Git
git cherry-pick <proposal-commit>  separate application/conflict handling
```

No default command performs writes. `check` reads existing cache/Git objects but
writes no Git objects, refs, index or working files. `create` consumes the same
`Repository::check_proposal` capability to publish. Saved JSON is not a capability;
a new CLI invocation checks again. Both leave the current branch untouched.

The strict private profile is at most64KiB. Replace placeholders and absolute paths
with actual enrolled values (Windows drive paths are supported):

```json
{
  "schema": "tig.git-proposal-profile.experimental-1",
  "git_executable": "/usr/bin/git",
  "repository": "/workspace/example",
  "base": "<exact Git commit ID>",
  "identity": {
    "name": "Example Creator",
    "email": "creator@example.invalid",
    "unix_seconds": 1700000000
  },
  "message": "Replace the corrected cel record",
  "byte_source": {
    "places": [{ "root": "/mnt/existing-cache", "layout": "flat" }],
    "required": "cooperative-immutable"
  },
  "changes": [{
    "path": "records/scene/01/cel-b.tig",
    "expected": { "sha256": "<old record raw SHA-256>", "bytes": 512 },
    "object": { "sha256": "<candidate record raw SHA-256>", "bytes": 530 }
  }]
}
```

Unknown fields/duplicate JSON keys refuse. Use `prevent-mutation` for strict
Windows sharing. Other platforms explicitly admit `cooperative-immutable`, which
requires cooperating writers. It is not mandatory mutation prevention; see
[existing byte reads](EXISTING-BYTE-READ.md) for the unchanged protection/limits.

`expected` is old raw Git-file content, not a Git blob ID. `object` is the exact
new small-record content already in an enrolled cache. `flat`, `reel-cas` and
`git-lfs` layouts and ordered roots reuse ReadCache. Missing roots are not created;
missing objects can fall through, corrupt present objects refuse. No unpinned
filename substitute, scan, fetch, media staging or cache initialization occurs.

There are1..128 unique target paths and at most64KiB old/new bytes per file.
Shared new objects are verified once, including reader/request accounting; actual
buffers must match the pins and guards remain held through the response. Caller
byte-source budgets may refuse more strictly. The library Git preflight/create
deadline is separate from bounded byte-source verification. Ancestor/identity/
message/command limits are in [proposal publication](GIT-RECORD-PROPOSALS.md).

Success emits private `tig.git-proposal-result.experimental-1` JSON: action, base,
change pins, verified-object count and proposal commit/ref (`null` for check).
Both modes report `current_selection: false`, `complete_semantic_coverage: false`
and `payload_custody: false`. Failure exits2 with a static/category error and no
success JSON. A lost create reply can retry identical identity/time/message and
pins; the existing commit/ref is reused. Partial Git publication can leave the
small recoverable metadata described by the library contract.

Keep generated profiles/results asset-side; they contain private paths/hashes.
Generic byte verification is not native schema, producer-claim, approval or build-
freshness validation. Native adapters/gates still supply those checks. Ordinary Git
application does not adopt source/job obligations or authorize payload eviction.
