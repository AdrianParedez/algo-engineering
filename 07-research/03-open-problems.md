---
doc_id: AE-07-03
title: Open Problems
status: active
last_verified: 2026-05-22
source_scope: research-gap-mapping
depends_on:
  - AE-07-02
see_also:
  - AE-07-04
  - AE-92-01
  - AE-08-03
---

# Open Problems

## Scope
High-value unresolved questions that sit between algorithm theory, systems performance, and measurement methodology.

## Decision Matrix
| Area | Open question | Why it is hard |
| --- | --- | --- |
| Hierarchy-aware algorithms | When do practical memory profiles invalidate worst-case separations? | Real systems fluctuate and share resources. |
| Concurrent structures | How to combine low contention, NUMA locality, and predictable tails? | Coherence and progress guarantees conflict. |
| Dynamic graphs | How much robustness can be retained with low recourse? | Adaptive adversaries change the game. |
| Methodology | How should external validity be reported for performance claims? | Hardware and workload heterogeneity are large. |

## Research Gaps
- There is still no universally satisfying bridge from elegant cache or concurrency models to the messiness of shared, noisy production systems. [SRC-001] [SRC-008]
- Robust dynamic maintenance under realistic adversaries remains expensive and domain-sensitive. [SRC-051]
- NUMA-aware nonblocking structures are promising but still under-specified in terms of tail predictability and operational complexity. [SRC-054]

## Related Docs
- [AE-07-04: Frontier Directions](04-frontier-directions.md)
- [AE-92-01: Open Questions](../_tracking/01-open-questions.md)
- [AE-08-03: Implementation Backlog](../08-execution/03-implementation-backlog.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-008](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.16) - When Are Cache-Oblivious Algorithms Cache Adaptive? A Case Study of Matrix Multiplication and Sorting. ESA 2022 paper. 2022-09-01.
- [SRC-051](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.17) - Simple Dynamic Spanners with Near-Optimal Recourse Against an Adaptive Adversary. ESA 2022 paper. 2022-09-01.
- [SRC-054](https://arxiv.org/abs/2406.06900) - SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. arXiv paper. 2024-06-10.
