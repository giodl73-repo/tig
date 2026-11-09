# Whole historical selections and original-binding pages

Experimental Rust metadata API: `tig_store::final_retirement::paged`. This first
layer supports the [paged-retirement contract](../../specs/PAGED-RETIREMENT.md).
The durable decision/page/claim/final-provider controller remains required.

Use this layer to freeze a large exact selection before independently deciding
what history and copies to retire. A bounded bulk batch preserves held bindings;
several batches can therefore keep one another held when shared content appears
in more than sixteen historical roots. A whole selection permits an honest
projection of the obligations that would remain after all selected history ends.
That observation does not authorize claim release or final deletion.

## Library sequence

1. Independently admit the metadata identity, complete source roster/current
   minima, owner/classes and authority/epoch. Open existing owned providers only.
2. Build each selected-binding Tree through existing Tree/NodeStore ports. Its
   exact paths and expected objects are checked against the actual historical
   source occurrence by compilation. No payload bodies need to exist in metadata.
3. `paged::request` writes small selection records and an immutable request header.
   A Selection names namespace, source, predecessor_revision and the binding Tree.
   Paging policy is frozen in that request; duplicate occurrences refuse.
4. `paged::build` validates the actual historical sources and projects complete
   admitted protection under one metadata/time budget and the intersection of
   private roster/local source limits. It emits a candidate manifest object.
5. Explicitly finish the owned metadata operation, reopen its read-only provider
   with the acknowledged minimum, then `paged::inspect` the full manifest. Adopt
   retention/custody and current authority independently before relying on it.
6. `paged::page` returns byte/count-bounded original-binding pages. Repeated calls
   with the returned cursor cover that fixed manifest; status/inspection perform
   no source/provider mutation or clock sampling.

Request and manifest headers, cohort/subject records and pages are canonical and
bounded to64KiB. Expanding indexes and binding sets live in asset-side Trees;
save only admitted small references in project Git. All writes are immutable
metadata. A failed builder may leave unacknowledged staged objects; the caller
owns that staging and must not implicitly finish another operation's work.

The initial profile caps4,096 selected occurrences and65,536 selected bindings/
objects, subject to the complete protection/read/write/time/provider budgets.
The full admitted closure, including hypothetically removed bindings, is charged.
Source, journal and provider capacity ceilings remain independent limits. A
limit failure refuses instead of truncating or resetting history.

## Read reports precisely

An occurrence is an exact namespace/source/predecessor/original root and context,
with admitted owner/class/policy. It is not a native asset count. Multiple native
assets may map to one occurrence; their meanings stay adapter-owned. Binding counts
include aliases; unique object/byte counts deduplicate immutable content. They are
not copy counts, file allocation, volume reclaim or proof that bytes are present.

`held_by_surviving_claims` records the hypothetical whole-selection observation.
An unselected alias, predecessor, current/pending root, job, retention obligation
or other admitted owner can keep content held. No remaining claim observed does
not establish provider eligibility, retained custody or deletion authority.
The inventory and every page deny claim-release/removal authority explicitly.

Inspection checks request/cohort/subject/count consistency and original binding
metadata. It reports historical data; a supplied digest/header does not authenticate
its own provenance. Before any future mutation, the independently trusted controller
must reconstruct the exact manifest against current privately admitted sources and
compare the root. A changed source vector or protection observation is a new
candidate requiring a new decision, never an amendment of old approval.

## Pages and later execution

A cursor binds one manifest and an actual original occurrence/path. Invalid
positions or another manifest refuse. It conveys an inspection position only;
skipped pages have no implied completion. The caller must retain acknowledged
execution progress separately through the future durable controller.

Pages name original bindings. After one subset transition retires an original
predecessor, later pages need a verified translation through the actual owned
survivor chain. They cannot be applied repeatedly to the original source. Prove
unchanged paths and actual transitions, preserve unselected members and inherit
age using [the age protocol](AGE-WITNESSES.md). Equal hashes cannot replace lineage.

The controller must acknowledge separate exact historical-claim and final-copy
decisions, page intents/plans, recovery admission and completion witnesses. It
must validate only its admitted command history under final live fences, preserve
original held exclusions and provider births/grants, stop on cancellation or stale
approval, and report partial effects honestly. Existing subset/final-provider/
accounted ports supply those primitives; this manifest API does not execute them.
