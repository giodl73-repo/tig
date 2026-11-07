# Source and runtime data budgets

Current ordinary source/documentation ceiling: **3.5 MiB** (3,670,016 bytes),
with **128 KiB** per file, enforced by tools/validate.ps1. Earlier 1, 2, 3 and
3.25 MiB ceilings were historical checkpoints. This covers implemented Rust,
locked dependencies, synthetic fixtures and docs; it does not allocate cache
space or allow runtime evidence in Git.

Media, builds, databases, runtime logs, inventories, observations and growing
receipts belong outside ordinary Git. Tests generate synthetic temporary bytes.
Machine profiles, credentials and project acceptance also remain external.
Keep meaningful generic regression fixtures.

Budget changes require a concrete reason and recorded review. Do not weaken
checks, disguise generated data as source, or accumulate operational journals
in docs. Replace private notes with public summaries only after verified private
preservation.
