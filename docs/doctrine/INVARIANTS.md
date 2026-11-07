# Tig invariants

Full-system invariants remain OPEN. The 13 passing pure tests provide only the
limited mechanism evidence recorded in specs/HANDOFF-PROTOTYPE.md.

| ID | Invariant | Status | Notes |
| --- | --- | --- | --- |
| TIG-I-01 | Tig-controlled writes, replacement, transfer, eviction and recovery preserve recoverability of acknowledged managed bytes and all admitted protection/custody obligations until explicitly authorized final retirement | OPEN | T02/T03/T07/T08/T09/T10/T23/T24/CL02/CL03/GW12/CR06/CR07/RW04/RW06/RW10/LF06/SP07/SP10/BN07/BN08/LC02/LC05/LC07; designed only |
| TIG-I-02 | Unknown coverage never proves absent demand | OPEN | T02/T03/T06/T07/T08/T09/T10/T11/T17/T19/T20/T23/CL01/CL02/CL03/CL10/GW08/GW09/GW10/GW12/GW15/CR08/RW04/LF06/LF07/LF08/LF09/SP07/SP10/BN01/BN02/BN04/BN07/BN08/BN10/LC02/LC03/LC05/LC06/LC07/LC08; designed only |
| TIG-I-03 | Reconciliation is idempotent and stale removal cannot override newer protection | OPEN | T06/T11/T12/T13/T19/T20/CL04/CL06/CL10/GW08/GW09/GW10/GW15/CR05/CR08/RW01/RW02/RW07/RW09/LF07/LF08/LF09/BN04/BN10/LC03/LC06/LC08; designed only |
| TIG-I-04 | Compatible bundle claims share one required object copy per assigned place; physical duplicates are accounted separately | OPEN | T01/T15/T18/T22/CL09/GW13/GW14/RW03/SP01/SP02/SP03/SP04/SP08/SP09/BN09/LC09; designed only |
| TIG-I-05 | Lost or uncertain protection records block destructive work until adequate recovery establishes scope | OPEN | T14/CL05/GW03/GW04/GW05/CR01/CR02/CR03/CR04/CR10/SP05/LC01; designed only |
| TIG-I-06 | Partial or unverified publication never satisfies a completed-object or retained-copy obligation | OPEN | T07/T08/T09/T10/T14/T23/CL02/CL03/CL05/CL07/CL08/GW01/GW03/GW04/GW05/GW06/GW07/GW11/GW12/CR01/CR02/CR03/CR04/CR10/LF01/LF02/LF04/LF05/SP05/SP07/BN07/BN08/LC01/LC02/LC04/LC05; designed only |
| TIG-I-07 | A granted valid active-use claim cannot be invalidated by concurrent replica retirement | OPEN | T12/T13/T24/CL04/CL06/CR05/CR06/CR07/RW01/RW02/RW06/RW07/RW09/RW10; designed only |
| TIG-I-08 | Each logical path resolves within one exact snapshot/overlay including explicit removal, and each action binds the intended place identity | OPEN | T04/T05/T16/T21/T25/GW02/CR09/RW05/RW08/LF03/LF10/SP06/BN03/BN05/BN06/LC10; designed only |
| TIG-I-09 | Objects and representations retain explicit exact identity/size mappings through transformation or relocation | OPEN | T04/T05/T18/T22/GW13/CR09/SP02/SP03/SP04/SP06/BN03/BN05/BN09/LC10; designed only |
| TIG-I-10 | Lookup, publication and compaction respect declared resource budgets and expose incomplete work | OPEN | T01/T15/T17/T18/T22/CL01/CL09/GW13/GW14/RW03/SP01/SP02/SP03/SP04/SP08/SP09/BN01/BN02/BN09/LC09; designed only |
| TIG-I-11 | Claim retirement, policy adoption and destructive actions require current admitted owner authority | OPEN | T06/T11/T16/T19/T20/T21/T24/T25/CL07/CL08/CL10/GW01/GW02/GW06/GW07/GW08/GW09/GW10/GW11/GW15/CR06/CR07/CR08/RW05/RW06/RW08/RW10/LF01/LF02/LF03/LF04/LF05/LF07/LF08/LF09/LF10/BN04/BN06/BN10/LC03/LC04/LC06/LC08; designed only |
| TIG-I-12 | Git LFS/client completion semantics and all local-store/worktree materialization costs remain explicit | OPEN | CL07/CL08/GW01/GW06/GW07/GW11/LF01/LF02/LF04/LF05/LC04; designed only |
