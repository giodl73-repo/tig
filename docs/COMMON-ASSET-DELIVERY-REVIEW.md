# Delivery belongs to the asset

Status: design proposal for review, not an implemented contract or migration.
The [landing plan](VERSION-DELIVERY-LANDING.md) adds production revisions,
selection/retention boundaries, bounded PRs and six-role closure obligations.
Baseline: Tig `1dfa6db6199a226661601e2e829dcbd9ad56c2f1`.
Scope: WP-010/011/015, REQ-001/006/010, SPEC-010/012/015,
IF-010/011/013 and PKG-006/008, following the existing publication-evidence slice.
No asset, provider, cache, producer binding or current receipt changes here.

## Rule

An asset keeps its actual type and stable identity when it is delivered.
Every asset type can carry the same optional delivery capability. A delivery
receipt is immutable evidence referenced by that object, not a new object type
and not a reason to create another editable selection.

A poem's Spanish text, English translation, recorded reading and printable PDF
can still be different real assets or explicitly declared streams. A movie and
its subtitle file have different bytes and independent revision histories.
Publishing one of them to another service does not create another text/movie/
subtitle asset. One asset can have many target observations over time.

Three identities must stay separate:

1. The current asset revision/representation.
2. The exact representation revision a particular target received.
3. The receipt and observations supporting that publication.

Creating a new asset revision changes the first; it does not rewrite the other
two. Different services can retain different revisions legitimately.

## What the current implementation provides

The design-1 envelope already has bounded named streams and an open metadata
map. Receipt blobs can already be pinned on an ordinary instance. Publication
checking already validates pinned receipts against explicit policies; the
retained-stream extension permits a published subject different from selected.
These mechanisms should be reused, with historical policies retained.

Current publication definitions combine file metadata, one target and receipt
selection in specialized authored schemas. That is not yet one shared lifecycle
contract for every asset type or an integrated multi-target publication history.
The generic native reader does not validate arbitrary metadata semantics.

Most existing publication instances select an actual delivered file, not the
receipt itself; their receipts are already asset-side stream references. Those
file identities must not be discarded merely because their definitions currently
describe publication. Inventory counts must inspect these distinctions first.

## Proposed common attachment

Use one versioned capability in the existing envelope, with one generic validator.
Do not add a Google Drive/Youtube/receipt variant of every domain definition.
Do not silently add new top-level fields to the strict design-1 envelope.

The following is a shape illustration; object pins are omitted:

```text
image / cue / scene / movie / text / score / other asset.tig
  id, definition, authored, selected, status
  metadata.delivery
    schema: tig.delivery-state.v1                 [PROPOSED]
    policy_slot: delivery-policy
    history_slot: delivery-history                [absent before history exists]
  streams
    delivery-policy  -> pinned common delivery policy
    delivery-history -> pinned immutable history root
```

The policy can be shared by many objects. It declares target keys, representation
selectors, profile/dimension requirements and publishing gates. Domain fields
remain governed by the object's original definition. No delivery configuration
or receipt is mandatory for an asset that has never been configured for delivery.
Missing history means unknown/not tracked unless coverage is explicitly known;
it must not be described automatically as never delivered.

The common validator checks the attachment and role/slot pins independently of
the domain-authored schema. Merely placing a schema label in metadata grants no
capability or evidence. An admitted publisher must validate the common contract,
exact policy and subject, plus the relevant domain operation's eligibility.

## Immutable history, small Git files

Each history event identifies:

- a stable operation/event identity and its causal predecessors;
- the logical asset and an explicit selected/stream representation selector;
- the exact published object hash and length, even if selected has since changed;
- the resolved variant/profile pins relevant to that representation;
- the target key and retained provider identity/version evidence;
- the result stage: attempted, acknowledged, read-back checked, failed, or a
  later observation such as target absent; and
- pinned receipt evidence, with an explicit selector/policy for a shared receipt.

The publishing policy separates intended delivery from those observed results.
A failed attempt does not replace the last acknowledged/read-back target state.
Provider acknowledgement, byte readback and creative approval remain distinct.
Git merges can change desired state or enqueue configured work; actual transport
still requires an admitted operation and its policy/authority gates. A receipt
or policy annotation alone does not authorize a network publication.

History and raw receipts belong in existing immutable asset storage. Use bounded
history/index pages through Tig's object/tree or admitted index interfaces. The
asset file changes only the small history-root pin and any intentionally changed
policy pin; it never grows an unbounded receipt/event array. One receipt can
provide evidence for many subjects without duplicating its bytes or creating a
receipt asset record for each row.

The history format/adapter must declare a complete reference closure for its
pages, receipts and retained published subjects. A root pointer alone is not
custody coverage. Unimplemented or incomplete traversal blocks reclamation.
History retention is not hot-cache demand: publication/evidence edges can require
archive/history custody without keeping every old subject in a working tier.
Placement policies must distinguish current working/draft requirements from
historical retention when computing store obligations.

An event can reference a pre-publication Git checkpoint and producer subject.
It must not require the hash of the final asset record that contains that event's
history root: that would be a circular self-hash. Receipt bytes bind the payload
and target; the event can then pin the receipt without a hash cycle.

## Example: one movie, two services, two revisions

| Change | Asset selected | Review target | Public-video target |
| --- | --- | --- | --- |
| Produce v1 | A | unknown | unknown |
| Deliver v1 for review | A | A, receipt R1 | unknown |
| Produce v2 | B | A, R1 retained | unknown |
| Deliver v2 for review | B | B, R2 current; A/R1 history | unknown |
| Explicitly publish retained v1 publicly | B | B/R2 | A/R3 |
| Remove the review file remotely | B | observed absent; R1/R2 retained | A/R3 |

The asset ID never changes. There are no movie-v1-drive-receipt.tig or
movie-v2-youtube-receipt.tig instances. The final row grants no permission to
delete A, B or their evidence from a cache. Choosing an older approved public
version is a publishing policy decision, not an implicit selection rollback.

## Review against three asset shapes

| Asset | Representation delivered | Natural receipt/history attachment |
| --- | --- | --- |
| Cel | Its exact image stream | Same cel ID and definition; target observations bind that image revision |
| Cue | Its declared language/performance audio stream | Same cue; exact audio subject and language/profile pins, with no inferred text/voice approval |
| Episode or poem work | A declared output, text or PDF representation | Receipt belongs to the actual byte-bearing asset/stream, not to an unrelated container JSON |

A new PDF or audio export may need a normal output asset because it is a real
independent artifact. It does not need a special receipt-object type. A report
someone deliberately authors and shares is likewise an ordinary asset; transport
acknowledgements and audit rows remain referenced evidence by default.

## Cases that must hold

| Case | Required behavior |
| --- | --- |
| One image, two delivery targets | One image asset/object; independent target histories |
| Spanish and English outputs | Explicit representation identities; no cross-language receipt substitution |
| New cue text, old recording still selected | Domain currency remains separate; a receipt cannot certify a fresh performance |
| A new receipt on an unchanged movie | Publication state changes; its rendering recipe stays unchanged |
| Deliver a previously retained revision | Pin that published subject without changing today's selected bytes |
| Two branches deliver to different targets | Preserve both event branches; do not choose only one history root |
| Two branches deliver to the same target | Use provider version/causal evidence; ambiguous current state requires a named-target check |
| Retry after a timeout | Preserve operation identity and truthful outcomes; do not invent a second asset or hide uncertain publication |
| Target rename or deletion | Keep stable provider identity/history; no cache deletion follows |
| Receipt cannot be interpreted | Retain its exact bytes and unverified status, with no inferred successful delivery |
| History grows to thousands of events | Small Git attachment; bounded asset-side traversal/paging |

Git merge order and wall-clock timestamps are not remote-version authority.
Same-target writes need provider-supported conditional updates where available;
otherwise the observed state stays uncertain until checked. Independently issued
real publications are not deduplicated solely because their payload hashes match.

Delivery edges are publication/evidence relationships, not production inputs.
Render recipes must exclude the delivery ledger and raw receipts. A publishing
operation does consume its publishing policy and exact subject. Existing editing
locks that bind whole record bytes may legitimately need refresh or a semantic
rebase after a concurrent delivery update; that is not a reason to rerender.

## consumer project migration review

Keep real file assets, with their own domain types. Move their existing receipt
pins and retained publication subjects into the common history attachment.
Unify target-specific wrappers only after proving the same logical asset and
representation; identical bytes or filenames alone do not prove identity.

Preserve old IDs/subjects/receipts and compatibility lookups throughout any
transition. Do not create dangling aliases that today's resolver cannot follow.
Where no normal asset exists for an actual delivered file, create that real
file asset first. Preserve historical-only/unknown target states explicitly.

Future inventory views must distinguish works, representations, published files,
targets, events and evidence blobs. A subtitle or receipt count must never be
presented as a count of films or poems. Existing source lane handoffs can proceed
without waiting for a mass conversion of historical publication records.

## Producer role review and bounded next implementation

Identity/namespace: stable asset IDs, exact published subjects and no circular
hashes; preserve legacy IDs until tested resolution exists. Concurrency/recovery:
event unions, retry identity and conditional same-target updates; no global lock.
Storage/custody: bounded attachment and complete history/receipt closures, with
delivery explicitly granting no eviction authority. Privacy: provider IDs and
raw evidence stay in private asset storage; public summaries need their own
redaction policy. Git/adoption: compatible metadata attachment, separately admitted
semantics and honest draft status; domain definitions and original approvals stay.

These are producer-applied design findings, not independent approval or executed
scenario results. All listed behavior beyond the existing publication primitives
remains an implementation/verification obligation.

The next bounded slice is one shared pure delivery-contract validator/history
operation over synthetic image, cue and document objects, reusing the existing
receipt policy/parser and selected/retained subject checks. Follow with one
metadata-preserving consumer project mapping example. Do not introduce provider/network
operations, a new cache engine or a wholesale asset rewrite in that slice.
