---
doc_id: AE-02-03
title: Graph Structures
status: active
last_verified: 2026-05-22
source_scope: graph-representation-engineering
depends_on:
  - AE-01-04
see_also:
  - AE-04-02
  - AE-03-03
  - AE-05-02
---

# Graph Structures

## Scope
Representation choices for sparse, dense, weighted, dynamic, and partitioned graphs. The central question is how layout affects traversal, update cost, and parallel work distribution.

## Decision Matrix
| Representation | Good for | Weakness |
| --- | --- | --- |
| CSR / CSC | Static sparse traversals | Expensive structural mutation. |
| Adjacency vectors | Moderate updates with local scans | Fragmentation under churn. |
| Edge list | Streaming ingest and transforms | Poor repeated neighborhood queries. |
| Blocked / partitioned graph | NUMA or distributed execution | Higher preprocessing cost. |

## Theory
- Graph algorithms are unusually representation-sensitive because traversal order, frontier shape, and locality vary drastically across workloads.
- Static sparse graphs reward compressed contiguous encodings; dynamic graphs shift the balance toward mutation-friendly layouts. [SRC-051]

## Production Reality
- Real graph performance is dominated by irregular memory access, frontier skew, and partition imbalance more often than by arithmetic cost.
- Direction-optimized traversals, edge blocking, and relabeling can change performance classes without changing asymptotic complexity. [SRC-003]

## Optimization Patterns
- Separate structure from payload so traversal kernels do not drag unneeded edge or vertex attributes through caches.
- For shared-memory graph processing, shard by locality domain first and work balance second if remote memory traffic is the main limiter. [SRC-026]

## Failure Modes
- Choosing a mutation-friendly structure for a read-mostly graph often leaves large traversal performance on the table.
- Using a single representation for ingest, storage, analytics, and serving is rarely optimal; conversion stages are often worth the cost.

## Related Docs
- [AE-04-02: Graph Algorithms](../04-algorithms/02-graph-algorithms.md)
- [AE-03-03: Memory Optimization](../03-performance-engineering/03-memory-optimization.md)
- [AE-05-02: Distributed Algorithms](../05-systems/02-distributed-algorithms.md)

## Sources used
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
- [SRC-026](https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/current/memory-access-analysis.html) - Memory Access Analysis for Cache Misses and High Bandwidth Issues. Intel VTune documentation. 2025-07-01 verified.
- [SRC-051](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.17) - Simple Dynamic Spanners with Near-Optimal Recourse Against an Adaptive Adversary. ESA 2022 paper. 2022-09-01.
