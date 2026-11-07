# Retained publication subjects independent of source selection

EVID-076; WP-010/011/015; REQ-001/006/010; SPEC-010/012/015;
IF-010/011/013; PKG-006/008. Entry `d3b8db1d649f813e1640fb515ded969724b9ce5a`.
Allowed scope: pure optional publication checker, synthetic fixtures, explicit
CLI and contract/review records. RUST + INTEGRATION; no provider or runtime change.

## Problem and contract

A target can retain A while its source selects B or is unbuilt. Replacing B
with A merely to validate A's receipt would falsify the source. Rejecting every
such relationship would also lose useful publication history. The admitted
policy now explicitly chooses `selected` (unchanged default) or
`declared-stream`. The latter selects one exact slot/role, verifies that object's
receipt, and reports exact equality to the pinned current selection separately.

There is no fallback, newest-version inference, metadata rewrite or second
authored selection. Missing slots, wrong roles, mismatching receipt subjects,
invalid modes and malformed evidence refuse. Receipt and subject slots remain
distinct. The original record, definition, schema, policy and receipt are still
pinned in the report. Default serialized report fields remain unchanged. Only
explicit stream mode adds its mode and `subject_is_selected` observation.

That observation is deliberately not named currency, approval or freshness.
Even equality proves none of those properties. An absent current selection
compares false; it does not make the retained subject a new selected build.
Target identity remains stable across byte revisions, while separate provider
targets can bind different streams from the same instance.

## Executed evidence and remaining gates

The baseline executes 35 publication cases: 29 pass and six fail, reproducing
the unsupported mode and compiled CLI gap before implementation. With the fix,
all 35 pass (1.03 s). Six new cases cover different/absent/equal selections,
default strictness and serialized compatibility, exact-role/slot/receipt/mode
refusals and two independent targets. The existing compiled CLI/privacy case
also runs the retained-subject fixture, checks redacted failures and verifies
that only its five explicit metadata files exist afterward.

Full locked all-feature workspace tests pass (689 passed entries, including
subprocess helpers). Strict all-target/all-feature Clippy passes (54.32 s),
as does the standalone Git/LFS build (17.69 s). Foundation, formatting and
whitespace checks pass. Reviewed target pins are updated only for the changed
contract/code/review scope; no source budget or runtime assertion is relaxed.
Exact-head hosted qualification remains separate. Actual consumer fixture
adoption, complete shelf reconciliation,
source/job protection, custody, placement and deletion remain open.

## Producer role review

| Lens | Finding and disposition |
| --- | --- |
| Identity/namespace | High: source and published subjects must not be substituted. Exact stream selection, equality reporting, immutable pins and unchanged-record tests address this component scope. |
| Provider/privacy | High: matching metadata must not imply cloud presence or expose target details. Provider/current-remote flags stay false; API/CLI diagnostics remain private-safe. |
| Git/adoption | Preserve the default contract and original source selection. New behavior requires an explicit admitted mode; older tools refuse that unknown policy field. Consumer/runtime acceptance remains separate. |
| Storage/custody | Different subjects are both known references, never deletion permission. No cache IO, claim changes, custody or eviction authority is added. |
| Concurrency/recovery | Checker inputs remain immutable and source-pinned; no mutable latest pointer, journal, retry writer or shared lock is introduced. |
| Placement policy | Object equality is not a placement or lifecycle classification. A retained publication may still be required even when selection differs. |

Method: producer-applied code/contract/fixture review, not independent review or
human publication approval. Owner: optional typed-asset checker. Disposition:
component verification only, pending separate qualification and consumer gates.
