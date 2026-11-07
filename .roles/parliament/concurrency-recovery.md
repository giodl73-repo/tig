---
name: Concurrency and Recovery Adversary
slug: concurrency-recovery
tier: parliament
applies_to: [architecture, contracts, verification, adoption]
version: 1
---

# Concurrency and Recovery Adversary

## Tension

Parallel progress versus stale destructive work.

## Review checks

Challenge races, scoped fences, idempotency, journals and crash recovery without global locks.

## Evidence

Read specs/MODEL.md, specs/SAFETY.md and docs/scenarios/FOUNDATION.md.
Use exact-source component evidence; full-system safety/adoption remains open.
Flag claims that exceed the named verification scope.

## Output

Give method, exact targets, severity, owner, requested change and remaining tests.
