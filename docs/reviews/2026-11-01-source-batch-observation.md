# Source batch observation review

Entry main `7b315b244db5a92fe30a29edb9ebccd434b7c2bd`.
WP-008/009/015/016, REQ-003/006/007/010, SPEC-005/008/014/015,
IF-004/006/013, PKG-003. Scope and entry/exit profiles are in
[the contract](../../specs/SOURCE-BATCH-OBSERVATION.md).

The actual episode inspection needed 25 separately observed source contexts.
Even equal reported sequences did not make those independent queries an atomic
snapshot. This extension reuses one existing read-only replay for an explicit
bounded set, returning one checkpoint and exact source states or nulls. It does
not invent another journal, change persistent records or discover job source IDs.

Producer concurrency/recovery review: selection bounds and duplicate checks run
before opening authority; all states are copied from the same replayed state.
The existing SQLite read transaction and hash-chain/witness checks are reused.
No reader holds a writer or media lease. Concurrent appends can return Busy;
other damaged/unsupported authority refusals remain possible. Observations must
never be substituted for fresh expected-revision/owner checks.

Producer privacy/Git review: public summaries expose aggregate counts only;
private ID-keyed state reports remain outside ordinary Git. Profile/report limits,
strict JSON, new-output-only publication and generic refusal diagnostics are
unchanged. Oversized reports must refuse before publication or success output.

Producer storage/custody review: `single_checkpoint` refers only to journal state.
It says nothing about process liveness, present byte availability, producer success,
full claim coverage or recovery/removal authority. Existing active jobs and their
runtime pins are untouched by implementation/testing in this isolated worktree.

Local verification passes 32 journal tests, six observation/acknowledgment CLI
tests and six transition CLI tests. New cases preserve current/pending/predecessor
states and explicit absence, reject invalid selections and foreign/rollback/
corrupt authorities, prove before-commit isolation and coherent two-source
concurrent observations, preserve existing outputs and refuse an actually oversized
six-source history report without mutation, truncation or success stdout.

An initial concurrent synthetic run refused with an unexplained generic Sql
error. Twelve isolated concurrency reruns, eight full journal-suite reruns and
the final expanded suite pass; this does not establish the cause of the original
refusal. The initial failure remains recorded privately. No assertion was relaxed
to accept Sql, and temporary raw diagnostic instrumentation was removed before
final verification. Residual availability finding, owner Tig concurrency/recovery:
investigate if it recurs, without converting a refusal into an observation.

Disposition: producer-applied pass_with_risk for this bounded observation seam.
Exact-head hosted qualification and actual consumer adoption remain separate.
No independent review, media verification or complete readiness is asserted.
