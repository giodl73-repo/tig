# Shared asset delivery history

T1b of [the landing plan](../docs/VERSION-DELIVERY-LANDING.md), input
`48140c1df346b7865dd417dbfc1b59854c0914e3`. WP-010/011/012/015,
REQ-001/006/010, SPEC-010/011/012/014/015, IF-010/011/013, PKG-006/008.
RUST/INTEGRATION profiles. Allowed: common pure delivery reader, shared bounded
history utilities, extraction of the existing receipt checker for explicit subjects,
synthetic integration tests and bounded specs/review. No network, CLI mutation,
provider/cache operations, automatic selection or consumer project conversion in T1b.

Entry: exact native/schema binding, T1a revision contracts and existing receipt
mapping/parser behavior. Exit: one common contract for multiple targets/retained
revisions; strict causal conflicts/limits/missing-state tests; exact retained
receipt subject/target checks using the existing parser; unchanged legacy receipt
reports and project selection. Full T1 qualification remains separate from T2
write operations and T3 custody/placement integration.

## Attachment and immutable records

`metadata.delivery` uses `tig.delivery-state.experimental-1`, required
`policy_slot` (role `policy/delivery`) and optional `history_slot` (role
`history/delivery`). No attachment is valid legacy/unconfigured state. A policy
with no history means untracked, not never delivered. Domain types remain intact.

A policy has schema `tig.delivery-policy.experimental-1` and 1..256 targets with
unique keys and declared identities:
`{key, identity, destination, representation, profile, gates}`. Identity is the
existing opaque declared-publication-target Digest; destination/profile/gates are
pinned opaque documents admitted by publishing adapters. This reader does not
interpret them or grant authority. Policy changes do not rewrite past events.

A page has schema `tig.delivery-history.experimental-1`, `asset`, `coverage`
(complete/partial), `parents` (0..16 history page Objects), and `events` (0..64
event Objects). Append and union preserve old pages and share exact objects.

An event has schema `tig.delivery-event.experimental-1`, `asset`, stable `event`
and `operation` keys, `parents` (0..16 event Objects), `policy`, `target` key,
`revision` manifest Object, `representation`, exact payload `subject`, `profile`,
`stage` and optional `evidence`. Stages: attempted, acknowledged, readback, failed,
absent. Evidence is `{kind: receipt, binding: Object}` or
`{kind: observation, object: Object}`. Acknowledged/readback require receipt
evidence; absent requires an observation. These are retained claims, not remote
authentication, creative approval or a current remote census.

Every referenced policy is pinned and loaded. Subject/representation/profile must
match that policy and a revision in the checked asset revision history. A parent
event must address the same exact target identity. Different representations
can succeed each other only if explicitly allowed by their own pinned policies;
no language/profile fallback. Reusing an event key with different bytes conflicts.
Events in one operation bind the same policy/key/identity/revision/subject/profile;
separate real publications of identical bytes can use distinct operation keys.

## Receipt evidence without receipt assets

An immutable asset-side binding document uses `tig.receipt-binding.experimental-1`
and `{subject, receipt, provider, scope, selector, mapping}`. The separately pinned
mapping uses `tig.receipt-mapping.experimental-1` and existing `limits`/`rules`.
The exact same JSON selector, row/key/opaque-locator rules, strict external JSON
parser and limits as the publication checker are reused. This is evidence data,
not an independently authored `.tig` receipt asset or domain definition.

Verify binding and mapping raw pins, receipt hash/length, selected row's exact
subject and declared target Digest. The latter must match the event policy.
No native instance with invented authored target fields is manufactured for this
check. Existing publication Checker/Report semantics and wire shape stay intact.
One parsed raw receipt can serve many bindings; each applies its own exact pin,
selector and admitted limits. Wrong subjects, targets or unknown rules fail.
Observation evidence remains an explicit unverified reference until an admitted
adapter evaluates it. A readback stage alone does not certify actual byte readback.

## Causal state and limits

Target state is queried by current policy key and grouped by exact identity across
historical key renames. A policy cannot pretend two keys are independent endpoints
with the same identity. Changing a key without changing exact identity retains its
history; changing actual identity never transfers old publication claims.
Consider acknowledged/readback/absent claims; failed/attempted descendants do not
erase a prior publication. Causally dominated claims leave the frontier. Zero
claims is untracked; one is one retained claim; incomparable claims are ambiguous.
Partial history reports unknown coverage, not a usable current-target assertion.
There is no timestamp or Git-order tie-breaker. A provider check/conditional update
is still necessary to establish real current state. The returned retained claim
also pins its actual profile/policy, independently of newer desired requirements.

Use the same strict 64 KiB native subset and global T1a limits for revision pages,
delivery pages/events/policies/bindings/mappings and checked receipt bytes together:
4,096 distinct read objects, 64 MiB cumulative bytes, 65,536 references. Receipt
bytes may use the existing 16 MiB per-object external-evidence ceiling. Readers
receive exact maximums and must bound acquisition; limits reject before fetching.
Digest/size consistency covers all native, revision, policy and evidence pins.
Missing/corrupt history fails closed. No recursive unbounded DAG walk, hidden
filesystem/provider access or generated inventory in Git.

Reports distinguish metadata, receipt bytes, payload and opaque evidence/policy
references. Opaque witness/destination/profile/authority closures still need
admitted adapters before retention decisions. No receipt/label/complete-declared-
history result proves independent custody or authorizes eviction. Reports avoid
Debug/Serialize of IDs/locators; explicit caller-known key lookup is available.

L0 structure/diff/fmt/role review. L1 existing publication regressions, new common
binding and delivery tests, locked all-feature workspace tests, strict Clippy and
standalone Git/LFS compilation. L2 synthetic image/audio/document integration,
two-target/version timeline, concurrent branch frontier, and thousand-event bounded
reader fixtures. Real CLI/provider/consumer acceptance remains T2/T3/consumer project work.
