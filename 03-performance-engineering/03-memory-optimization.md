---
doc_id: AE-03-03
title: Memory Optimization
status: active
last_verified: 2026-05-22
source_scope: allocator-locality-and-bandwidth-optimization
depends_on:
  - AE-01-04
see_also:
  - AE-02-05
  - AE-05-05
  - AE-06-04
---

# Memory Optimization

## Scope
Reference for locality improvement, allocation strategy, page policy, and NUMA placement when data movement dominates.

## Decision Matrix
| Problem | Likely signal | First move |
| --- | --- | --- |
| Allocator contention | High lock or CAS pressure around alloc/free | Shard heaps or switch allocator policy. |
| Remote NUMA traffic | Bandwidth limit with socket imbalance | Pin threads and place memory locally. |
| Fragmentation / footprint | RSS growth and cache dilution | Revisit object lifetime and arena policy. |
| Copy amplification | High memcpy or writeback cost | Redesign data flow or ownership boundaries. |

## Theory
- Memory optimization is about reducing bytes moved, reducing costly placement decisions, and increasing useful reuse per byte fetched.
- Allocator policy is algorithmic: it changes placement, synchronization, fragmentation, and tail behavior, not just constant factors. [SRC-038] [SRC-039]

## Production Reality
- General-purpose allocators are optimized for broad workloads; hot services often need arena sharding, object pools, or different purge policies. [SRC-038] [SRC-039]
- NUMA-local placement is a best-effort property that can be undermined by thread migration, cross-socket freeing, or background compaction. [SRC-038]

## Optimization Patterns
- Treat allocation sites as part of the hot path: move object creation out of tight loops, batch reclamation, and stabilize ownership graphs.
- Use first-class heaps or arenas when lifetimes align strongly enough that bulk release beats per-object churn. [SRC-038] [SRC-039]

## Failure Modes
- Custom pools can improve throughput while worsening footprint, observability, or leak behavior if lifecycle boundaries are weak.
- Huge-page or aggressive pinning strategies may help one workload and destabilize mixed-tenant hosts.

## Related Docs
- [AE-02-05: Cache-Efficient Layouts](../02-data-structures/05-cache-efficient-layouts.md)
- [AE-05-05: Low-Latency Systems](../05-systems/05-low-latency-systems.md)
- [AE-06-04: Observability](../06-tooling/04-observability.md)

## Sources used
- [SRC-026](https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/current/memory-access-analysis.html) - Memory Access Analysis for Cache Misses and High Bandwidth Issues. Intel VTune documentation. 2025-07-01 verified.
- [SRC-038](https://github.com/microsoft/mimalloc) - mimalloc README. GitHub repository documentation. 2026-04-29 release.
- [SRC-039](https://github.com/jemalloc/jemalloc/wiki/Background) - jemalloc Background. GitHub wiki. 2023-06-01 verified.
