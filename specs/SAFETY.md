# Tig safety contract

Status: full-system target, not verified. A bounded local publication subset is
implemented under specs/PUBLICATION.md; other obligations remain open.
Owners: storage/custody and recovery roles.

Governing principle TIG-P-09 and invariant TIG-I-01: preserve acknowledged managed
bytes by default. Publication, replacement, movement and recovery must not silently
overwrite content or make it unrecoverable. Explicit admitted final-retirement
policy/authority is distinct from ordinary replica eviction. The full contract
and durability levels are in [the safety promise](../docs/SAFETY-PROMISE.md).

1. Verify exact destination bytes before retiring a protected source copy.
2. Check all current claims, jobs, authorized drafts and retention obligations.
   Unknown coverage is not zero demand. Never infer independent custody from
   drive letters, symlinks, peer counts or a file-name-only index.
3. Revalidate authority, claim/policy generation and custody immediately before
   removal using an appropriate provider/local fence. A stale dry run cannot apply.
4. Serialize only overlapping destructive scopes; parallel additions must remain
   possible without a laptop-wide cache/render lock.
5. Reserve bounded output, scratch, network and capacity. Refuse or defer work
   that exceeds limits. Do not build full duplicated bundle staging directories.
6. Persist retryable intent/completion records; never report partial files as
   complete. Crashes and cancellation preserve known-good retained data.
7. Distinguish availability from retained custody, eviction from archival deletion,
   logical bytes from allocated bytes, and transfer success from actual reclaim.
8. Imported paths/metainfo cannot escape declared roots. No arbitrary executable
   hooks or credentials in Git, logs or transport descriptors.
9. Providers declare capabilities; unsupported integrity, fencing or retention
   cannot be silently emulated as a successful safety check.
10. Automatic apply is explicitly enabled under bounded policy. Initial default
    is inspection/dry run. No provider runs merely by opening a repo or branch.
11. Treat retained replica count/failure-domain requirements separately from
    proximity. Keep initial ingest protected while redundancy is pending; a Git
    commit/push/merge event alone is not evidence that another copy exists.
12. Catalog restore, lost acknowledgements, uncertain lease expiry and external
    pruning need explicit recovery/fencing contracts; do not infer safe removal
    merely from a timeout, pointer inventory or absent journal row.

Historical migration evidence may be admitted only through explicit provenance,
trust and freshness policy; existence in an old index is not universally proof
of present independent retained custody.
