# Source observation concurrency qualification

Entry: `1dfa6db6199a226661601e2e829dcbd9ad56c2f1`. WP-008/009/015;
REQ-002/004/006/007/009, SPEC-005/006/007/008/014, IF-004/005/006/013,
PKG-003. RUST/PROVIDER/INTEGRATION profiles. Read source-claim and batch-observation
contracts, MODEL/SAFETY and six producer role checklists before changing behavior.

Allowed: bounded source-journal observation/recovery error handling, deterministic
temporary-store regression fixtures and this review. No journal wire changes,
automatic recovery writes, enlarged lock scope, cache operations, relaxed assertions
or new authority. Diagnosis uses synthetic temporary stores only; temporary raw
SQLite diagnostics must be removed before committing.

Observed failures: baseline main run41367150892 Windows and PR67 run41504708240
attempt2 Windows fail `batch_observation_matches_one_checkpoint_while_two_sources_advance`
with `Sql`; attempt1 Linux fails single-source concurrent observation with `Sql`.
The public error hides the underlying SQLite error. Root cause remains unconfirmed
until a diagnostic reproduction identifies the specific path/code; do not broaden
all readonly/SQL errors into Busy or silently recover on a read-only connection.

Exit: identify exact failure, add a regression that fails before the bounded fix,
preserve readonly/no-mutation and immediate-busy semantics, retain corruption and
hot-journal recovery refusals, pass concurrency/checkpoint tests and full required
qualification. No failed/pending CI may be treated as a merge qualification.
L0 validator/diff/fmt; L1 focused error/observation tests and strict workspace lint;
L2 actual temporary journal concurrency and interruption/recovery fixtures.

## Diagnosed condition and bounded fix

Concurrent synthetic stress reproduced SQLite extended code776,
`SQLITE_READONLY_ROLLBACK`, in the failing batch-observation test. This is a
read-only connection encountering a journal requiring rollback, not a successful
read or a general permission error. See the authoritative
[SQLite result-code definition](https://www.sqlite.org/rescode.html#readonly_rollback).
The bundled SQLite pager also returns this exact code after its hot-journal
check on a read-only connection. No claim about data loss follows from this result.

Map only that exact extended code to existing `NeedsRecovery`; preserve ordinary
Busy/Locked handling and other SQL/readonly/corruption refusals. Uncertain handles
stay poisoned until explicit reopen. Concurrent-reader callers may retry a read
after a writer advances; persistent recovery demand needs an explicitly admitted
owner operation. Observation still does not open a writable connection, recover,
mutate the journal, change claims, wait internally or acquire a new global lock.

The concurrent tests now admit the specific no-observation `NeedsRecovery` result
as well as Busy; their successful-snapshot coherence, positive-observation and
final checkpoint assertions remain intact. They do not ignore generic SQL errors.

## Producer review and evidence

Method: manually apply all six local role checklists to the exact mapper,
uncertain-handle condition and source-claims tests. No independent approval.

| Role / finding | Severity | Disposition / owner |
| --- | --- | --- |
| Identity: recovery might invent another authority/checkpoint | High | No identity/wire changes; final recovered witness and original selected root checked |
| Placement: recovery/error labels mistaken for claim withdrawal | High | No claim or placement changes; source obligations remain owner-controlled |
| Custody: readonly inspection silently repairs damaged authority | Critical | Deterministic child-exit hot-journal test requires refusal and byte-identical DB/journal before explicit owner recovery |
| Concurrency: valid recovery refusal collapsed into generic Sql | High | Exact extended-code mapping; no broad readonly whitelist. Concurrent snapshot and final-state assertions retained |
| Provider/privacy: raw SQLite messages expose paths/details | High | Temporary diagnostic removed before commit; public errors remain fixed enums, no raw provider message or diagnostic environment switch |
| Git/adoption: suppress a failing test to qualify unrelated changes | High | Two deterministic regressions fail before fix, pass afterward; separate fix PR required, no repeated CI retries until green |

`readonly_rollback_is_recovery_required_not_generic_sql_or_success` checks eight
specific SQLite codes. `hot_journal_observation_preserves_bytes_until_explicit_owner_recovery`
uses a real child process that spills an uncommitted temporary transaction and
exits. Single/batch observation must refuse without changing either file; explicit
owner reopen restores the original checkpoint/selection and clears rollback state.
Both fail against the unmodified mapper (Sql instead of NeedsRecovery). All35
source-claims tests pass after the fix (12.96s), including both concurrency cases.
Raw diagnostics/regression logs remain outside Git; no live user store was read.

Refresh only the two existing reviewed source/test pins for these actual changes;
no limit, old approval or unrelated source pin changes. Strict lint, structural
checks and full hosted Windows/Linux qualification remain separate recorded gates.
The full asset-revision branch workspace passed locally before incorporating this
fix; that is not exact-source qualification of this fix. Consumer adoption and all
live storage authority remain unchanged.
