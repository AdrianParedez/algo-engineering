---
doc_id: AE-07-02
title: Modern Breakthroughs
status: active
last_verified: 2026-05-22
source_scope: recent-research-curation
depends_on:
  - AE-07-01
see_also:
  - AE-07-04
  - AE-05-01
  - AE-03-05
---

# Modern Breakthroughs

## Scope
Recent papers and directions that materially shift what “modern” algorithm engineering means after 2022.

## Decision Matrix
| Breakthrough | Theme | Why it matters |
| --- | --- | --- |
| Methodology of Algorithm Engineering (2025) | Meta-method | Integrates validity concerns across algorithm research. |
| Cache adaptivity case study (2022) | Memory models | Tests worst-case theory against practical fluctuations. |
| Adaptive-adversary spanners (2022) | Dynamic graphs | Raises the robustness bar for dynamic maintenance. |
| Funnelselect line (2023-2024) | Cache-oblivious selection | Shows continuing vitality of memory-aware theory. |
| SmartPQ (2024) | NUMA-aware concurrency | Targets topology-sensitive concurrent priority queues. |

## Themes
- The frontier is becoming more validity-aware: papers increasingly test whether asymptotic gaps survive contact with realistic memory or concurrency behavior. [SRC-001] [SRC-008]
- Dynamic and adversarial settings are getting more practical, especially in graph maintenance and concurrent structures. [SRC-051] [SRC-054]

## Implications
- Expect more research that blends formal guarantees with experimental methodology rather than treating them as separate publication cultures.
- NUMA, disaggregated memory, and topology-aware data structures are moving from niche systems concerns into core algorithm-engineering territory. [SRC-054]

## Related Docs
- [AE-07-04: Frontier Directions](04-frontier-directions.md)
- [AE-05-01: Database Algorithms](../05-systems/01-database-algorithms.md)
- [AE-03-05: Lock-Free Systems](../03-performance-engineering/05-lock-free-systems.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-008](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.16) - When Are Cache-Oblivious Algorithms Cache Adaptive? A Case Study of Matrix Multiplication and Sorting. ESA 2022 paper. 2022-09-01.
- [SRC-051](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.17) - Simple Dynamic Spanners with Near-Optimal Recourse Against an Adaptive Adversary. ESA 2022 paper. 2022-09-01.
- [SRC-052](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2023.25) - Funnelselect: Cache-Oblivious Multiple Selection. ESA 2023 paper. 2023-09-01.
- [SRC-053](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SWAT.2024.17) - Deterministic Cache-Oblivious Funnelselect. SWAT 2024 paper. 2024-07-01.
- [SRC-054](https://arxiv.org/abs/2406.06900) - SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. arXiv paper. 2024-06-10.
