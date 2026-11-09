# Explicit roster-wide history precedence

Input106f6f524259fa15ecb487f9e808665fad723d1f; review finding PL-01.
WP-004/006/007/009; REQ-002/003/004/005/009; SPEC-004/008/009;
IF-004/006/012; PKG-002/003/008. RUST/PROVIDER profiles.
Allowed: fresh explicit synthetic store bindings, bounded source projections,
existing ordinary placement/retirement fences and compatibility fixtures.
No live enrollment/upgrade, authority migration, final-deletion relaxation or
new cache/journal. Entry: immutable source policy and exact complete store roster.
Exit: planner and live guarded assessment agree on cross-namespace active use,
job withdrawal restores history, hard/additive demands survive, old writers
refuse the new binding and legacy behavior/identity remain unchanged.

## Explicit enrollment

Binding::new retains namespace-local precedence and its existing canonical bytes
and hash. Binding::new_roster adds history_scope:"roster" to the immutable binding
and explicitly admits all of its members as one advisory precedence group.
Absence/default scope is namespace; unknown fields/values refuse. Existing roots
are never modified or re-enrolled. New roots use existing bounded provider ports.
Old strict binding readers reject the new field. Already-open old writers, live
migration and rollback/latest-authority protection remain separate adoption gates.

Placement enrollment design-1 remains namespace-local. Design-2 explicitly selects
roster-wide precedence and a distinct enrollment identity. Every protected provider
must use roster scope, and every removable cache binding must contain the exact
complete configured namespace roster and its actual physical-place mappings.
Mixed profiles and incomplete group enrollment refuse before planning/effects.
Namespace aliases never become global place names; each assignment is translated
through its admitted namespace-to-provider mapping after precedence resolution.

## One rule for planning and assessment

Collect raw active/history/retained demand buckets from every admitted namespace,
under one aggregate existing Work budget. A branch/draft/job current or pending
use anywhere in the group suppresses advisory predecessor demand in namespaces
whose immutable policy has prefer_active enabled. Explicit prefer_active:false
keeps that namespace's history additive. Retention-class demand and active demand
are always additive; other projects cannot weaken them through membership.
All roots/aliases still resolve, even when their advisory history is suppressed.
Unknown/missing/corrupt closure, conflicting sizes and resource limits hold.

Read-only planning captures exact checkpoints, then rechecks them after collection;
it grants no absence or deletion authority. Store assessment acquires all existing
namespace subject fences and the provider exclusive guard first. The same raw
bucket rule then resolves absence for each mapped alias. Shared admission already
fences raw bucket changes, so another namespace cannot withdraw the active use
justifying suppression while removal holds the assessment. Unrelated partitions
remain concurrent; no global cache lock or SQL transaction spans provider IO.
Existing ordinary retirement still requires independently retained keepers and
durable exact removal intent. Final content-wide membership clearance is unchanged.

L0: fmt, reviewed hashes, portable validator, diff and six producer lenses.
L1: focused default/roster/additive/mixed-profile, locked workspace default/all-
feature, strict Clippy and standalone Git. L2: actual three-store historical
copy -> remote job start -> working reacquisition/history eviction -> job end ->
history reacquisition/working eviction; live remote withdrawal contention,
incomplete scope and explicit pinned pre-change artifact refusal/legacy control.
Full native/real provider/large-file/lifetime/authority and human acceptance stay
open. This is an explicit compatible profile, not an implicit migration.
