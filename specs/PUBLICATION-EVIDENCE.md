# Retained publication evidence binding

WP-010/011/015 bounded slice; REQ-001/006/010, SPEC-010/012/015,
IF-010/011/013, PKG-006/008, RUST + INTEGRATION profiles. Entry source is
`46feb03a434ef2b7226884194359273ddcb00b6d`; documentation-only feedback main
`cb40af0` is integrated without changing that code baseline. Allowed changes: optional pure
`tig-assets` checker, explicit read-only diagnostic CLI, synthetic fixtures and
trace/review documentation. No provider, placement or production binding changes.

An instance/definition/schema binding and successful authored-schema execution
must precede evidence evaluation. A caller-admitted policy identifies the authored
target object and the payload stream. The target names a provider, logical scope,
receipt stream slot and JSON Pointer to one row in that exact receipt object.
The selected subject must equal the declared payload stream under the default
subject policy. No schema default, filename, timestamp or alternate stream
supplies a missing subject. The explicit retained-stream extension below keeps
publication observations independent of a later source selection.

Policy rules explicitly admit array rows, keyed rows or keyed hash-only entries.
Object rows map exact relative pointers for SHA-256, optional length and target
path. Hash-only entries use their decoded member key as target path; a match
does not claim receipt-asserted length. Rules have disjoint selector roots and
selectors must identify exactly one immediate member. Reject malformed escapes,
noncanonical array indices, ambiguity, missing/null values, bad targets and
subject/pin differences. Relative target paths cannot escape their logical scope.

Receipt bytes must match the exact declared stream SHA-256 and length before
parsing. External evidence may contain finite decimal metadata; native identity
documents keep their strict canonical rules. Reject duplicate keys at all depths,
non-finite numbers and exceeded caller limits. Hard ceilings are 16 MiB receipt,
262,144 JSON nodes and depth 32. Policies and native documents remain 64 KiB.
No raw evidence is rewritten, normalized into an identity, fetched or published.
An immutable parsed receipt can be reused across anchors. Each subsequent check
still enforces its own exact receipt pin and actual byte/node/depth limits; reuse
cannot broaden a stricter policy or substitute another receipt.

The result binds instance, definition, schema, policy, selected payload and receipt
and exposes an opaque logical-target identity. It confirms only the retained
receipt relationship. Remote file identity, current publication, payload readback,
independent custody, approval and eviction authority remain unverified. Diagnostics
must not expose target paths, titles, document fields or raw parser errors.

Exit evidence: L0 fmt/structure/strict Clippy; L1 positive/negative generic tests
for two non-media consumers, row shapes, pins, parser/pointer/budget boundaries,
schema prerequisites and privacy; L2 actual explicit CLI with synthetic inputs,
fixed failures and no unexpected files. Full migration/custody/remote proof and
producer/independent acceptance remain open. The existing Git/LFS-only build must
remain independent of the optional asset layer.

## Explicit local command and policy

```text
tig-publication INSTANCE DEFINITION AUTHORED_SCHEMA 2020-12 POLICY RECEIPT
```

Build with `cargo build --locked -p tig-assets --features schema --bin tig-publication`.
All six arguments are explicit local inputs except the dialect. The command
reads no payload or provider and writes only its local result to stdout. Errors
are fixed and omit paths, selectors and raw parser details. Result object pins
are local provenance, not a public/shareable receipt or source trust assertion.

The admitted policy has schema `tig.publication-evidence-policy.experimental-1`:

```json
{
  "schema": "tig.publication-evidence-policy.experimental-1",
  "target_pointer": "/target",
  "subject_slot": "payload",
  "subject_role": "output/document",
  "receipt_role": "evidence/publication",
  "limits": { "bytes": 1048576, "nodes": 8192, "depth": 32 },
  "rules": [
    {
      "kind": "array-rows",
      "root": "/rows",
      "fields": { "sha256": "/sha256", "bytes": "/bytes", "target": "/path" }
    }
  ]
}
```

The authored target object requires exactly `provider`, `scope`, `receipt_slot`
and `receipt_selector`. Scope is a caller-declared logical namespace, not an
enrolled remote identity. Rules may use `object-rows` with the same field map,
or `keyed-hashes` with only `root`. Pointers in `fields` are relative to the
selected row; an empty pointer selects that whole value. Omitted/null policy
`fields.bytes` explicitly requests no receipt-length assertion. If a length
pointer is declared, an absent/null/fractional/string value refuses.

`receipt_selector` selects one immediate member below a declared root. Array
indices use canonical unsigned decimal spelling. JSON Pointer escaping is
decoded exactly, without URI decoding or Unicode/path normalization. Targets
use Tig's portable `LogicalPath` admission. The opaque declared-target digest
uses provider/scope/target path, not the receipt revision; it is not a verified
provider file ID. Multiple revisions can describe the same declared target.
No one-record success establishes complete shelf coverage or chooses which
receipt is current. The caller must admit policy/trust and reconcile coverage.

## Explicit retained subject independent of current selection

Entry `d3b8db1d649f813e1640fb515ded969724b9ce5a`; WP-010/011/015,
REQ-001/006/010, SPEC-010/012/015, IF-010/011/013, PKG-006/008.
Allowed scope: the optional pure checker, synthetic API/CLI cases and its
contract/review records. No provider, placement, rendering or runtime adoption.
L0: structure/fmt/strict Clippy. L1: selected/default compatibility, different or
absent current selection, target separation, exact slot/role/receipt and malformed
policy refusals. L2: explicit CLI with synthetic metadata and private-safe output.
Exit requires those checks, exact-head hosted qualification and separate later
consumer acceptance; implementation alone closes no migration/custody gate.

An optional policy field `subject_selection` admits exactly `selected` (the
default) or `declared-stream`. Null and unknown values refuse. In default mode,
the existing selected-subject requirement and serialized report remain unchanged.
In `declared-stream` mode, the exact declared `subject_slot` and `subject_role`
identify one retained object even if the current selection differs or is absent.
No fallback to selection, another stream or a receipt-supplied hash occurs.

The report in explicit stream mode adds `subject_selection: "declared-stream"`
and `subject_is_selected`, an exact object equality observation at this pinned
instance. False does not imply that the publication is obsolete, unapproved or
safe to remove. True does not establish production freshness or current remote
contents. Exact schema binding, receipt pin/row/length and all unverified custody,
provider, approval and eviction boundaries remain required and unchanged.

For example, source revision B can remain selected while a reviewer target still
retains A. Its publication stream stays pinned to A; the checker verifies A's
receipt and reports false without changing B or pretending A was republished.
Two target policies can select different retained streams in the same instance.
The target identity remains provider/scope/literal path, independent of the
published object's revision. Historical streams remain explicit byte obligations;
this checker does not choose placement, retention or a newest publication.

## Explicit member-key target extension

Entry source `90be4657be0839de2163dc64acb00833b5ffca07`, WP-010/011/015,
REQ-001/006/010, SPEC-010/012/015, IF-010/011/013, PKG-006/008; RUST +
INTEGRATION profiles. Scope: bounded pure target extraction for keyed object
receipts, synthetic policy/CLI cases and later owner-run metadata acceptance.
No provider operations, payload/remote proof or runtime/selection change.
L0 format/foundation/lint; L1 explicit-key admission, escapes, unsafe paths,
subject/length mismatch and compatibility; L2 synthetic CLI plus separately
pinned consumer evidence. Hosted qualification remains a separate exit gate.

For an `object-rows` rule only, `fields.target` may explicitly be
`{"kind":"member-key"}` instead of a relative JSON Pointer string. This admits
the selected immediate member's decoded key as its target path while SHA-256
and optional length still come from the row. For example:

```json
{
  "kind": "object-rows",
  "root": "/files",
  "fields": {
    "sha256": "/sha256",
    "bytes": "/bytes",
    "target": {"kind":"member-key"}
  }
}
```

This supports receipts shaped as `files: {"Reports/item.pdf": {sha256,bytes}}`
without rewriting source evidence or inferring a filename. Missing/null target,
unknown marker kinds/fields and array-row use refuse at policy admission.
Pointer strings keep their existing meaning. Selector escaping is decoded once;
the resulting key must pass the same portable target-path validation as a row
field. No fallback to a row's path property, URI decoding or path normalization.
Equivalent provider/scope/path values retain the same declared-target identity
across row representations; the exact evidence policy pin still distinguishes
which extraction was admitted. Existing parser budgets and false custody/remote/
eviction flags remain unchanged. Older tools refuse the new marker, never ignore it.

## Explicit opaque locators

Entry `72bd1edcb37744d394052e736d8dbbe3090eba1c`; WP-010/011/015,
REQ-001/006/010, SPEC-010/012/015, IF-010/011/013, PKG-006/008;
RUST + INTEGRATION profiles. Scope: pure, explicitly admitted literal provider
address binding, synthetic schema/identity/privacy tests and later owner-run
receipt acceptance. L0 format/foundation/lint; L1 admission/bounds/domain vectors;
L2 metadata-only CLI. Full workspace, hosted and consumer gates stay separate.

An array/object row may use
`fields.target: {"kind":"opaque-locator","pointer":"/path"}`. The row value
must be a nonblank string of at most 4,096 UTF-8 bytes without control characters.
Its exact bytes are an uninterpreted provider locator: a file ID, URI or historical
mount address. It is **not** a Tig LogicalPath and never becomes a filesystem,
network, materialization or transfer argument. The checker still opens only the
explicit local metadata inputs and emits no locator value.

No slash/case/Unicode/URI normalization, prefix stripping, alias inference or
credential/provider interpretation occurs. Unknown/extra marker fields, bad
pointers and nonstring/missing values refuse. Hash/length/selector/pin rules stay
unchanged. Pointer-string and member-key targets retain strict relative-path
validation; absolute locators never enter those modes by fallback.

Locator identities use the distinct
`tig.declared-publication-locator.experimental-1` domain with provider, scope and
literal locator, separate from logical publication path identities. Equal-looking
strings in different domains are not aliases. Different mount spellings remain
different unverified locators until an explicit provider reconciliation supplies
stronger evidence. This proves retained row correspondence, not address safety,
reachability, provider identity, current publication, custody or removal authority.

## Explicit singleton object rows

Entry `dae494d9bac18456020e90043b2764e9225c1d6e`; WP-010/011/015,
REQ-001/006/010, SPEC-010/012/015, IF-010/011/013, PKG-006/008;
RUST + INTEGRATION profiles. Allowed scope: pure checker/explicit CLI, synthetic
tests and trace/review. No provider, cache, production-binding or subject changes.
L0 format/foundation/lint; L1 exact root/nested selectors, nonobject and ambiguous
policy refusal, subject/length/target/pin/budget/privacy compatibility; L2 original
synthetic root receipt through the compiled CLI. Hosted and consumer acceptance
remain separate exits.

A `single-row` rule admits exactly one JSON object at `root`, using the same
relative `fields` map as collection rows. Its `receipt_selector` must equal that
root, rather than selecting an immediate child. Empty root and selector select
the receipt object itself under JSON Pointer rules; an empty string is distinct
from `/`, which selects the member with an empty key. The caller's authored
schema must explicitly permit the chosen selector. No schema is bypassed.

Only objects qualify. No array auto-wrap, descendant fallback or native evidence
rewrite occurs. Member-key targets refuse because a singleton has no selected
collection member; pointer-string and explicit opaque-locator targets retain
their previous domains and validation. All rule roots remain disjoint, so a root
singleton cannot coexist with another rule under the same receipt. Existing
collection rules still require an immediate child. Exact evidence and policy
pins, parser budgets, fixed errors and all false authority flags remain intact.
