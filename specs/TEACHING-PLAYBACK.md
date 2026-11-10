# Recorded teaching lessons in the catalogue

Input: 6d9a2e024a2ddaeae5409a4df3d7077a8c9b1965; producer runtime
3ee533c. Extends [the public course](TEACHING-CATALOGUE.md), without closing it.
WP-008/010/015/016; REQ-001/005/006/007/009/010;
SPEC-002/005/008/010/014; IF-002/005/008/010/011/013;
PKG-002/004/006/008. RUST/PROVIDER/UI/INTEGRATION profiles.
Allowed: optional privately enrolled teaching metadata, bounded verified read-only
API, sample report export and shipped-script lesson playback. Existing catalogue
and standalone Git compatibility remain. No storage command, new cache engine,
provider initialization by the viewer, arbitrary browser path or live root.

Entry: qualified ordinary public rehearsal, existing verified reader and host/
session protections, metadata outside the catalogue mutex, current source/roles.
Exit: a learner selects recorded lessons, sees actual before/desired/observed
copies with stable identities, understands shared/job-held bytes, and returns to
the sample catalogue without losing the lesson. Missing/corrupt/foreign metadata
and stale replies cannot produce a successful or misleading lesson view.
L0: structure/pins/fmt/diff. L1: locked native/API/controller and strict Clippy.
L2: fresh sample producer -> verified API -> shipped controllers; tampering,
missing data, source mismatch, stale replies, bounded work and browser/user
rehearsal. Keep implementation, verification, validation, trace and review separate.

An optional catalogue teaching profile pins an exact trace object through an
existing private byte-reader profile and explicit project/provider-role order.
The fixed authenticated GET route accepts no filesystem paths or commands.
Read at most16 reports, each at most64KiB, with a finite aggregate byte/read budget.
Check canonical JSON, exact object hashes/sizes, unique lesson names, matching
plan/receipt identities, consistent provider order and object sizes, known roles
and explicit synthetic-domain limits. Cap the sanitized response at64KiB.

Read metadata outside the catalogue mutex. Return only lesson labels/explanations,
source commits and availability, stable object identities, planned/observed role
sets, transfer/effect summaries and report/plan pins. A missing row is unknown,
not an empty set. The view is a recorded rehearsal, not current custody or new
authority. Existing metadata snapshots and current copy inspection stay distinct.
Use text nodes for enrolled labels, retain stale-response fencing and keyboard
controls, and keep lesson selection when visiting the catalogue and returning.

The producer exports actual sealed report/trace bytes through the existing flat
reader layout in fresh fixture roots. It does not regenerate outcomes in the
viewer. Ordinary copies and source transitions remain owned by the existing
executor. Publisher/failure/final-loss/lifetime lessons, general native writer/
authority/provider/large-file qualification and human acceptance remain owed.
