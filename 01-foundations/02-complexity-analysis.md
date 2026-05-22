---
doc_id: AE-01-02
title: Complexity Analysis
status: active
last_verified: 2026-05-22
source_scope: cost-models-and-measurement
depends_on:
  - AE-01-01
see_also:
  - AE-01-03
  - AE-03-06
  - AE-08-04
---

# Complexity Analysis

## Scope
Connects asymptotic complexity to practical cost accounting. The goal is not to replace Big-O, but to choose the smallest cost model that still predicts the observed bottleneck.

## Decision Matrix
| Metric | Captures well | Hides |
| --- | --- | --- |
| Big-O work | Growth under abstract operation counts | Hierarchy, skew, constants, and contention. |
| Span / critical path | Parallel lower bound and scalability ceiling | Scheduler overhead and locality loss. |
| I/O or cache complexity | Movement cost between levels | Branch, ALU, and synchronization costs. |
| Wall time | User-visible runtime | Mechanism unless decomposed with counters. |

## Theory
- Work, span, I/O complexity, and cache complexity answer different questions; the right analysis depends on whether the dominant cost is arithmetic, serialization, data movement, or coordination. [SRC-006] [SRC-007]
- Amortized, smoothed, and parameterized analyses are often more predictive for engineered systems than worst-case bounds alone. [SRC-001]

## Production Reality
- An O(n log n) algorithm can beat O(n) alternatives when the latter scatter memory, force branches, or trigger synchronization hot spots.
- The crossover point between algorithms is empirical and can shift with compiler, SIMD width, allocator, cache size, or input entropy. [SRC-004] [SRC-021]

## Optimization Patterns
- Fit measurements across increasing input sizes and inspect residuals; if the curve is wrong, the cost model or workload model is incomplete. [SRC-021]
- Track operation counts together with bytes moved, cache misses, branch misses, and synchronization counts when explaining a speedup. [SRC-006] [SRC-007]

## Failure Modes
- Using only wall-clock time on one machine encourages accidental overfitting to transient placement, thermal, or scheduler effects. [SRC-022] [SRC-025]
- Normalizing everything to a single “nanoseconds per op” metric hides throughput/latency tradeoffs and cache-sensitive regime changes.

## Benchmark/Profiling Notes
- For microbenchmarks, include confidence intervals and noise diagnostics; for macrobenchmarks, report workload composition and steady-state criteria. [SRC-021] [SRC-022]
- Counter-based metrics often have lower variance than wall time and can reveal whether a change improved work, locality, or just placement luck. [SRC-025] [SRC-027]

## Language Notes
- C and C++ expose aliasing and vectorization assumptions directly through layout and compiler flags. [SRC-010] [SRC-011]
- Rust profile settings such as opt-level, codegen-units, and LTO materially change the effective cost model seen by the machine. [SRC-040]
- Python complexity reasoning must distinguish interpreter overhead from algorithmic work and from native-library kernels. [SRC-022] [SRC-023]

## Related Docs
- [AE-01-03: Computational Models](03-computational-models.md)
- [AE-03-06: Profiling and Benchmarking](../03-performance-engineering/06-profiling-and-benchmarking.md)
- [AE-08-04: Verification Methodology](../08-execution/04-verification-methodology.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-006](https://doi.org/10.1145/48529.48535) - The Input/Output Complexity of Sorting and Related Problems. Communications of the ACM article. 1988-09-01.
- [SRC-007](https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf) - Cache-Oblivious Algorithms. FOCS paper PDF. 1999-05-01.
- [SRC-021](https://bheisler.github.io/criterion.rs/book/analysis.html) - Criterion.rs Analysis Process. Criterion.rs documentation. 2026-05-08 verified.
