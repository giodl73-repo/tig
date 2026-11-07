# Deferred build assessment

Entry 7adb6e529e0374e0f41d561a04fe0a838296ef0d. WP-010/011/012;
REQ-001/006/009/010; SPEC-010/011/012; IF-010/011; PKG-004/006.
Allowed: bounded pure assessment over existing Plan, BoundSubject and
PlanCheckedClaim capabilities; synthetic tests and small control documents.
No new provider, renderer, queue executor, claim authority, Git hook or cache IO.
RUST/INTEGRATION L0: structure, fmt, warnings-denied Clippy, diff checks.
L1: mixed states, malformed/duplicate keys, limits, foreign claims, redaction,
checkpoint drift and deterministic ordering. L2: synthetic independent cel/cue
changes coalesced at one exact source checkpoint, plus 1,000 output requests.
Exit requires actual passing fixtures; full producer/adoption acceptance stays open.

An explicitly scoped assessment evaluates each (asset ID, selected-or-stream
selector) once. Desired plans must name the caller's exact source checkpoint.
An unavailable plan is Blocked, not Unbuilt. A mismatched checkpoint, current
subject or checked claim is Blocked, never a successful reuse or build decision.
Missing output/claim is Unbuilt; existing output without proof is Unproven;
an exact checked claim can yield ClaimedRecipeMatch or StaleRecipe through the
existing correspondence algorithm. Original context replay stays caller-owned.
Current-output and claim observations explicitly distinguish Absent, Present and
Unavailable; a failed/corrupt lookup must be Unavailable and blocks assessment.

Unbuilt and StaleRecipe are catch-up candidates only. Unproven and Blocked
require explicit investigation; never automatically rerender historical approved
masters to invent proof. A matching unsigned recipe is not a delivery gate pass.
No result grants producer execution, complete semantics, eligibility, approval,
payload integrity, custody or removal authority. Reassess after any source change;
this is not a durable job capability or a claim to have enumerated every output.

Reject duplicate/invalid keys and entry budgets atomically. Sort results by exact
key for reproducibility. Bound each batch to 1..4,096 entries (caller may choose a
smaller maximum); split larger reports outside ordinary Git. Independent variants
use distinct stream selectors/asset IDs, not implicit language expansion. Debug
reveals counts only; explicit entries contain private IDs for the trusted caller.
Reports and growing triage lists belong asset-side, not shared authored manifests.

Producer pre-code review: identity requires exact checkpoint and subject keys;
Git/adoption requires no duplicated dependency list or working-tree mutation;
privacy requires no raw claim/prompt/error text in diagnostics; placement/custody
require candidate status never imply eviction authority; recovery requires no
mutable queue or background work. Full schema/variant, actual native consumed
coverage, managed scheduling and custody remain open.
