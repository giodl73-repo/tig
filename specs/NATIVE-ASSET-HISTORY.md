# Explicit native history inspection

T2 read/validation entry of [the landing plan](../docs/VERSION-DELIVERY-LANDING.md),
input `672c05b2237777f97811d705cc3e7263aa71e2fa`. WP-010/011/012/015,
REQ-001/006/007/010, SPEC-005/010/011/012/014/015, IF-005/010/011/013,
PKG-003/004/006/008. RUST/GIT/INTEGRATION profiles. Allowed: native captured-view
history SDK, existing tig-resolve command, safe aggregate helpers and synthetic
Git/cache integration tests. No new cache protocol/store, writes/proposals,
provider changes, automatic Git hooks, public data or runtime/consumer adoption.

Entry: qualified native binding/schema/byte reader plus common revision/delivery
contracts (delivery PR qualification remains a merge prerequisite). Exit: real CLI
checks exact committed/captured asset history through existing enrolled byte
stores, reports unknown/ambiguous state honestly, refuses malformed/missing data
and remains usable without optional schema support in ordinary modes. This does
not finish T2 registration/selection/delivery proposal operations or T3 placement.

```text
tig-resolve --history PROFILE ASSET_ID DIALECT
```

Build with tig-git `schema`. Ordinary lookup/stream/--verify remain available with
`assets` alone; unsupported history mode refuses before reading any profile.
Profile format remains tig.asset-resolver-profile.experimental-1. Read its exact
Git commit/scopes; execute the requested record's pinned authored schema under
the explicit dialect. Do not interpret worktree changes or validate an unrelated
asset in place of the requested one. SDK also works over captured/archived views.

The existing optional byte_source profile supplies places/layouts/protection and
limits. No second index, per-object file map, new layout or fallback directory.
An unconfigured legacy asset can report unknown history without byte_source or
byte reads. Configured history requires explicit enrolled access; missing profile,
missing object, denial, integrity and unsupported access remain refusals, never
empty history. The reader checks expected maximum before opening and uses held
verified existing reads. Only pinned metadata/receipts are read; referenced media
and opaque witnesses are not opened. No copy, hydration, scan, cache initialization,
lease journal, network call or configuration change occurs.

SDK preserves typed revision/delivery views; CLI emits only bounded exact-source
pins, coverage and aggregate counts. It omits asset/target keys, IDs from histories,
locators, authored text, raw provider paths/messages and unbounded arrays. Single
publication claim, absent claim, ambiguous, untracked and unknown-coverage targets
stay distinct. A declared
receipt/readback/absence is not actual remote currentness or independent custody.
Payload/remote/semantic/custody/eviction flags stay false. Authored-schema success
applies only to the explicitly reported record, not every asset in the namespace.

L0 fmt/structure/diff and six-role review. L1 focused SDK/CLI admission, bound,
pin/schema/reader failures and prior resolver regressions; locked all-feature
workspace tests/strict Clippy; standalone Git/LFS and assets-only builds. L2 real
temporary Git plus existing-layout stores for media/non-media, dirty-worktree and
old commit isolation, unavailable source after capture, shared receipts, missing
history, refusal privacy and no writes. No production cache access or switch.
