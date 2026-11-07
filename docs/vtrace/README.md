# Tig engineering baseline

Scope: full local specification/work-package design, producer-reviewed with risk.
This is an engineering control set, not certification or proof of production
storage safety. Current component counts and exact-source qualification are
recorded in EVIDENCE.md; current-head cross-platform CI remains required. These include
local sampling/publication/restart fixtures. All 110 workflow scenarios remain
designed and full requirements/invariants remain OPEN.

| Question | Controlled record |
| --- | --- |
| Why build Tig and for whom? | [Mission](MISSION.md), [CONOPS](CONOPS.md) |
| What must it do and what exists today? | [Requirements](REQUIREMENTS.md), [current/target specs](SPECIFICATION_BASELINE.md) |
| Who owns each component and contract? | [Architecture](ARCHITECTURE.md), [packages](PACKAGE_BOUNDARIES.md), [interfaces](INTERFACES.md), [design](DESIGN.md) |
| How will it be built and checked? | [Code rigor](CODE_RIGOR.md), [profiles](LANGUAGE_PROFILES.md), [implementation plan](IMPLEMENTATION_PLAN.md), [17 work packages](WORK_PACKAGES.md) |
| How do seams, changes and risks stay controlled? | [Integration](INTEGRATION_PLAN.md), [change control](CHANGE_CONTROL.md), [risks](RISKS.md) |
| What is proven and what is accepted? | [Evidence](EVIDENCE.md), [verification](VERIFICATION.md), [validation](VALIDATION.md), [trace](TRACE.md) |
| How was this reviewed? | [Gate](REVIEW.md), [checklist](REVIEW_CHECKLISTS.md), [stage execution](STAGE_EXECUTION.md), [process](PROCESS.md) |
| How do users learn the product honestly? | [Communications](COMMUNICATIONS_STRATEGY.md), root README and ASCII guides |

Next proof obligations include full source/job coverage and guarded adoption, attribute-aware
extraction, metadata compaction/native scale and typed asset/build semantics. Current
component scopes and entry criteria remain mandatory.
Publication/fencing/local executor packages must prove their right side before
any live cache-removal capability is enabled. Optional typed assets/UI are not
prerequisites for standalone Git/LFS byte management.

Run the portable repository checks with tools/validate.ps1. The optional native
canonical VTRACE structural assessment and source pin are recorded in PROCESS.md.
The explicit-profile experimental tig-lfs is component-tested; no daemon, automatic
configuration or hidden approval system is added here.
