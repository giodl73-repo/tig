# Pulse 01: Foundation and placement model

Goal: Scaffold, 25 designed scenarios, roles, backend decision and structural checks.

Status: complete for design/scaffold only. Local structural checks pass.
No runtime behavior is claimed from foundation.

Validation: pwsh -NoProfile -File tools/validate.ps1; git diff --check.
When implementation exists, add fmt/locked Rust tests/Clippy and exact focused
failure fixtures before closing the pulse. Update docs/vtrace/REVIEW.md with
producer/independent method and exact evidence scope.
