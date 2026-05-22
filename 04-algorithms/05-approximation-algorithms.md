---
doc_id: AE-04-05
title: Approximation Algorithms
status: active
last_verified: 2026-05-22
source_scope: bounded-suboptimality-for-hard-problems
depends_on:
  - AE-01-05
see_also:
  - AE-04-06
  - AE-07-03
  - AE-08-02
---

# Approximation Algorithms

## Scope
How to use bounded-suboptimal algorithms in practice: where guarantees matter, where heuristics are good enough, and how to report solution quality honestly.

## Decision Matrix
| Approach | Engineering strength | Caveat |
| --- | --- | --- |
| Primal-dual / LP rounding | Strong guarantees with structure | Often heavy implementation and solver coupling. |
| Local search | Simple and often competitive | Hard-to-predict runtime and quality plateaus. |
| PTAS / EPTAS style | Strong asymptotic quality | May be impractical at production scales. |
| Heuristic + certificate | Useful middle ground | Certificate may be weak or expensive. |

## Theory
- Approximation is the disciplined alternative to pretending hard optimization problems are either exactly solved or hopeless.
- A useful guarantee should be interpreted together with instance structure and the runtime/memory budget required to realize it.

## Production Reality
- Bounded suboptimality matters only if the objective reflects business or system cost accurately; otherwise a strong ratio on the wrong objective is empty.
- Heuristics often beat theoretically stronger methods on operational constraints, but should still be benchmarked against exact or bounded references on tractable slices. [SRC-003]

## Optimization Patterns
- Use approximation algorithms as calibrators even when the final system uses heuristics; they tell you what quality you are leaving on the table.
- Record runtime-quality tradeoff curves, not only the best quality found.

## Failure Modes
- Comparing heuristics only against each other creates local optimism and hides how far the solution may be from a useful baseline.
- Approximation schemes with beautiful theory can still be dominated by memory or preprocessing costs in production.

## Related Docs
- [AE-04-06: Randomized Algorithms](06-randomized-algorithms.md)
- [AE-07-03: Open Problems](../07-research/03-open-problems.md)
- [AE-08-02: Project Roadmap](../08-execution/02-project-roadmap.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
