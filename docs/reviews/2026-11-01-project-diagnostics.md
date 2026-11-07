# Project refusal diagnostics: producer review

Entry: main `81a97d3`. Targets: `tig-project` stderr mapping, CLI refusal
regressions and `specs/PROJECT-DIAGNOSTICS.md`. Controls and exit obligations are
recorded in that spec; this is a producer checklist, not independent acceptance.

Provider/privacy lens: only exact reviewed static errors map to public static
codes/hints. Arbitrary provider text, paths, identifiers, appended secrets and
terminal control characters fall back without interpolation. Private JSON result
reports remain private. No new logging destination or telemetry is added.

Concurrency/recovery lens: restore, source protection, copy, no-replace, progress
and cleanup code are unchanged. Existing copies can remain after a reporting
failure. The hint recommends verified retry and never claims rollback or safe
disposal. No global lock or automatic recovery is introduced.

Local verification: one bin privacy test, 42 project integration tests and eight
single-restore CLI tests pass. Hosted qualification and consumer installation
remain separate. Static error matching deliberately falls back when a library
message changes; it cannot disclose new raw errors by default.
