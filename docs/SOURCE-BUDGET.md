# Source and runtime data budgets

Current ordinary source/documentation ceiling: **5 MiB** (5,242,880 bytes),
with **128 KiB** per file, enforced by tools/validate.ps1. Earlier 1, 2, 3, 3.25, 3.5 and 4 MiB ceilings were historical checkpoints. This covers implemented Rust,
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

2026-11-08 controlled increase (input355aad7): the 4 MiB implementation had
4,165,590 ordinary bytes before mandatory source-admission code and its process/
fault tests. The 5 MiB ceiling admits actual generic Rust implementation/tests;
per-file limits and all runtime/private/binary exclusions remain enforced. This
changes no payload, journal, inventory or capacity limit. DOCS/PROVIDER review is
recorded with the admission slice; generated fixture bytes remain external.
