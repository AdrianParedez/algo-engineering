---
doc_id: AE-92-01
title: Open Questions
status: active
last_verified: 2026-05-22
source_scope: verified-gaps-and-hypotheses
depends_on:
  - AE-07-03
see_also:
  - AE-07-04
  - AE-08-03
---

# Open Questions

## Scope
Mutable list of unresolved issues, explicitly labeled as verified gap or synthesis hypothesis.

## Decision Matrix
| Label | Meaning |
| --- | --- |
| Verified gap | Source-backed missing answer or unstable evidence. |
| Synthesis hypothesis | Plausible direction that needs measurement or better sourcing. |

## Usage
- Use this file to prevent premature certainty from leaking into stable documents.
- Promote items out of tracking only after a source-backed update or a reproducible experiment closes the gap.

## Seed Questions

| Label | Question | Reason |
| --- | --- | --- |
| Verified gap | What is the most robust public methodology for comparing continuous-profiling overhead across stacks? | The signal is evolving quickly. |
| Verified gap | What benchmark corpus best captures NUMA-sensitive lock-free queue behavior across architectures? | Current public examples are narrow. |
| Synthesis hypothesis | Profile-guided data-layout adaptation will become a standard optimization loop for long-lived services. | Tooling and observability prerequisites are converging. |

## Related Docs
- [AE-07-04: Frontier Directions](../07-research/04-frontier-directions.md)
- [AE-08-03: Implementation Backlog](../08-execution/03-implementation-backlog.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-054](https://arxiv.org/abs/2406.06900) - SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. arXiv paper. 2024-06-10.
