# Receipt member-key targets: producer review

Controls/entry: [PUBLICATION-EVIDENCE](../../specs/PUBLICATION-EVIDENCE.md),
WP-010/011/015, REQ-001/006/010, IF-010/011/013, PKG-006/008;
source `90be465`, now qualified as main `3ff0b24` with identical whole tree.
Allowed change: explicit target extraction from an object member key. No receipt
rewriting, target inference, provider operation, approval or runtime adoption.

The older mapping required a target string inside every object row. Some retained
receipts instead store `{path: {sha256,bytes}}`. `fields.target` now accepts the
strict marker `{"kind":"member-key"}` for object rows only. Existing pointer
strings keep their behavior. Unknown/missing/null markers, additional marker
fields and array-row use refuse. Every subject hash and declared length check,
immediate-member selector rule, parser budget and portable path rule remains.

Four new tests plus the existing CLI case expanded to both representations pass:
20 publication cases in 0.61 s. They check stable target identity across row
representations, exact policy pin differences, escaped keys, literal percent
encoding, unsafe paths, malformed selectors, bad hashes/lengths, explicit marker
admission and absence of row-path fallback. Optional omitted length stays
unasserted. The actual CLI emits no private key, title, scope or record ID and
creates no additional files. Full workspace and hosted qualification are separate.

Producer-applied lenses: identity requires exact provider/scope/path and policy
pins; provider/privacy validates decoded keys exactly once and preserves fixed
errors; Git/adoption keeps large source receipts immutable outside Git; storage/
custody does not upgrade row correspondence into current availability or backup;
recovery/compatibility preserves pointer policies and refuses unsupported markers
on older tools. No independent review or human publication approval is implied.

Disposition: pass_with_risk for this extraction extension. Actual consumer
acceptance and exact-head hosted checks remain separate gates. Complete delivery
coverage, current remote state, payload readback, custody and eviction remain
outside the checker. A container or catalog count never establishes per-file
delivery coverage; unresolved target mappings stay unresolved.
