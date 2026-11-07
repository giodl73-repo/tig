---
name: tig-research
description: Run a cited research pass for Tig design, standards, benchmarks, or adoption decisions.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# Tig Research

Use this skill before standardizing a new contract, changing a benchmark, or
making a cross-repo adoption decision.

## Workflow

1. State the research question and the decision it informs.
2. Inspect local evidence first: README, specs, tests, fixtures, benchmarks, and
   dependency trackers.
3. Check external sources only when needed for ecosystem or protocol claims.
4. Record findings with IDs, citations, implications, confidence, and non-goals.
5. Split recommendations into adopt now, prototype, and defer/reject.
6. Run or cite validation commands for measurable claims.

## Output

Write a repo-local research note under `docs/research/` or another documented
research location, then update any affected tracker or dependency-system file.
