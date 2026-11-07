---
name: tig-pulse
description: Execute the next Tig wave pulse with docs, implementation, validation, and commit-ready updates.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# Tig Pulse

Use this skill for Tig development pulses.

## Workflow

1. Read `AGENTS.md`.
2. Read `CONTEXT.md` when present.
3. Read `context/waves/PHASES.md`.
4. Read the active wave `WAVE.md`.
5. Read the target pulse under `pulses/`.
6. Implement the smallest complete slice.
7. Keep shared kernels free of consumer-specific behavior.
8. Update docs and wave/pulse status.
9. Run validation commands and `git diff --check`.
