# Explicit retirement CLI and agent contract

Input 3e300ca0dea0c3e36944e2f6743ecdbc939dd89a; WP-003/004/006/007/008/009/015;
REQ-001/002/003/004/005/006/007/008/009; SPEC-002/003/004/005/006/007/008/009/013;
IF-002/003/004/005/006/008/012/013; PKG-002/003/008; RUST/PROVIDER profiles.
Allowed: versioned CLI plans/status and explicit execution over existing synthetic
managed stores, existing metadata readers and namespace/retirement ports. Entry:
incarnation-fenced retirement/cancellation/reacquisition implementation. Exit:
actual process plan/apply/status/resume/cancel/reacquire with independent private
policy admission, redacted errors and exact retry outcomes. No initialization,
live deployment, automatic Git workflow, final deletion or provider migration.

`tig-storage retire describe` discovers this interface. `retire plan PROFILE
REQUEST` produces canonical library plan bytes; `retire status PROFILE PLAN`
reads historical action state plus a distinct fresh object observation.
`retire apply PROFILE PLAN`, `resume` and `cancel` use the same existing library.
`retire reacquire-plan PROFILE RETIRED_PLAN REQUEST` emits a canonical explicit
new-operation plan; `retire reacquire PROFILE PLAN` streams from the independently
admitted retained source, with no arbitrary input path or implicit resurrection.
`retire reacquire-status PROFILE PLAN` reports the new publication's fresh state.
Existing copy commands/encoding remain compatible; discovery links this namespace.

Requests have no paths, keeper grants or policy floor. The retirement request is
`{"schema":"tig.retirement-request.design-1","operation":"unique-action","publication":PUBLICATION}`;
the reacquisition request is
`{"schema":"tig.reacquisition-request.design-1","publication":PUBLICATION}`.
PUBLICATION is the existing exact Request: operation, owner, object (SHA-256 and
bytes), and required acknowledgement `process_restart`. Reacquisition uses a new
publication operation. Success emits canonical plan/status/receipt JSON directly;
failure exits nonzero with the existing `tig.storage-response.design-1` envelope,
`ok:false` and a static code, without root paths or provider errors. Keep exact
plan bytes outside Git. The integration fixture demonstrates the complete shapes.

All documents are strict JSON, at most 64 KiB, with no links/reparse points. Plans
are canonical and input-bound. A private `tig.storage-retirement-profile.design-1`
admits actual source/enrollment, ledger witness/job, metadata reader, namespace
witnesses, keeper roots/ledgers/grants/domain enrollment and namespace-place
assignments. Its policy pins owner/context, minimum survivor domains, byte/peak
budgets and separately enabled eviction/reacquisition/cancellation. Disabled
execution remains inspectable. Portable plans cannot lower the floor, substitute
grants, change role/domain assignment, owner/context or authority, or enable apply.

The first policy conservatively requires every configured survivor for each action.
Source and survivor place assignments must cover every actual enrolled namespace
policy key; unknown/unmapped roles refuse execution. This is a fixed admitted
assignment, not an optimizer. Vocabulary is project configuration. Profiles remain
operator-owned outside Git; their context/authority assertions are not automatic
authentication or unmanaged-client discovery. Untrusted repository/workflow content
must not become the private profile. No per-copy prompt is required once bounded
execution is independently enabled; dry-run is the default policy state.

Profile fields (unknown fields refuse):

| Field | Meaning |
| --- | --- |
| schema | `tig.storage-retirement-profile.design-1` |
| source_root, enrollment | Existing design-7 provider root and exact store-protection Enrollment |
| ledger_root, minimum, job | Existing source capacity authority, trusted minimum Checkpoint and reserved JournalRef |
| source | Independently enrolled store identity and failure_domain |
| metadata | Existing root and trusted minimum metadata Checkpoint |
| namespaces | 1..16 existing root/minimum source Checkpoint pairs, exactly matching the store's immutable roster |
| backups | 1..16 distinct existing root/ledger_root/minimum pairs, enrolled domains, exact grant and places (namespace/place pairs) |
| limits | Existing aggregate protection Limits, including a cooperative elapsed-time bound |
| policy | owner/context, minimum_domains, max_object_bytes (1..8 MiB), max_peak_bytes (1..1 GiB), allow_eviction, allow_reacquisition, allow_cancellation |

Every root is absolute. The three allow flags default independently to false.
Planning/status need no enabled flag. Apply/resume admit every keeper and role;
status/cancel inspect exact historical owner/store/capacity bindings without
requiring current keeper/context availability. Reacquisition selects the first
configured keeper, verifies its exact retained grant and streams those bytes;
unavailability refuses without fallback discovery. Profiles are per-object scopes
in this first interface, not a project-wide inventory or policy optimizer.

Status does not admit allocations, append journal commands, initialize/repair
metadata, hydrate bytes or deserialize live guards. Retired/cancelled receipts
remain historical even after same-hash reacquisition. Present bytes are freshly
verified separately; missing/corrupt/unknown evidence is not absent demand. An
unavailable keeper/namespace does not prevent reading a recorded source outcome;
execution still requires full fresh scope. Cancellation adds protection and does
not require keeper or tree availability. Ledger opening retains existing recovery
semantics; read-only here means no new commands, not an OS read-only connection.
Historical state and fresh observation are separate samples, not an atomic global
snapshot. Missing terminal evidence under a settled allocation reports
recovery_required. A damaged acknowledged active copy is unproven, never converted
to authorized absence. Reacquisition status checks both lifecycle and payload
allocation bindings; corrupted/missing completed bytes require recovery.

L0 validate/diff/fmt, hashes and six producer lenses. L1 focused process fixtures,
locked default/all-feature workspace, strict Clippy and standalone Git. L2 actual
temporary unlink/restore, cross-process restart/retry, held claims/readers,
disabled/forged policy, unmapped role, stale instance/context/budget, missing keeper,
status without command growth, historical-versus-present reporting and static
redacted failures. Full native/large-file/restore/epoch/lifetime and live adoption
gates remain; implementation and verification/validation/trace/review close apart.

The explicit pending-cancellation L2 fixture requires a built binary unavailable
to a library-only Cargo invocation. After building `tig-storage`, set
TIG_RETIREMENT_CLI to its absolute path and run:

    cargo test -p tig-store --lib --locked retirement::tests::explicit_cli_pending_cancellation_qualification -- --ignored --exact

It creates synthetic stores, exits a child after real durable intent, then runs
actual status/cancel/retry processes with keeper/tree/namespace roots unavailable.
The ordinary library suite marks this one cross-artifact fixture ignored; it is
explicitly required/run by Windows and Linux CI after building binaries. A skipped
ordinary-suite entry is not evidence that this qualification passed.
