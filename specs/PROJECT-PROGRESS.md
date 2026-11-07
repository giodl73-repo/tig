# Explicit project restore progress evidence

Separate [partial inspection](PROJECT-PARTIALS.md) can explicitly list bounded
private-name candidates in planned destination parents. It does not read a log,
verify payloads, infer ownership or authorize cleanup.

Entry source `0e53ee83c963062bd2f8f773ed72860ca94a0b94`, qualified with the
identical whole tree on main `7f795f8eb99de4c8b512e0cff084ec8fd5f9ffc5`.
WP-005/011/015, REQ-001/003/006/010, SPEC-004/007/011/012/015,
IF-005/008/010/013, PKG-003/004/008. RUST + INTEGRATION profiles.
Allowed scope: explicit local bounded operation evidence over the existing
project plan/copy/observer, not a new transfer engine or persistent cache journal.
L0 format/foundation/lint; L1 malformed/bounded/path/identity tests; L2 synthetic
real Git, child exit after synced entry, explicit retry and actual CLI tests.
Exit requires scoped passing evidence and separate producer review/hosted gates.

```text
# Preview as before; retain its exact plan hash.
tig-project local-profile.json profiles/view.json

# An explicit apply creates a fresh private evidence file, never reuses one.
tig-project --apply --expect-plan SHA256 --progress ABSOLUTE_NEW_LOG local-profile.json profiles/view.json

# Inspect historical progress only; no payload verification or copying.
tig-project --expect-plan SHA256 --inspect-progress ABSOLUTE_LOG local-profile.json profiles/view.json
```

Progress flags precede positional arguments. `--progress` requires explicit apply
and expected plan; inspection requires expected plan and forbids apply. The old
preview/apply route remains unchanged. Inspection of interrupted evidence is a
successful inspection, not a claim that the restore completed successfully.

## File and identity contract

The caller supplies an absolute new ordinary file under existing ordinary
parents, outside the restore destination, source checkout and enrolled source
caches. Link/reparse parents refuse. Exclusive create prevents overwrite or
concurrent reuse of the same file; unrelated attempts need no global lock. Unix
creates mode 0600; Windows inherits caller-controlled parent ACLs and allows
read sharing while refusing write/delete sharing for the writer's lifetime.
The namespace is trusted local storage, not a hostile-directory-swap sandbox.
No directory or cache is initialized for progress. File creation and every
row are explicit operational writes, separate from no-write preview.

The experimental `tig.project-progress.experimental-1` format is bounded JSONL:
at most 4 MiB total and 16 KiB per row, covering at most the admitted 1,024
entries plus header and terminal event. Duplicate keys/unknown fields refuse.
The header binds exact plan, source commit/checkpoint, raw profile/manifest
objects and entry count. A random attempt identity distinguishes invocations.
Plan identity already includes manifest path, definitions, selected records,
selectors, destinations, provider profile and budgets. Moving machine paths can
change the plan without changing object identity.

Each completed row binds an in-order ordinal, record reference, selected Object,
action and verified-destination observation. Only `restored` and
`already-present` completed actions are admitted. The writer syncs a completed
row after successful verified copy/reuse and before starting the next entry.
A trusted Rust observer runs after that sync, allowing deliberate operator stop
or consumer-owned handoff; no manifest-loaded executable hooks are introduced.
The terminal event reports success/failure and logged completed count, not
custody. CLI stdout retains the full existing phase/result report.

The writer stops on evidence write/sync failure, never appending after a possibly
torn row. Earlier verified copies remain. No file rollback, automatic deletion,
resume-on-startup, mutable latest pointer or implicit log registration occurs.
Detailed logs contain private record identities and belong asset-side, outside
ordinary Git/public logs; they can be enrolled through existing asset mechanisms.

## Interruption and interpretation

A crash can occur after copying but before logging. Therefore **not logged
completed does not mean missing**. An explicit retry always rechecks actual
destinations and source bytes with the unchanged guarded restore primitive.
Use the same exact source/profile/manifest and expected plan with a fresh log;
changed context refuses the earlier plan. Never replay commands from a log.

Inspection binds the header to the prepared plan and validates every complete
row against the planned in-order record/Object. A final non-newline tail is
reported incomplete and never counted as completion. A torn/missing header,
malformed complete row, duplicate/reordered entry, changed identity, excessive
size or data following a terminal event refuses. A header-only log leaves every
entry unlogged; no terminal event means terminal success is unknown. The summary
pins the exact bytes observed, including any incomplete tail. This is an observed
file snapshot, not cryptographic authentication of an operator's claims.

Inspection does not examine destination payloads or orphan partials and cannot
establish their present state. The source guards are no longer held after copy;
independent custody and eviction authority remain false. `sync_all` is requested
per acknowledged row, but no portable directory/power-loss durability or
multi-file atomicity promise is made. Orphan discovery/classification, complete
runtime/environment handoff, consumer host requirements and live adoption remain
separate work. A log is evidence, never an asset-selection or deletion authority.
