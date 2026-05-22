---
doc_id: AE-02-02
title: Tree Structures
status: active
last_verified: 2026-05-22
source_scope: hierarchical-indices-and-search-structures
depends_on:
  - AE-01-04
see_also:
  - AE-02-05
  - AE-05-01
  - AE-04-01
---

# Tree Structures

## Scope
Practical comparison of balanced trees, B-trees, radix trees, Fenwick/segment trees, and heaps under cache, concurrency, and update-frequency constraints.

## Decision Matrix
| Family | Best use | Engineering pressure point |
| --- | --- | --- |
| B-tree / B+ tree | Storage and cache-friendly indexing | Node sizing and split policy. |
| Radix / ART | Prefix-heavy or string keys | Key normalization and sparse branching. |
| Binary balanced tree | General ordered maps | Pointer chasing and allocator overhead. |
| Fenwick / segment tree | Range aggregates | Memory footprint and update/query balance. |
| Heap | Priority scheduling and selection | Contention and locality under concurrency. |

## Theory
- Trees trade flat locality for asymptotically structured updates, ordering, or hierarchical aggregation.
- High fan-out structures reduce height and I/O depth, which is why B-tree variants dominate storage-aware indexing. [SRC-006]

## Production Reality
- Node size is an engineering parameter, not a theorem artifact; cache line size, page size, and branch predictor behavior shape the sweet spot.
- Pointer-rich balanced trees are often beaten by flatter alternatives once memory bandwidth or allocator overhead becomes dominant. [SRC-039]

## Optimization Patterns
- Choose fan-out to match the access layer you are protecting: cache-resident trees favor one regime, page-resident trees another.
- Consider radix or ART-style layouts when comparison cost or key-length variance dominates traversal cost.

## Failure Modes
- Concurrent heaps and trees frequently serialize on a small part of the structure; contention, not asymptotic complexity, becomes decisive.
- Order-maintaining trees can be a poor fit for scan-heavy analytics that would rather sort once and traverse linearly.

## Related Docs
- [AE-02-05: Cache-Efficient Layouts](05-cache-efficient-layouts.md)
- [AE-05-01: Database Algorithms](../05-systems/01-database-algorithms.md)
- [AE-04-01: Sorting and Searching](../04-algorithms/01-sorting-and-searching.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-006](https://doi.org/10.1145/48529.48535) - The Input/Output Complexity of Sorting and Related Problems. Communications of the ACM article. 1988-09-01.
- [SRC-039](https://github.com/jemalloc/jemalloc/wiki/Background) - jemalloc Background. GitHub wiki. 2023-06-01 verified.
