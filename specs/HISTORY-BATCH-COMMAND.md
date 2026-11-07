# History changes across one captured source

Input `89606502149bace28a6bd55e2daa73285afca7b4`, extending
[the single-asset service](HISTORY-COMMAND.md) and its
[host seam](HOSTED-HISTORY-COMMAND.md). Controls, review and qualification are in
[the entry](../docs/reviews/history-multi-entry.md).

Commands: `tig-history batch-check|batch-publish|batch-propose PROFILE REQUEST`.
The same application service is available to admitted compiled provider hosts.
No host upgrade, store initialization or operation runs automatically.

Profile schema `tig.history-batch-profile.experimental-1` contains the existing
git_executable/repository/base/records/definitions/dialect/optional capture limits,
byte_source and publication enrollment. Optional proposal contains identity and
message, without a single path. Required source base is an exact Git commit.
Request schema `tig.history-batch-request.experimental-1` contains items; each has
asset, path, expected raw record Object and request (one existing operation or
ordered sequence). Relative change-document paths resolve against this explicit
request file. Generated requests/profiles/reports belong outside ordinary Git.

The batch contains one to64 distinct IDs and paths, at most256 operations. All
paths must be inside declared record scope and bind their exact expected bytes
at the captured source, including publish-only operations. Duplicate IDs/paths,
swapped paths, missing records, unknown schemas and stale source pins refuse.
No inference from equal payload hashes and no implicit current-revision choice.

Each input file remains bounded to64KiB. With at most256 operations, explicit
change documents occupy at most16MiB. Capture is clamped to16MiB/16,384 documents;
existing-reader acquisition is clamped to64MiB/4,096 requests and16MiB/object,
honoring any smaller caller budgets. Prepared metadata plus record buffers charge
their actual serialized lengths/counts, including duplicates, against64MiB/4,096
blobs. Original per-edit and sequence limits still apply. Proposal copies add at
most64 records of64KiB each. Limits are safety bounds, not storage estimates.

```text
one exact Git listing/snapshot
  -> load and validate every declared asset's ordered edits in memory
  -> check aggregate budgets and complete sparse Git write set
  -> open the admitted backend and preflight ALL item capabilities
  -> publish existing immutable intents/objects for changed items
  -> independently read back every new object and final record
  -> create ONE immutable Git proposal (explicit caller applies it)
```

Check and all-noop batches never construct a publisher. An all-noop batch preserves
original raw records and creates no proposal. Item keys derive from the explicit
batch operation namespace and stable asset ID, not array ordinal. Reordering an
exact retry preserves publication identities and the Git proposal; changed reuse
cannot silently borrow an earlier operation acknowledgement.

All candidates and Git source/path/identity preflights precede provider opening.
All declared capabilities are checked before the first intent write. Capacity
admission, IO, later readback or Git creation may still fail after earlier objects
are acknowledged. Retain those objects and old source protection; return an error
without a success report or proposal. Exact retry recovers through existing
publication operations. This is no atomic commit across Git and byte storage.

Results contain the pinned base/source checkpoint, ordinal item pins, no-op and
publication/readback observations, and optional proposal identity. Do not emit
asset names, target locators, authored bodies, provider settings or credentials.
Approval, selected-current authority, old-protection release, independent custody
and complete semantic coverage remain false. A batched selection is still an
explicit caller-supplied domain status decision, not human approval.

Synthetic acceptance includes two assets in one proposal, reordering/replay,
late invalid candidate without provider construction, missing/stale/swapped paths,
bounded input, later-object capability refusal before any write, partial publish
and retained-ack retry, wrong readback place and corrected retry, and the full64
item boundary. Actual family conversion, historical source closure adapters,
runtime adoption, storage handoff and reclamation remain separate qualifications.
