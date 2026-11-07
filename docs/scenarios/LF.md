# Git and LFS integration

Producer-applied design walkthrough, not an executed test. Each row starts with
its stated condition, applies the next step and checks the required consequence.
All mitigations are planned; references link to the common doctrine and mitigation
register. See [learnings](LEARNINGS.md).

| ID | Starting condition | Next step | Required consequence | Pitfall / mitigation |
| --- | --- | --- | --- | --- |
| LF01 | A clone uses pointer-only checkout and requests one file | Resolve one OID through Tig | Acquire only the explicitly requested object under budget; no branch-wide hydration | TIG-F-08 / M08 |
| LF02 | Git LFS local storage is configured outside the common Git directory | Enroll admitted repository places | Account for the actual configured local-store identity; avoid an invented duplicate default | TIG-F-08 / M08 |
| LF03 | A remote reports authorization failure rather than missing content | Attempt an object acquisition | Expose denied access distinctly; do not rewrite it as absence or try unadmitted sources | TIG-F-09 / M09 |
| LF04 | An upload process exits after a partial destination write | Receive a nominal success indication and inspect transfer completion | Require exact destination-contract verification; partial bytes do not satisfy upload | TIG-F-08 / M08 |
| LF05 | A pointer uses an unsupported extension/transform | Build its object inventory | Block the affected mapping; do not equate transformed bytes with a plain OID | TIG-F-08 / M08 |
| LF06 | A Git LFS prune would touch storage shared with another enrolled worktree | Preview cleanup | Require coordinated all-consumer protection; Tig claims do not make an external pruner safe automatically | TIG-F-01 / M01 |
| LF07 | A change to attributes changes pointer interpretation for many unchanged files | Use an incremental tree cache | Invalidate affected interpretation context and rescan only admitted scope with coverage | TIG-F-12 / M12 |
| LF08 | Git garbage collection expires reflogs that once named a retained release | Reconcile configured historical retention | Preserve admitted release objects independently of reflog lifetime | TIG-F-12 / M12 |
| LF09 | A ref is force-pushed while an LFS upload is still in flight | Complete upload and derive branch placement | Keep verified ingest useful but bind adoption to the actual new ref, not transfer timing | TIG-F-12 / M12 |
| LF10 | A repository supplies a custom transfer executable or credential-bearing URL | Open/enroll the repository | Require trusted local admission; never execute or export it merely because it is committed | TIG-F-09 / M09 |
