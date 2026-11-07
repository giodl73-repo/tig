# Open risks and disposition

These risks are accepted only as visible design work, not waived runtime safety
conditions. No critical failure is marked structurally prevented by prose.

| Risk | Exposure / doctrine | Accountable owner | Mitigation package / closure |
| --- | --- | --- | --- |
| RISK-001 | Lost authoritative protection or misleading durability acknowledgement; F03/F07 | concurrency-recovery | WP-003/004; fault/replay/restore byte recoverability and true acknowledgement levels |
| RISK-002 | Reader/claim race, stale removal or timeout-as-permission; F01/F04/F05 | concurrency-recovery | WP-004/007/009; effective scoped fencing, clock/renewal proof and no global pause |
| RISK-003 | Same-domain, corrupt/offline or key-inaccessible archive counted as backup; F07/F11 | storage-custody | WP-006/007/013; admitted independent retention and verified recoverability |
| RISK-004 | Duplicate materializations/pack pinning and unexpected peak space; F02/F11 | storage-custody | WP-006/007/008/013/014; bounded physical measurements and full-disk recovery |
| RISK-005 | Git hooks, OID-only transfers or external prune erase lifecycle scope; F08/F12 | git-adoption | WP-005/009; exact source owners, real-client integration and explicit unsupported pruning |
| RISK-006 | Namespace/root-only pointer or representation drift; F06/F10/F11 | identity-namespace | WP-001/002/010; vectors, merge granularity and bounded path/tree interpretation |
| RISK-007 | Backend lacks required removal/durability semantics or operational fit; F09 | provider-privacy | WP-000/003/004/013; capability evaluation, unavailable operations remain blocked |
| RISK-008 | Art extraction changes semantic identity or makes typed layer mandatory; F06/F09 | identity-namespace | WP-010/011; source/lock/license parity and independent low-level caller |
| RISK-009 | Evaluation or old commit bypasses current authority, leaks evidence or claims false delivery; F09/F12 | provider-privacy | WP-012/015; exact subjects, trusted policy/withdrawal and redacted fake-host runs |
| RISK-010 | Review/good documentation implies production safety; all OPEN invariants | git-adoption | Every package; source-bound evidence, stated review method and separate validation/readiness decisions |

## Stage review

Producer-applied six-role lenses: pass_with_risk for risk allocation. RISK-01:
deferred critical work could be read as an accepted live hazard. Resolved by
restricting acceptance to design planning; affected live actions remain unavailable
until their mandatory evidence closes the scoped gap.
