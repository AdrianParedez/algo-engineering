---
doc_id: AE-05-01
title: Database Algorithms
status: active
last_verified: 2026-05-22
source_scope: query-processing-and-storage-engineering
depends_on:
  - AE-02-05
see_also:
  - AE-06-04
  - AE-05-04
  - AE-07-02
---

# Database Algorithms

## Scope
Applied algorithm-engineering view of indexing, scans, joins, aggregation, and vectorized execution in modern analytical and mixed workloads.

## Decision Matrix
| Subsystem | Design pressure | Typical win |
| --- | --- | --- |
| Executor | Interpretation overhead vs flexibility | Vectorized or compiled batches. |
| Join engine | Selectivity and memory locality | Hash vs sort-merge specialization. |
| Aggregation | Skew and cardinality | Adaptive hash/table layout and spill policy. |
| Storage layout | Update vs scan tradeoff | Columnar compression and blocked reads. |

## Theory
- Database kernels are algorithm engineering in concentrated form: data movement, predicate shape, and batch structure determine performance.
- Vectorized execution improves CPU efficiency by amortizing interpretation overhead across batches while preserving manageable memory footprint. [SRC-034] [SRC-035]

## Production Reality
- Join and aggregation behavior is workload-sensitive: skew, null density, batch compaction, and spill thresholds often dominate operator choice.
- Execution engines must balance locality with flexibility; the best design is often a staged one rather than a single universal operator path.

## Optimization Patterns
- Separate logical and physical operator concerns so data layout, batch size, and vector width can be tuned without rewriting query semantics.
- Use algorithm switching based on cardinality and selectivity estimates, but instrument misestimation because wrong operator choice is expensive.

## Failure Modes
- Small fragmented batches can erase the benefit of vectorization; compaction policy becomes a first-order concern.
- Ignoring memory-object visibility in profiling makes it hard to distinguish hash-table design issues from allocator or page-layout issues.

## Related Docs
- [AE-06-04: Observability](../06-tooling/04-observability.md)
- [AE-05-04: AI and ML Infrastructure](04-ai-and-ml-infrastructure.md)
- [AE-07-02: Modern Breakthroughs](../07-research/02-modern-breakthroughs.md)

## Sources used
- [SRC-034](https://duckdb.org/docs/current/internals/vector.html) - Execution Format. DuckDB internals documentation. 2026-05-15 verified.
- [SRC-035](https://clickhouse.com/resources/engineering/vectorised-query-execution) - What is vectorised query execution?. ClickHouse engineering article. 2026-05-15 verified.
