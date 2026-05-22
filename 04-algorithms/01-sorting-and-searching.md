---
doc_id: AE-04-01
title: Sorting and Searching
status: active
last_verified: 2026-05-22
source_scope: core-ordering-and-lookup-algorithms
depends_on:
  - AE-01-05
see_also:
  - AE-02-01
  - AE-03-02
  - AE-07-01
---

# Sorting and Searching

## Scope
Implementation-oriented comparison of comparison sorting, radix sorting, selection, binary search variants, and string-aware lookup strategies.

## Decision Matrix
| Family | Primary win | Primary limitation |
| --- | --- | --- |
| Comparison sort | Robust general-purpose ordering | Comparator cost and branch pressure. |
| Radix / counting family | High throughput on fixed-width keys | Key normalization and extra passes. |
| Selection / partitioning | Top-k and pivot-driven workflows | Unstable access patterns under skew. |
| Binary / galloping search | Cheap ordered lookup | Update cost in mutable datasets. |

## Theory
- Sorting and searching are a canonical example of why asymptotics alone are insufficient: comparator cost, memory locality, and key distribution frequently dominate choice.
- Cache-oblivious and memory-adaptive results matter because real ordering pipelines often exceed a single cache or memory level. [SRC-007] [SRC-008]

## Production Reality
- For small keys and simple comparators, branch and data movement dominate; for strings and complex objects, key extraction and indirection often dominate instead.
- Hybrid algorithms win in practice because no single strategy owns all data sizes, entropy levels, or cache regimes. [SRC-003]

## Optimization Patterns
- Normalize keys and separate payloads from sort keys when movement is expensive.
- Use radix-style methods for fixed-width integer-like keys when the extra passes still cost less than comparator and branch overhead.

## Failure Modes
- Comparator-heavy generic sort can hide massive object movement or cache churn behind a seemingly simple API.
- Top-k pipelines often sort too much; selection plus partial ordering is usually the correct baseline.

## Related Docs
- [AE-02-01: Linear Structures](../02-data-structures/01-linear-structures.md)
- [AE-03-02: SIMD Vectorization](../03-performance-engineering/02-simd-vectorization.md)
- [AE-07-01: Landmark Papers](../07-research/01-landmark-papers.md)

## Sources used
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
- [SRC-007](https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf) - Cache-Oblivious Algorithms. FOCS paper PDF. 1999-05-01.
- [SRC-008](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.16) - When Are Cache-Oblivious Algorithms Cache Adaptive? A Case Study of Matrix Multiplication and Sorting. ESA 2022 paper. 2022-09-01.
