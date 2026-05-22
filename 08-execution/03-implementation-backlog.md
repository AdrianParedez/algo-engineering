---
doc_id: AE-08-03
title: Implementation Backlog
status: active
last_verified: 2026-05-22
source_scope: prioritized-project-backlog
depends_on:
  - AE-08-02
see_also:
  - AE-08-04
  - AE-07-03
  - AE-92-01
---

# Implementation Backlog

## Scope
Prioritized backlog for turning the corpus into code, experiments, and reusable engineering assets. Priority rubric: impact x measurability x implementation cost x portability x operational risk.

## Decision Matrix
| Backlog class | Example item | Priority bias |
| --- | --- | --- |
| Measurement infrastructure | Benchmark harnesses, result diffing, metadata capture | Highest |
| Reference kernels | Sorting, graph, sketch, and allocator-sensitive kernels | High |
| Systems replications | Vectorized executor, NUMA queue, collective tuning | High |
| Frontier probes | Adaptive layouts, profile-guided tuning loops | Medium until instrumentation is mature |

## Prioritization Rules
- Favor work that improves the quality of all future evidence: benchmark harnesses, metadata capture, and profile-diff tooling rank above isolated hero optimizations. [SRC-022] [SRC-024]
- Prefer tasks that illuminate multiple documents at once, such as a cache-aware graph kernel or a shared profiling recipe. [SRC-001]

## Related Docs
- [AE-08-04: Verification Methodology](04-verification-methodology.md)
- [AE-07-03: Open Problems](../07-research/03-open-problems.md)
- [AE-92-01: Open Questions](../_tracking/01-open-questions.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
- [SRC-024](https://nnethercote.github.io/perf-book/profiling.html) - The Rust Performance Book: Profiling. Rust performance guide. 2026-05-08 verified.
