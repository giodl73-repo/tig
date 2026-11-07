# Opaque publication locators: producer review

Entry `72bd1edcb37744d394052e736d8dbbe3090eba1c`, qualified with identical
whole tree on main `a35f48fb12eeedea36c430cb226f01a59762cf9d`.
Controls: WP-010/011/015, REQ-001/006/010, SPEC-010/012/015,
IF-010/011/013, PKG-006/008; RUST + INTEGRATION profiles.
Allowed scope: explicit literal locator extraction from retained metadata.
No provider IO, receipt rewriting, normalization, runtime switch or approval.

Historical receipts may name a Windows mount address, URI or provider file ID.
The explicit opaque-locator marker admits these as bounded uninterpreted strings
in a separate identity domain. Existing logical-path modes retain their strict
validation. No fallback or inferred alias connects the domains. The result emits
only a target digest, never the private address or a usable transport command.

Four additional cases and the expanded three-shape CLI/privacy test pass:
24 publication cases in 0.93 s. They cover exact spelling/domain separation,
Windows addresses/URIs/IDs, strict marker/pointer/shape admission, control/blank
refusal, 4,096-byte UTF-8 boundaries, object/array rows, exact subjects/lengths
and unchanged logical-path refusal. No tests were removed, ignored or weakened.
Full locked all-feature workspace tests, strict all-target all-feature Clippy
and standalone Git/LFS check pass. Hosted qualification and actual consumer
receipt checks remain separate gates; synthetic tests contain no private media.

Producer-applied lenses and disposition:

- Identity/namespace: domain separation prevents equal-looking path/locator
  strings from aliasing. No case, slash, percent or Unicode normalization.
- Provider/privacy: locator bytes never reach filesystem/network operations;
  fixed errors and CLI output remain redacted. The locator is not authenticated.
- Git/adoption: source receipts remain immutable asset-side; policies carry only
  extraction instructions. Older binaries refuse the marker.
- Storage/custody: row correspondence cannot establish availability, retained
  protection, independent custody or eviction authority; existing false flags stay.
- Recovery/compatibility: malformed explicit markers refuse rather than selecting
  a different target mode. Existing pointer/member-key policies retain behavior.

Disposition: pass_with_risk for this bounded extension. Owner: Tig typed-asset
integration. No independent review or human publication acceptance is claimed.
Actual provider reconciliation, all-family delivery completeness, production
adoption and safe removal remain open. Exit: recorded local checks, exact-head
hosted gates and separately pinned consumer acceptance before adoption.
