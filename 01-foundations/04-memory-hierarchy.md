---
doc_id: AE-01-04
title: Memory Hierarchy
status: active
last_verified: 2026-05-22
source_scope: hierarchy-awareness-and-data-movement
depends_on:
  - AE-01-03
see_also:
  - AE-02-05
  - AE-03-03
  - AE-05-05
---

# Memory Hierarchy

## Scope
Practical reference for registers, caches, TLBs, NUMA, storage, and the cost of moving data across hierarchy levels.

## Decision Matrix
| Level | Typical benefit | Typical engineering concern |
| --- | --- | --- |
| Registers / SIMD lanes | Highest arithmetic density | Vector legality and packing. |
| L1/L2/L3 caches | Low-latency reuse | Blocking, misses, and false sharing. |
| TLB / page layer | Address translation locality | Page churn and random walks. |
| NUMA DRAM | Socket-local bandwidth | Placement, remote traffic, and allocator policy. |
| SSD / external storage | Capacity | Batching and sequentiality. |

## Theory
- Data movement is often the dominant cost once working sets exceed the cheapest local level; I/O and cache complexity exist precisely because arithmetic counts stop being predictive. [SRC-006] [SRC-007]
- Hierarchy-aware analysis should track not only reuse distance but also access regularity, write amplification, and page translation effects.

## Production Reality
- Memory-bound workloads often present as low retiring percentages and high memory-bound fractions in top-down analysis. [SRC-027]
- NUMA effects and remote DRAM traffic can erase nominal speedups from threading if data placement is left to chance. [SRC-026] [SRC-038]

## Optimization Patterns
- Change layout before adding instructions: blocking, SoA, compressed sparse layouts, and prefetch-friendly traversals usually beat heroic instruction scheduling.
- Use huge pages, arena allocators, or page-local sharding only when the access pattern and fragmentation story justify the extra operational complexity. [SRC-038] [SRC-039]

## Failure Modes
- Overblocking can increase code complexity and register pressure while providing no benefit for small or irregular working sets.
- Optimizing for cache while ignoring TLB behavior can fail on pointer-heavy structures and random graph traversals.

## Benchmark/Profiling Notes
- Profile with cache-miss, bandwidth, local-vs-remote memory, and page-fault signals before attributing a plateau to “CPU limits.” [SRC-026] [SRC-027]
- Measure both cold-start and steady-state behavior for hierarchies that rely on cache warming or buffer-pool reuse.

## Related Docs
- [AE-02-05: Cache-Efficient Layouts](../02-data-structures/05-cache-efficient-layouts.md)
- [AE-03-03: Memory Optimization](../03-performance-engineering/03-memory-optimization.md)
- [AE-05-05: Low-Latency Systems](../05-systems/05-low-latency-systems.md)

## Sources used
- [SRC-006](https://doi.org/10.1145/48529.48535) - The Input/Output Complexity of Sorting and Related Problems. Communications of the ACM article. 1988-09-01.
- [SRC-007](https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf) - Cache-Oblivious Algorithms. FOCS paper PDF. 1999-05-01.
- [SRC-026](https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/current/memory-access-analysis.html) - Memory Access Analysis for Cache Misses and High Bandwidth Issues. Intel VTune documentation. 2025-07-01 verified.
- [SRC-027](https://www.intel.com/content/www/us/en/docs/vtune-profiler/cookbook/2024-0/top-down-microarchitecture-analysis-method.html) - Top-down Microarchitecture Analysis Method. Intel VTune cookbook. 2026-05-20 verified.
