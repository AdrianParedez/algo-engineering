---
doc_id: AE-01-03
title: Computational Models
status: active
last_verified: 2026-05-22
source_scope: abstract-and-practical-machine-models
depends_on:
  - AE-01-02
see_also:
  - AE-01-04
  - AE-03-04
  - AE-05-02
---

# Computational Models

## Scope
Catalog of machine models that matter in algorithm engineering, from RAM through external-memory, SIMD, and distributed communication models.

## Decision Matrix
| Model | Best for | Blind spot |
| --- | --- | --- |
| RAM | Basic algorithm design and proofs | Hierarchy and communication. |
| External-memory / I-O | Large-data movement | In-core microarchitecture. |
| Cache-oblivious | Portable hierarchy-aware reasoning | Exact cache sizes and replacement quirks. |
| Work-span / fork-join | Shared-memory parallelism | NUMA and scheduler policy. |
| Bulk-synchronous / message-passing | Cluster-scale coordination | Fine-grained local stalls. |
| SIMD lane model | Data-parallel kernels | Irregular control flow and scattered access. |

## Theory
- A computational model is a compression of reality: it isolates the costs you intend to reason about and deliberately hides the rest. [SRC-001]
- Modern algorithm engineering often composes models instead of choosing one forever; for example, work-span plus NUMA plus cache reasoning may all be necessary for one pipeline. [SRC-003] [SRC-006] [SRC-007]

## Production Reality
- The implementation boundary often shifts the relevant model: a database executor cares about vector batches and cache lines, while a collective runtime cares about topology and synchronization domains. [SRC-034] [SRC-033]
- SIMD and accelerator execution models reward regularity, alignment, and predictable access more than they reward theoretical elegance alone. [SRC-009] [SRC-016] [SRC-017]

## Optimization Patterns
- State explicitly which model a claim is about; “optimal” without a model is nearly content-free.
- Use model transitions intentionally: prototype in RAM, test movement costs under cache or I/O models, then validate against actual counters and traces. [SRC-001] [SRC-027]

## Failure Modes
- Assuming the same implementation is simultaneously optimal in RAM, cache, NUMA, and network models is usually wrong unless the workload is exceptionally regular. [SRC-006] [SRC-007]
- Model mismatch is a common cause of “unexpected” regressions after parallelization or vectorization. [SRC-009] [SRC-031]

## Related Docs
- [AE-01-04: Memory Hierarchy](04-memory-hierarchy.md)
- [AE-03-04: Concurrency](../03-performance-engineering/04-concurrency.md)
- [AE-05-02: Distributed Algorithms](../05-systems/02-distributed-algorithms.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-006](https://doi.org/10.1145/48529.48535) - The Input/Output Complexity of Sorting and Related Problems. Communications of the ACM article. 1988-09-01.
- [SRC-007](https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf) - Cache-Oblivious Algorithms. FOCS paper PDF. 1999-05-01.
- [SRC-015](https://www.openmp.org/specifications/) - OpenMP Specifications. OpenMP specifications index. 2026-05-16 verified.
