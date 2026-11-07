# Producer review: journal role placement

Input/controls: [JOURNAL-PLACEMENT](../../specs/JOURNAL-PLACEMENT.md).
Apply the six [role checklists](../../.roles/ROLE.md) as producer review, not
independent validation, consumer acceptance or storage-operation authority.

| Role | Finding and disposition |
| --- | --- |
| Identity | High: archived claim IDs may collide across owners or journals. Derive projection IDs from stable journal genesis/source/owner. Keep the exact observation checkpoint; do not derive IDs from its changing sequence. New checkpoint and View accessors are read-only; no wire bytes change. |
| Placement | High: predecessor roots retaining their old working assignment leak hot demand. Map predecessor to retained places only; current/pending use full assignments. Coalesce per source while preserving peer owners. |
| Custody | Critical: dropping working demand is not removal permission. Projection always denies complete claim coverage; independent custody and provider successor protection/release remain separate unproved boundaries. |
| Recovery | High: interrupted prepare/adopt must keep both working revisions. Actual temporary journal reopen preserves the pending/current projection. Adoption leaves old retention; peer withdrawal leaves its predecessor retained. No journal mutation occurs inside projection. |
| Provider/privacy | High: resetting budgets per root evades limits. A read-only NodeStore wrapper charges lookup attempts and returned bytes across repeated roots. Missing/foreign roots refuse the whole projection. Underlying acquisition remains provider-bounded. No IDs/paths/errors are printed. |
| Git/adoption | High: missing roots could imply absent claims or currentness. Only supplied positive obligations are projected; even empty journals have incomplete global coverage. Source adoption, other journals and unregistered drafts remain external obligations. |

Four actual journal/archive integration tests pass: prepare/adopt/reopen/peer
withdrawal, stable/different journal/source identities and combined claims,
operation-wide repeated-root IO budgets, and missing-root/empty-journal refusal.
No payload moves or physical reclaim occur in these fixtures. Full workspace,
standalone, exact-head hosted qualification and consumer integration remain open.
Disposition pass_with_risk for read-only desired placement; all later T3 gates
remain required.
