# Pre-execution composition dependencies

Entry `4e80e0116e36567d1e1f10dbc6c6acf742e2f192`; WP-004/008/009/010/011,
REQ-001/002/003/004/006/007/009/010, SPEC-002/004/005/008/010/011,
IF-002/004/005/008/010/011, PKG-003/004/006. Scope and entry/exit/L0/L1/L2 are
controlled by [the amendment](../../specs/NATIVE-JOB-CONTEXT.md).

The existing job envelope could not retain already-produced composition objects
before execution without classifying them as primary inputs or runtime binaries.
`Dependencies` and explicit freeze/restore APIs add named composition references
and input evidence, with optional retained variants. Composition has a distinct
role; input evidence remains Evidence. Original Admission and entry points stay
source-compatible. Nonempty maps use experimental-3; empty maps preserve exact
experimental-1/2 roots. All maps share whole-entry, safe-size and prefix checks.

Exact readback admission rejects omission, addition, substitution, rename and
role changes. Byte aliases may share one immutable object and logical size;
semantic role is not inferred from hash. Links protect declared resources, not
consumed meaning. Consumers must also bind composition subjects/claims in recipe
parameters/policy and qualify freshness. No new provider/journal, renderer, cache
operation, automatic transition, global lock or removal is introduced.

## Producer role review

| Lens | Finding and disposition | Evidence / remaining scope |
| --- | --- | --- |
| Identity/namespace | New version/namespaces only for added obligations; old roots preserved | Actual v1/v2 identity, map/role/prefix tests; no released wire contract |
| Placement | Composition differs from primary/runtime/claimed output | Role/shared-byte tests; no global demand or placement assignment proof |
| Storage/custody | Metadata links acknowledge no payload availability or custody | Missing payloads permitted in metadata fixtures; removal remains open |
| Concurrency/recovery | Independent owners survive restart and own-reference retirement | Withdraw retains predecessor; explicit retirement leaves peer protected |
| Provider/privacy | Existing explicit metadata ports; no discovery or public raw reports | Bounded maps, zero-IO refusal, no live cache or transport |
| Git/adoption | Candidate freeze differs from finish/readback/owned admission | Real Git/SQLite/source-offline and interruption/retry fixtures; consumer adoption pending |

Method: producer-applied checklist review, not independent or human approval.
Disposition: pass_with_risk for this seam. The existing long `prepare` remains
the single shared validation/role derivation for all APIs, preventing budget or
role bypasses. Prefix validation and dependency insertion are small helpers;
zero-IO negatives cover the shared boundary.

Initial fixture compilation caught Rust argument evaluation moving a key before
its lookup; corrected without dependencies. The first restart assertion expected
withdrawal to discard protection, but the engine correctly retained two roots.
The corrected fixture verifies that protection, explicitly retires only the
owner's predecessor, and checks the peer remains. No invariant was weakened.

Qualification: six dependency tests pass (4.72s), ten variant tests pass (5.84s),
all 78 native source tests pass (265.73s), all-feature/all-target workspace Clippy
with warnings denied passes (1.98s), and standalone no-default-feature Git/LFS
compilation passes (18.21s). The first structural run correctly refused changed
reviewed source pins. After explicit review-target byte/hash refresh, portable
structure/trace/role checks pass: 263 files, 2,372,886 bytes and 255 local links.
Hosted Windows/Linux and downstream qualification remain required before adoption.
Canonical VTRACE CLI is not configured here; repo-owned trace/role checks apply.
