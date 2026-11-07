# Closed capacity reservation working-set retirement

Input `84136b516a3f784c51398d819af7b44826236be1`. Bounded extension of
RESERVATION-JOURNAL.md: WP-003/006/008; REQ-003/004/005/007/009;
SPEC-003/004/005/008; IF-003/004/005/006; PKG-001 pure capacity model and
PKG-003 persistent ledger; RUST/PROVIDER profiles.

Entry: existing generation-fenced capacity tokens, staged SQL transitions,
canonical command replay and explicit native journal enrollment. Exit: closed
reservations can leave the bounded working set while their exact history and
read-only dispositions remain recoverable from the same journal. No payload IO,
deletion, new authority, larger limits, journal pruning or automatic migration.

## Contract

- The pure model retires only an exactly matched, explicitly finished reservation
  with zero remaining headroom and no undisposed allocation. It returns an opaque
  immutable history value. Retirement changes neither sampled free space nor
  outstanding allocation debits, observation watermarks or generation counters.
- The existing reservation/event working-set limits remain. Unobserved debit
  history has its own existing-size bound; retirement cannot make that map grow
  without limit. A fresh coherent observation may cover earlier debits as before.
- Old tokens become stale in the live model. Reusing a name there creates a new
  generation, never revives an old token. Callers of the pure retirement API must
  retain their own operation/replay fence; the persistent ledger supplies it.
- A canonical `RetireClosed` command uses a deterministic reservation-derived
  operation key. It is staged with the model and SQL event commit. All preceding
  journal bytes, genesis, sequence/hash witnesses and old command outcomes remain.
  Reopening reconstructs the same retirement and immutable disposition.
- Exact historical command retries still return their original receipt and are
  marked replayed. Conflicting operation reuse and new writes against retired
  references reject. Retired history never grants fresh physical IO.
- Publication, cleanup, retention and handoff status queries continue to report
  their exact historical dispositions. Cancellation/binding checks still apply.
- A bounded native preview selects only closed, unretired reservations from the
  admitted ledger view. Explicit retirement may partially commit a batch; exact
  retries are safe. Another handle must synchronize the retirement before acting.
- Journal limits (16,384 commands / 16 MiB encoded payload) remain unchanged.
  This is working-set retirement, not unbounded service lifetime or journal
  compaction. Older readers refuse the new command; consumers must explicitly
  enroll a compatible binary before using it.

## Verification and review

L0: repository validation, exact source/diff, fmt and six producer-applied lenses.
L1: locked workspace/default and all-feature tests, all-target Clippy, standalone
Git feature check. L2: temporary actual SQLite roots; fill the reservation window,
retire/reopen/admit, preserve accounting and old receipts, peer synchronization,
conflicting/foreign/live-reference refusal, and injected pre/post-commit failures.
Pure tests cover generation fencing, non-retirable records, debit bounds and
staged-transition rollback. Evidence/trace/review closure is recorded separately
after execution; no independent or human acceptance is implied.
