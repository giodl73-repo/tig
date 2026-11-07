# Source refusal guidance: producer review

Entry main `ad330ddf9e5635bc66be715f7684dc40396efba5`. Controls, scope and
L0/L1/L2 exit criteria are in [the contract](../../specs/SOURCE-DIAGNOSTICS.md).
Targets: source CLI stderr, private diagnostic module and synthetic regressions.
No library, source journal, transition, provider or persistent format changes.

Provider/privacy lens: direct typed journal errors map to reviewed static codes
and hints. JSON errors get static profile guidance. Unknown/private/decorated
messages cannot impersonate journal categories; no error text, nested error
chain, path, owner, request or terminal escape reaches stderr. Detailed evidence
remains private. The fallback is deliberately less specific than raw errors.

Recovery lens: hints distinguish stale source state from owner/checkpoint and
operation problems. They forbid weakening minima, bypassing ownership, resetting
authority or discarding obligations. A reporting failure can follow a committed
transition, so generic guidance preserves exact requests for acknowledgment
instead of promising rollback or recommending a new operation ID. No automatic
retry, cleanup or global lock is introduced.

Local verification: 15 scoped tests pass (two diagnostic unit tests, six
observation/acknowledgment CLI cases and seven transition CLI cases), including
typed category bounds and private lookalikes. The stale-apply and wrong-owner
cases preserve journal bytes and create no intent/report. The existing
committed-transition/receipt-failure recovery fixture also passes. Strict scoped
Clippy passes. Hosted qualification, installed consumer usability and independent review remain
separate; these are producer-applied checklists, not additional reviewers.
