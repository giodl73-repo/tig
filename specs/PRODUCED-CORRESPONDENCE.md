# Claimed produced-output correspondence

Entry 77a7e14d25a40b03e5343724156e90e638fbf078, depends on qualified PR 26.
WP-008/010/011/012; REQ-001/006/009/010; SPEC-005/010/011/012;
ARCH-003/005/006; IF-005/010/011; DES-005/009/010; PKG-004/006.
RUST/GIT/INTEGRATION: L0 fmt/Clippy/portable/canonical VTRACE; L1 exact byte pins,
strict parsing, duplicate/foreign subjects, parameters/provenance/policy mismatch,
bounded claims and planned/unproven/stale states; L2 synthetic report and scene-like
outputs plus actual temporary Git/metadata-store reopen and native context replay.
Allowed: pure optional tig-assets claim/comparison module, explicit native metadata
replay adapter, synthetic tests and bounded docs. No producer execution, new
provider/cache, daemon, auto-selection/enrollment, transfer/removal or live runtime
switch. Art donor 119c66446f2e95129d8d8f143d60ec6220d2813a model freshness tests
inform behavior, not legacy record/hash import or execution evidence.

A recipe fingerprint alone cannot attribute selected bytes to production.
Generate a bounded asset-side claim (at most 65,536 bytes, 1..256 outputs), with
an exact byte pointer from the producing project. Reuse native .tig selected/
stream subjects; do not add another authored dependency list or flat inventory.
Proposed experimental schema: tig.produced-claim.experimental-1. Store exact
operation/code/parameters, claimed recipe, source checkpoint/binding/admission/
consumption provenance, native context root/header witness, and output subjects.
The claim is unsigned data, never execution, approval or custody authority.

An output subject is stable asset ID, exact admitted definition reference,
selected-or-named-stream selector and payload Object. Construct it from an exact
bound native instance or qualified immutable resolver result under caller
admission. A sealed BoundSubject is required for the current comparison; a
deserialized claim Subject cannot stand in for current source selection.
Do not pin the entire .tig record
inside a claim it references: that creates a self-hash cycle. Different output
bytes, slot, asset or definition are not interchangeable. Exact definition changes
need separate compatibility admission; a mismatch is not an automatic rerender.
Producer subjects are not globally restricted to graph Output class: a generated
cel is also an episode's canonical primary input. Compiled masters remain forbidden
as canonical Production dependencies. Project status vocabulary stays outside the
claim schema; no built-in golden field/value or inherited approval.

Strict parse checks the externally supplied Object before decoding; refuse
unknown/duplicate fields, unsafe/conflicting sizes, invalid selectors, duplicate
asset/slot keys, wrong schema and oversized claims. Parsed claims are immutable.
Pure plan checking requires the exact original Plan, including operation/parameters
and every provenance digest; it does not verify the context pin or producer use.
Only a plan-checked claim may enter the pure comparison. Compare against the
current exact output subject and desired Plan, allowing truthful old provenance
with an unchanged recipe. States: Unbuilt, Unproven, ClaimedRecipeMatch, StaleRecipe.
Present bytes without a claim remain Unproven; do not fabricate receipts for
historical approved masters. A foreign/mismatched claim is an error, not a stale
or missing result. Same-recipe multiple produced candidates remain distinct.

Native replay takes external graph/consumption/operation admission and explicit
native/tree/query/recipe bounds. Restore the claimed context through existing
ports, require its retained descriptor Object to equal Operation.parameters,
replay the original graph/recipe, then check the claim against that exact plan.
Return the source, context and checked claim without byte-cache IO or execution.
This verifies declared metadata correspondence only; it does not authenticate
the producer, read output payloads, prove actual consumption, enroll a job, grant
approval/delivery currency, complete claim coverage or authorize eviction.

Callers retain/verify context, code, parameters and outputs through actual job/
custody admission. A claim pin or journal row cannot substitute for those gates.
Raw claims/descriptors remain private assets; only bounded observations enter Git.
Exit: metadata-only source edits can reuse a matching claim, consumed/parameter
changes are stale, byte/subject substitution refuses, unbuilt/unproven history
is explicit, and frozen original context replays after store reopen/Git metadata
unavailability. Producer six-role findings must retain privacy/authority and exact
subject boundaries; no independent or human acceptance is implied.
