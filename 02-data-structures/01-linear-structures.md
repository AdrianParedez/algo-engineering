---
doc_id: AE-02-01
title: Linear Structures
status: active
last_verified: 2026-05-22
source_scope: layout-and-sequential-containers
depends_on:
  - AE-01-04
see_also:
  - AE-02-05
  - AE-03-03
  - AE-04-01
---

# Linear Structures

## Scope
Engineering guide to arrays, ring buffers, deques, packed vectors, and bitmaps. The focus is on when contiguous layout turns into a force multiplier for locality, vectorization, and predictable iteration.

## Decision Matrix
| Structure | Wins on | Breaks on |
| --- | --- | --- |
| Flat array / vector | Scan, sort, SIMD, binary search | Frequent middle inserts/deletes. |
| Ring buffer | Bounded queues and streaming windows | Unbounded growth and random access hot paths. |
| Deque / segmented buffer | Double-ended mutation | Tight SIMD-friendly sequential kernels. |
| Bitmap / bitset | Dense boolean or set operations | Sparse high-cardinality keys. |

## Theory
- Linear structures minimize pointer chasing and maximize sequentiality, which makes them the default baseline for throughput-oriented work.
- Amortized O(1) append remains useful, but the engineering question is often whether growth, copying, and alignment behavior disturb the hot path.

## Production Reality
- Contiguous layout amplifies hardware prefetchers and SIMD loads, but growth strategies, alignment, and ownership semantics still decide whether the benefit is realized.
- Bit-packing reduces footprint and bandwidth but may increase decode overhead and complicate branchless updates.

## Optimization Patterns
- Prefer structure-of-arrays over array-of-structures when kernels consume only a subset of fields and vector loads can stay aligned.
- Use ring buffers for fixed-latency streaming windows because they preserve bounded allocation behavior and stable addresses. [SRC-001]

## Failure Modes
- Segmented containers can quietly defeat vectorization and prefetching when used inside arithmetic-heavy loops.
- Manual small-buffer optimizations often backfire if they increase branches or copy volume in the steady state.

## Language Notes
- In C and C++, contiguous ownership and allocator choice directly affect relocation cost and alignment control.
- Rust slices and Vec provide strong default linear baselines, but performance still depends on reserve strategy, element size, and copy semantics. [SRC-024]
- Python lists are dynamic arrays with high per-element object overhead; use array, NumPy, or memoryview-style structures for numeric hot paths.

## Related Docs
- [AE-02-05: Cache-Efficient Layouts](05-cache-efficient-layouts.md)
- [AE-03-03: Memory Optimization](../03-performance-engineering/03-memory-optimization.md)
- [AE-04-01: Sorting and Searching](../04-algorithms/01-sorting-and-searching.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-026](https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/current/memory-access-analysis.html) - Memory Access Analysis for Cache Misses and High Bandwidth Issues. Intel VTune documentation. 2025-07-01 verified.
- [SRC-027](https://www.intel.com/content/www/us/en/docs/vtune-profiler/cookbook/2024-0/top-down-microarchitecture-analysis-method.html) - Top-down Microarchitecture Analysis Method. Intel VTune cookbook. 2026-05-20 verified.
