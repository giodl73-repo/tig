# Fresh admission in the handoff fixture

Input `083db03dbcbaa83d0a639841645e9b71b3664193`; WP-006/009,
REQ-003/004/005/009, SPEC-007/008/009, IF-003/004/006, PKG-002/003/004.
RUST/PROVIDER/GIT profiles. Allowed scope: synthetic handoff fixture admission and
one deterministic refresh regression. No production code, limit or policy change.

Windows run41569965126 refused two fixture reservations with NeedsProbe at
placement_handoff_cases.rs:149. The fixture observed free capacity and assumed
that sample would remain fresh until reserve. The engine correctly enforces its
five-second freshness fence even under scheduler/IO delay. Do not raise the fence
or treat a stale/historical probe acknowledgement as fresh admission.

Refresh only on NeedsProbe, with distinct probe/observation operation IDs, one
stable reservation operation and at most eight attempts. Other failures remain
failures. A regression reopens the ledger after the first probe to deterministically
invalidate process-local freshness, then checks fresh retry succeeds without a
sleep or changing the engine's clock rules. Producer recovery/custody/placement
lenses preserve bounded admission and all original byte-protection tests; no
independent review, production readiness or custody claim follows.

Exit evidence: named refresh case and all existing handoff cases pass, strict
Clippy/L0 and exact-head Windows/Linux CI. Qualification pending at entry.
