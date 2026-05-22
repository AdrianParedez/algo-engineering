---
doc_id: AE-04-02
title: Graph Algorithms
status: active
last_verified: 2026-05-22
source_scope: graph-computation-and-engineering-tradeoffs
depends_on:
  - AE-02-03
see_also:
  - AE-05-02
  - AE-07-02
  - AE-07-03
---

# Graph Algorithms

## Scope
Engineering reference for traversal, shortest paths, connectivity, routing, and dynamic graph maintenance.

## Decision Matrix
| Problem class | Best engineering question | Common limiter |
| --- | --- | --- |
| Traversal / BFS | How to shape frontier work? | Random access and frontier skew. |
| Shortest paths | Which priority structure and heuristic pay off? | Queue contention and graph locality. |
| Connectivity / union-find | Can the workload be batched? | Synchronization and update ordering. |
| Dynamic graph maintenance | How much recourse is acceptable? | State churn and adversarial updates. |

## Theory
- Graph problems are algorithmically rich and structurally unstable: the right method depends on sparsity, diameter, weights, mutability, and query/update mix.
- Recent work on adaptive-adversary dynamic graph algorithms underscores that update robustness is now part of the frontier, not only static throughput. [SRC-051]

## Production Reality
- The main costs are usually irregular memory access, priority-queue behavior, and partition balance rather than arithmetic complexity.
- Preprocessing-heavy methods can be the right trade when query volume is massive and graph evolution is slow. [SRC-003]

## Optimization Patterns
- Match representation to dominant access pattern: frontier scan, edge relaxation, neighborhood expansion, or incremental update.
- If dynamic updates are rare, rebuild into a traversal-friendly static layout rather than carrying mutation overhead forever.

## Failure Modes
- Benchmarking only synthetic Erdos-Renyi style graphs hides degree skew and locality pathologies common in production graphs.
- A fast queue or faster relax kernel does not fix a partitioning strategy that causes remote memory storms.

## Related Docs
- [AE-05-02: Distributed Algorithms](../05-systems/02-distributed-algorithms.md)
- [AE-07-02: Modern Breakthroughs](../07-research/02-modern-breakthroughs.md)
- [AE-07-03: Open Problems](../07-research/03-open-problems.md)

## Sources used
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
- [SRC-051](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.17) - Simple Dynamic Spanners with Near-Optimal Recourse Against an Adaptive Adversary. ESA 2022 paper. 2022-09-01.
