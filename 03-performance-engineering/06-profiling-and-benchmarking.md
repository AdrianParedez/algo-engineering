---
doc_id: AE-03-06
title: Profiling and Benchmarking
status: active
last_verified: 2026-05-22
source_scope: operational-performance-standards
depends_on:
  - AE-00-03
see_also:
  - AE-06-01
  - AE-06-02
  - AE-08-04
---

# Profiling and Benchmarking

## Scope
Operational standard for performance measurement in this workspace. It defines how experiments should be run, interpreted, and rejected when noise or poor design makes conclusions unsafe.

## Decision Matrix
| Layer | Mandatory practice | Reject result when |
| --- | --- | --- |
| Environment | Record CPU model, affinity, frequency policy, allocator, compiler, and input set | Host noise, oversubscription, or missing metadata dominate. |
| Microbenchmark | Warm up, isolate setup, use statistical framework | Measurement overhead is comparable to work and not accounted for. |
| Macrobenchmark | Use representative workload mix and steady-state criterion | Scenario is synthetic without linkage to production behavior. |
| Profiling | Profile before optimizing and after each major change | Optimization target is chosen without bottleneck evidence. |

## Environment Control
- Pin the benchmark to a defined CPU set when stability matters; record whether isolation, governor changes, or NUMA placement were used. [SRC-022]
- Warm caches, JITs, allocators, and background threads until the measured phase reflects steady-state rather than startup artifacts. [SRC-021] [SRC-022]
- On shared hosts or CI, treat results as regression indicators, not absolute truths, unless noise has been characterized explicitly. [SRC-022] [SRC-025]

## Benchmark Design
- Separate setup from measured work; for mutating kernels use batch or custom timing loops that avoid charging setup and teardown to the core operation. [SRC-020]
- Use black_box or equivalent barriers where needed, but verify that the resulting benchmark still resembles real code and does not become a benchmark of the barrier itself. [SRC-019] [SRC-020]
- For throughput benchmarks, report both wall time and processed bytes/items; for latency, report distributional data or confidence bounds, not only means. [SRC-019] [SRC-021]

## Counter and Profiler Policy
- Use sampling profilers and hardware counters to classify the bottleneck before editing code. [SRC-024] [SRC-027]
- Prefer a mixed stack: coarse hotspot profiler, microarchitectural counter view, allocation or heap profiler, and continuous profiling where production drift matters. [SRC-024] [SRC-027]
- If wall-time changes are smaller than noise but counters move consistently, state that explicitly instead of forcing a yes/no speedup verdict. [SRC-021] [SRC-025]

## Statistical Treatment
- Keep raw samples, confidence intervals, and outlier diagnostics. A result that cannot be audited later is not a stable engineering artifact. [SRC-021] [SRC-022]
- Use repetitions or resampling appropriate to the harness; do not hand-average a handful of ad hoc runs and call it evidence. [SRC-019] [SRC-021]

## Benchmarking Pitfalls
- Do not compare binaries built with different profile, LTO, or debug-info regimes unless the difference is the point of the experiment. [SRC-040]
- Do not extrapolate microbenchmark wins to end-to-end systems when cache residency, I/O, or queueing structure changes between the two contexts.
- Do not merge benchmark results across machines unless hardware topology and frequency behavior are close enough that the comparison is still meaningful. [SRC-022]

## Language Notes
- C++: Google Benchmark is the default microbenchmark harness for stable repeated function-level measurements. [SRC-019]
- Rust: Criterion is preferred when statistical analysis and mutation-aware timing-loop control are needed. [SRC-020] [SRC-021]
- Python: pyperf is the baseline harness; when profiling CPython on Linux, integrate perf for native and interpreter-side visibility. [SRC-022] [SRC-023]

## Related Docs
- [AE-06-01: Profiler Stack](../06-tooling/01-profiler-stack.md)
- [AE-06-02: Benchmark Frameworks](../06-tooling/02-benchmark-frameworks.md)
- [AE-08-04: Verification Methodology](../08-execution/04-verification-methodology.md)

## Sources used
- [SRC-019](https://github.com/google/benchmark/blob/main/docs/user_guide.md) - google/benchmark user guide. GitHub documentation. 2026-05-15 verified.
- [SRC-020](https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html) - Criterion.rs Timing Loops. Criterion.rs documentation. 2026-05-08 verified.
- [SRC-021](https://bheisler.github.io/criterion.rs/book/analysis.html) - Criterion.rs Analysis Process. Criterion.rs documentation. 2026-05-08 verified.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
- [SRC-024](https://nnethercote.github.io/perf-book/profiling.html) - The Rust Performance Book: Profiling. Rust performance guide. 2026-05-08 verified.
- [SRC-025](https://nnethercote.github.io/perf-book/benchmarking.html) - The Rust Performance Book: Benchmarking. Rust performance guide. 2026-05-08 verified.
- [SRC-027](https://www.intel.com/content/www/us/en/docs/vtune-profiler/cookbook/2024-0/top-down-microarchitecture-analysis-method.html) - Top-down Microarchitecture Analysis Method. Intel VTune cookbook. 2026-05-20 verified.
