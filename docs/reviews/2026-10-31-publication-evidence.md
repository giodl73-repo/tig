# Publication evidence binding: producer review

Public summary: consumer identifiers generalized; original notes retained privately.

Controls and limits: [PUBLICATION-EVIDENCE](../../specs/PUBLICATION-EVIDENCE.md),
WP-010/011/015, REQ-001/006/010, IF-010/011/013, PKG-006/008. The optional
`tig-assets` layer owns retained evidence semantics; Git/LFS and storage remain
independent. No provider or consumer binding changes.

The exact schema-checked instance is required before a caller-admitted mapping
can compare its selected payload to one row in its hash-pinned receipt. Array,
keyed-object and hash-only mappings have no field-name/filename inference.
Hash-only rows explicitly do not assert a receipt length. Parsed evidence can be
reused with each checker's own pin and actual structural limits enforced.

Sixteen focused tests pass, including synthetic invoice/font consumers, actual
CLI, source/role/pin/target/schema negatives, missing/null values, escaped and
invalid selectors, duplicate keys, finite decimal metadata, hard/declared bounds,
cross-policy reuse and a 1,000-row external receipt. Full locked all-feature
workspace tests, strict workspace/all-target/all-feature Clippy and standalone
`tig-git --no-default-features` check pass. No existing tests are removed or skipped
to qualify this change. Native identity JSON is unchanged.

An owner-run local candidate also checks all 79 existing consumer project poem publication
anchors against their exact retained receipt: 79 unique declared targets, 13
length-bearing rows and 66 hash-only rows. Raw inputs/results remain in the
consumer's private storage; no private assets or evidence content enter Tig.
These checks read metadata only and do not refresh any past producer, approval,
payload availability, remote identity, current sync or independent custody.

Producer lenses: identity/namespace requires exact instance/schema/policy/receipt
pins and stable declared target identity; provider/privacy keeps target values
out of reports and fixed CLI errors; storage/custody keeps availability, cloud
state and removal flags false; Git/adoption keeps large evidence asset-side and
the standalone consumer independent. Severity: high if relationship success is
misread as remote delivery/custody. Mitigation: explicit false fields, private
parsed data, two consumer tests and caller-owned trust/freshness policy. Owner:
typed assets / consumer integration; remaining remote and complete coverage gates
stay open. Disposition: pass_with_risk for the bounded checker only; not
independent acceptance, full requirement closure or production authorization.

The [portable replay feedback triage](../../feedbacks/README.md)
distinguishes this work from the next project-restore orchestration slice. This
checker does not implement batch copying, retry journals, host dependency
discovery or codec interpretation.
