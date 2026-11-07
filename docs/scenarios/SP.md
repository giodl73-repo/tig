# Space, representations and place identity

Producer-applied design walkthrough, not an executed test. Each row starts with
its stated condition, applies the next step and checks the required consequence.
All mitigations are planned; references link to the common doctrine and mitigation
register. See [learnings](LEARNINGS.md).

| ID | Starting condition | Next step | Required consequence | Pitfall / mitigation |
| --- | --- | --- | --- | --- |
| SP01 | A sparse file reports a large logical size but occupies few blocks | Estimate transfer and source-space reclaim | Distinguish logical transfer size from measured allocation; do not invent savings | TIG-F-02 / M02 |
| SP02 | Two paths are hard links to the same underlying data | Count copies and edit one path | Do not count independent custody; prevent mutation of shared immutable content | TIG-F-11 / M11 |
| SP03 | Compacting a shared pack requires more temporary space than is free | Plan removal of obsolete members | Keep pack pinned and report blocked capacity or a separately budgeted compaction | TIG-F-11 / M11 |
| SP04 | An encrypted object changes storage keys without changing plaintext | Rotate representation while retaining old readers | Preserve plaintext identity and key/representation access until protected readers finish | TIG-F-11 / M11 |
| SP05 | A cross-volume copy is interrupted before source retirement | Restart transfer with a partially allocated destination | Resume/revalidate within budget; source remains until destination obligations are satisfied | TIG-F-03 / M03 |
| SP06 | Case folding or Unicode normalization makes two logical paths collide | Materialize a bundle on a less expressive filesystem | Reject or use a declared reversible mapping; no silent overwrite | TIG-F-06 / M06 |
| SP07 | An object becomes corrupt after a prior integrity observation | Request it or consider using it as retained custody | Invalidate the stale observation, reacquire if permitted, and preserve other good copies | TIG-F-07 / M07 |
| SP08 | Another application consumes disk space after Tig reserves capacity | Attempt the next write | Handle provider full-disk safely; reservations are admission controls, not guarantees about external writers | TIG-F-02 / M02 |
| SP09 | A producer exceeds its output estimate halfway through writing | Request more admitted capacity | Bound or pause the operation; renegotiate explicitly rather than fill the entire cache | TIG-F-02 / M02 |
| SP10 | An unregistered hot object is classified for archive-only retention | Admit an explicit archive claim and reconcile | Verify archive requirements before releasing the hot copy; no last-copy deletion | TIG-F-01 / M01 |
