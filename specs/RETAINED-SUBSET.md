# Partial historical-root settlement

Input63f2c96e991553e45a55f2cab1b645a945ede496.
WP-003/004/006/007/008/009/015; REQ-001/002/004/005/007/009;
SPEC-002/004/005/006/008/009/014; IF-002/004/005/006/008/013;
PKG-002/003/004/008/009. RUST/PROVIDER profiles. Allowed: exact owner-authorized
branch/draft historical flat-binding subset settlement, existing tree/metadata/
guarded-journal composition and explicit private CLI. No payload removal, source
format migration, native graph interpretation, retention/job override, automatic
age/category selection or live store enrollment. Entry: admitted namespace,
policy, owner and eligible exact paths within one exact predecessor revision.
Exit: bounded plan/status/apply/resume with source fencing, protected survivors,
historical retry outcomes and synthetic interruption/race qualification.

A request names one source, predecessor revision and 1..1,024 sorted unique exact
logical paths. Missing paths, unavailable or corrupt closure, an unresolved source
prepare, foreign owners and unadmitted classes hold. This controller admits only
Branch/Draft classes; it cannot release Job/Retention obligations. Callers must
admit selection and owner authority independently. Paths are resolved only in the
selected immutable flat root; their names have no category or age semantics.

Planning reads the full bounded original closure, removes the exact bindings with
Tree::edit and verifies the resulting closure. Only sparse newly written nodes
are staged; existing metadata is reused. Aliased bytes still bound at an unselected
path survive. Summary counts concern this root only, never global eligibility,
physical copies, custody or reclaim. Other owners, current/pending selections,
jobs, namespaces and provider retention remain independent protection.

For a nonempty survivor set, finish and independently read back replacement
metadata before enrolling a deterministic derived source with the same owner and
immutable class. Namespace::select_predecessor atomically composes existing
Prepare/Adopt/Withdraw commands, protecting the survivors as a predecessor under
the same original policy. No observer sees an artificial active selection that
could suppress original history placement through prefer_active. No global pause
is needed. An unrelated Retention-class source is not a substitute because it
could have different placement obligations. Original protection remains throughout.

Only after exact survivor revision/state and historical controller receipts are
present may the controller Retire the original exact predecessor. The original
expected revision and the survivor source-revision condition are checked in the
existing short journal transaction. A concurrent change to the witness holds the
retirement. No new event encoding or SQL transaction spanning tree IO is required.
An empty survivor set needs no derived source, but still retires only the exact
owner's historical obligation. No step removes payloads or grants removal authority.

Effect identities bind authority, namespace/policy, owner/class, original selection,
request and survivor root. Runtime budgets and unrelated original-source advances
do not create another survivor source on replanning. The final Retire operation
also binds the full plan and its fresh expected original revision. Exact receipt
replay cannot apply a new transition or repair changed survivor protection. A
matching source without the controller's original enrollment receipt is held.

Interruption preserves the original or acknowledged survivor protection; orphaned
derived sources may overprotect after a stale plan. Cancellation means stopping
future steps, not rollback. Completed claim settlement is reported as such;
owner-authorized recovery is separate. Metadata-offline status separates historical
retirement from observed current survivor protection and never claims a lease.
Later explicit survivor retirement is not undone by an old plan retry.

Plans use tig.subset-plan.design-2 strict canonical JSON bounded to 64 KiB;
the draft-only design-1 plan/effect is refused after correcting transient active
placement. Existing source event bytes and historical receipts are unchanged.
Enrollments pin finite projection
read/time/binding limits plus at most 65,536 sparse writes / 32 MiB. Each projection
shares one aggregate budget across original traversal, edits and survivor traversal.
Apply executes at most two such projections plus a separately bounded finished
closure readback; each existing source mutation independently enforces admission
limits. Metadata provider capacity and source/journal lifetime bounds still apply.
A nonempty subset uses one of 1,024 source slots and four survivor commands before
the original Retire; safe exhaustion holds with extra protection. Continuation and
eventual record retirement remain required before sustained use, never a limit reset.

L0: fmt, portable validation, reviewed hashes and diff. L1: locked default/all-
feature workspace, strict Clippy, parsing/limits/owner/class/stale/replay cases.
L2: existing temporary metadata/namespaces, partial-root aliases and foreign/job
holds, sparse edits, witness/original races and actual child exits before/after
survivor protection and original retirement. Independent review, native/private
coverage, final deletion, metadata expiry and live adoption remain open.

## Explicit operator interface

`tig-source subset plan PROFILE REQUEST` emits exact canonical plan bytes to
stdout without a newline; save those bytes outside Git. `status PROFILE PLAN`
inspects receipts/source state without opening metadata. `apply PROFILE PLAN`
and `resume PROFILE PLAN` perform the same resumable steps. Strict private
`tig.subset-profile.design-1` contains enrollment, namespace_root/minimum,
metadata_root/minimum and default-false allow_settlement. The actual field names
are namespace_root, namespace_minimum, metadata_root and metadata_minimum.
All roots must already exist, be absolute and pass their providers' owned-path
checks; mutation refuses overlapping namespace/metadata roots. No source or
metadata store is initialized by the CLI. Plans omit private filesystem roots.
The enabled private profile supplies authority; plan content cannot enable apply.
Status's survivor_protection_current concerns source claim state, not metadata
availability, complete global protection, custody or physical convergence.
