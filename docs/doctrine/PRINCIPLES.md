# Tig principles

Active decision rules, not proof that the intended behavior is implemented.
Stable scenario-to-doctrine links are in tests/scenarios/learning-links.json.

## TIG-P-01: Separate intent and reality

**Decision rule:** Treat desired, planned and observed states as distinct.

**Rationale:** A committed selection does not make a transfer complete.

**Evidence:** T07/T08/T09/T10/T23/CL02/CL03/GW12/SP07/BN07/BN08/LC02/LC05; designed only.

**Status:** ACTIVE

## TIG-P-02: Content survives location changes

**Decision rule:** Key immutable content independently of names, physical paths and transport representations.

**Rationale:** Reuse and history need stable exact identity while locations change.

**Authoring implication:** Preserve independent leaf selections in Git; immutable aggregate roots must not force unrelated edits through one authored file.

**Evidence:** T04/T05/T18/T22/GW13/CR09/SP02/SP03/SP04/SP06/BN03/BN05/BN09/LC10; designed only.

**Status:** ACTIVE

## TIG-P-03: Protection composes across owners

**Decision rule:** Union all admitted branch, job, draft and retention obligations; retire only owned claims.

**Rationale:** A local lifecycle event cannot release another consumer's protection.

**Evidence:** T02/T03/T06/T11/T19/T20/T24/CL10/GW08/GW09/GW10/GW15/CR06/CR07/CR08/RW04/RW06/RW10/LF06/LF07/LF08/LF09/SP10/BN04/BN10/LC03/LC06/LC07/LC08; designed only.

**Status:** ACTIVE

## TIG-P-04: Unknown is explicit

**Decision rule:** Carry scope, integrity and freshness gaps; never treat unknown as absent or safe.

**Rationale:** Incomplete positive evidence is not proof of absent demand or present custody.

**Evidence:** T02/T03/T04/T05/T07/T08/T09/T10/T17/T23/CL01/CL02/CL03/GW12/CR09/RW04/LF06/SP06/SP07/SP10/BN01/BN02/BN03/BN05/BN07/BN08/LC02/LC05/LC07/LC10; designed only.

**Status:** ACTIVE

## TIG-P-05: Durable protection precedes discovery

**Decision rule:** Persist admission and operation identity before exposing new bytes or shrinking old obligations.

**Rationale:** A rebuilt lookup catalog cannot reconstruct unrecorded active work reliably.

**Evidence:** T06/T11/T14/T19/T20/CL05/CL10/GW03/GW04/GW05/GW08/GW09/GW10/GW15/CR01/CR02/CR03/CR04/CR08/CR10/LF07/LF08/LF09/SP05/BN04/BN10/LC01/LC03/LC06/LC08; designed only.

**Status:** ACTIVE

## TIG-P-06: Concurrency is scoped and fenced

**Decision rule:** Coordinate overlapping object/place actions and reject stale operations without pausing unrelated writers.

**Rationale:** Readers, claims and publishers can change while transfer IO is in progress.

**Evidence:** T12/T13/T24/CL04/CL06/CR05/CR06/CR07/RW01/RW02/RW06/RW07/RW09/RW10; designed only.

**Status:** ACTIVE

## TIG-P-07: Integration preserves authority and protocol meaning

**Decision rule:** Use admitted adapters, exact source context and truthful capability/completion semantics.

**Rationale:** Convenient filenames, pointers or provider responses cannot grant hidden authority.

**Evidence:** T16/T21/T25/CL07/CL08/GW01/GW02/GW06/GW07/GW11/RW05/RW08/LF01/LF02/LF03/LF04/LF05/LF10/BN06/LC04; designed only.

**Status:** ACTIVE

## TIG-P-08: Measure bounded physical costs

**Decision rule:** Budget peak output/scratch and count real copies; measure reclaim and reuse.

**Rationale:** Logical sharing alone does not establish allocated-space savings or transfer efficiency.

**Evidence:** T01/T15/T17/T18/T22/CL01/CL07/CL08/CL09/GW01/GW06/GW07/GW11/GW13/GW14/RW03/LF01/LF02/LF04/LF05/SP01/SP02/SP03/SP04/SP08/SP09/BN01/BN02/BN09/LC04/LC09; designed only.

**Status:** ACTIVE

## TIG-P-09: Preserve acknowledged bytes by default

**Decision rule:** Make preservation of acknowledged managed content the default
across writes, replacement, movement, cleanup and recovery. Refuse destructive
actions without verified protection and current admitted authority. Disclose the
actual achieved durability/replication level; do not imply a backup from a hash.

**Rationale:** Users must be able to work without wondering whether a routine
commit, merge, crash or cache cleanup will silently discard their only good bytes.

**Evidence:** T02/T03/T08/T09/T14/T24, CL05/CL06, GW04/GW05, CR01-CR04,
RW01/RW02, LF04, SP04/SP05/SP07/SP10 and LC01/LC02/LC07 are designed obligations.
The full scenario linkage is in tests/scenarios/learning-links.json. Existing
13 pure tests do not prove durable storage or loss-free execution.

**Acceptance:** Fault-injected publication/recovery, reader-fence, independent
custody and explicit final-deletion fixtures must demonstrate recoverability.
See docs/SAFETY-PROMISE.md and M01/M03/M04/M05/M07/M08/M11.

**Status:** ACTIVE
