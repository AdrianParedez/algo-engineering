---
doc_id: AE-02-05
title: Cache-Efficient Layouts
status: active
last_verified: 2026-05-22
source_scope: layout-transformations-for-locality
depends_on:
  - AE-01-04
see_also:
  - AE-02-01
  - AE-03-02
  - AE-05-01
---

# Cache-Efficient Layouts

## Scope
Patterns for making data placement match traversal order: blocked arrays, cache-oblivious recursion, packed layouts, and metadata separation.

## Decision Matrix
| Layout tactic | Primary win | Tradeoff |
| --- | --- | --- |
| Blocking / tiling | Higher temporal reuse | Tuning sensitivity. |
| Cache-oblivious recursion | Multi-level adaptability | Implementation complexity. |
| Packed / bit-packed layout | Lower bandwidth | Decode cost and complexity. |
| Metadata/data separation | Cleaner hot path | Extra indirection in cold operations. |

## Theory
- Cache-efficient layout is the data-structure face of hierarchy-aware algorithmics: reduce movement by making access footprints smaller and more sequential. [SRC-006] [SRC-007]
- Cache-oblivious designs aim to obtain good behavior across multiple levels without hardcoding cache parameters. [SRC-007] [SRC-008]

## Production Reality
- Layout wins are often more portable than instruction-level tuning because they improve both scalar and vector code paths.
- Metadata separation is increasingly important in allocators and databases where symbol, page, or control data would otherwise pollute hot cache lines. [SRC-034] [SRC-038]

## Optimization Patterns
- Use empirical tuning for block size when the target machine family is narrow; use cache-oblivious recursion when deployment diversity is broad. [SRC-008]
- Pack cold fields and optional attributes away from the hot traversal kernel, even if it complicates construction or debug views.

## When Not To Use
- Do not force cache-oblivious structure into mutation-heavy code if rebuild or rebalance cost dominates end-to-end latency.
- Do not bit-pack aggressively when decode cost destroys vectorization or branch predictability.

## Related Docs
- [AE-02-01: Linear Structures](01-linear-structures.md)
- [AE-03-02: SIMD Vectorization](../03-performance-engineering/02-simd-vectorization.md)
- [AE-05-01: Database Algorithms](../05-systems/01-database-algorithms.md)

## Sources used
- [SRC-006](https://doi.org/10.1145/48529.48535) - The Input/Output Complexity of Sorting and Related Problems. Communications of the ACM article. 1988-09-01.
- [SRC-007](https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf) - Cache-Oblivious Algorithms. FOCS paper PDF. 1999-05-01.
- [SRC-008](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.16) - When Are Cache-Oblivious Algorithms Cache Adaptive? A Case Study of Matrix Multiplication and Sorting. ESA 2022 paper. 2022-09-01.
