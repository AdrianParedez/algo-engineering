---
doc_id: AE-08-05
title: Skill Progression Matrix
status: active
last_verified: 2026-05-22
source_scope: capability-maturity-model
depends_on:
  - AE-08-01
see_also:
  - AE-08-03
  - AE-91-02
  - AE-92-02
---

# Skill Progression Matrix

## Scope
Capability matrix from competent practitioner to elite algorithm engineer, organized by reasoning, implementation, measurement, and systems scope.

## Decision Matrix
| Level | Reasoning | Implementation | Measurement | Systems scope |
| --- | --- | --- | --- | --- |
| L1 | Reads asymptotic results | Implements known algorithms | Runs simple timers | Single-process only |
| L2 | Chooses cost models consciously | Tunes layout and basic parallelism | Uses benchmark harnesses correctly | Understands cache and allocator impact |
| L3 | Designs experiments and interprets counters | Specializes kernels and data paths | Can explain bottlenecks mechanistically | Works across NUMA, I/O, or clusters |
| L4 | Bridges theory and production evidence | Builds reusable performance infrastructure | Creates trustworthy organizational benchmarks | Shapes architecture and research direction |

## Use
- Use the matrix to choose next tasks: weak measurement skill means backlog should emphasize harnesses and profiler literacy before advanced lock-free or distributed work.
- Progression is nonlinear; strong coding ability without measurement discipline does not qualify as advanced algorithm engineering. [SRC-001] [SRC-024]

## Related Docs
- [AE-08-03: Implementation Backlog](03-implementation-backlog.md)
- [AE-91-02: Benchmark Template](../_templates/02-benchmark-template.md)
- [AE-92-02: Expansion Modules](../_tracking/02-expansion-modules.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-024](https://nnethercote.github.io/perf-book/profiling.html) - The Rust Performance Book: Profiling. Rust performance guide. 2026-05-08 verified.
