# Bounded project CLI refusal diagnostics

Entry source `81a97d30bd4ea2b54ab5845f213e6eaa92adbe07`;
WP-005/011/015/016, REQ-001/003/006/010, SPEC-004/007/011/012/015,
IF-005/008/010/013, PKG-003/004/008; RUST + INTEGRATION profiles.
Scope: operator stderr diagnostics in `tig-project`, with no changes to restore,
planning, providers, journals, permissions or automatic operations.

The existing generic refusal line and exit status 2 remain. An additional bounded
stderr line contains `code=CODE; HINT`. Exact reviewed static library errors map
to argument, profile, manifest, plan-change, destination, budget, selection or
reporting diagnostics. No substring matching, raw error interpolation, paths,
record IDs, provider addresses or credential-bearing errors reach this line.
Unknown and decorated errors use `operation-refused` and a static generic hint.
These experimental codes identify a refused check, not complete root-cause
classification or a structured provider-error API.

Failures represented by the existing JSON operation report retain that report's
per-entry and phase information; this extension does not replace or summarize
private report contents on stderr. An output-sink failure can occur after copies
exist. A refusal does not establish rollback or authorize cleanup. Retry remains
explicit and verifies existing files through the unchanged restore primitive.

Entry: clean source above and synthetic fixtures only. L0 foundation, fmt and
Clippy; L1 exact-match redaction, injected private paths/credentials/control
characters, static fallback; L2 real CLI stale-plan and duplicate-flag refusals
plus project/single-restore regressions. Exit: scoped local verification, producer
privacy/recovery review and separate exact-head hosted qualification. Consumer
adoption and independent review remain separate.
