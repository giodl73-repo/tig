# Updating one versioned asset

Build `tig-history` with the optional `schema` feature. This command uses existing
LocalStore enrollment; it does not initialize a store or switch a live runtime.
Other provider hosts reuse the same Rust preparation/publication APIs.

```text
one stable asset file in Git
       |
exact base + expected record + pinned raw change document
       |
tig-history check profile.json request.json       reads only
       |
tig-history publish profile.json request.json     acknowledges new metadata
       |
tig-history propose profile.json request.json     also creates a Git proposal
       |
git show <proposal-reference>                     review
git cherry-pick <proposal-commit>                 explicit application
       |
separate source/claim reconciliation              old protection still required
```

`propose` repeats validation and publication; running `publish` first is optional.
Identical retries reuse acknowledgements and proposal identity. A changed request
needs a new operation key; reusing the old key for a different candidate refuses.
Changing the branch's HEAD does not replace the profile's explicit immutable base.
Ordinary Git application handles later independent edits or stops for conflicts.

The [command contract](../../specs/HISTORY-COMMAND.md) lists exact profile fields.
Use a private profile with an exact base, asset ID, expected raw record Object,
chosen dialect, bounded record/definition scopes and admitted byte-source places.
The already-enrolled publication root must also be explicitly listed as a Flat
reader place. Proposal configuration supplies the path, author identity and message.
When present, that path selects the exact native file inside the record scope.

To register a revision, the request has operation `register-revision`, coverage
`complete` or `partial`, and document `{path,expected}`. The expected Object pins
the original raw manifest bytes; a relative path is relative to the request file.
Registration adds a small history page and manifest without changing selection or
approval. Configure-delivery and record-delivery use the same pinned-document
form. Select-revision supplies a retained revision, representation, exact subject,
the complete desired status map and explicit stream names to follow selection.

Output contains hashes/counts and, after proposal creation, its Git reference.
It omits authored bodies, destination configuration and local provider paths.
Saved check output is not permission to publish: each invocation checks again.
An intent object alone is not proof that a batch completed. A no-op writes no
objects or proposal. Missing/corrupt/busy history refuses without rebuilding it.

These operations preserve old history; they do not verify all old bytes, establish
independent custody, upload a delivery, apply Git, grant approval or release a job.
REEL/consumer project runtime integration and acknowledged claim handoff remain separate
acceptance steps. Keep prior protections until those steps finish.
