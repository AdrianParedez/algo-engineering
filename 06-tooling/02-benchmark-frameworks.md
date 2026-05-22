---
doc_id: AE-06-02
title: Benchmark Frameworks
status: active
last_verified: 2026-05-22
source_scope: benchmark-harness-selection
depends_on:
  - AE-03-06
see_also:
  - AE-06-03
  - AE-08-04
  - AE-91-02
---

# Benchmark Frameworks

## Scope
Framework selection guide for C++, Rust, and Python benchmarking, with rules for when to use microbenchmark harnesses, shell-level tools, or custom runners.

## Decision Matrix
| Framework | Use when | Main strength |
| --- | --- | --- |
| Google Benchmark | C++ function-level microbenchmarks | Mature harness and counters integration. |
| Criterion.rs | Rust benchmarks needing statistics and setup isolation | Strong analysis and timing-loop control. |
| pyperf | Python microbenchmarks and environment tuning | Worker isolation and metadata collection. |
| Custom runner | Pipelines, services, or distributed workloads | Full control over workload semantics. |

## Framework Policy
- Use the smallest harness that preserves workload meaning; benchmark frameworks are valuable only if the measured unit is still representative. [SRC-019] [SRC-020] [SRC-022]
- Prefer harnesses with sample persistence, outlier reporting, and environment metadata over ad hoc timers. [SRC-021] [SRC-022]

## Language Defaults
- C++: Google Benchmark for local kernel work, with custom counters for bytes and items processed. [SRC-019]
- Rust: Criterion when mutation-aware setup and statistical reporting matter. [SRC-020] [SRC-021]
- Python: pyperf for anything expected to survive review or regression tracking. [SRC-022]

## Failure Modes
- Framework defaults are not universal truths; min-time, repetitions, and warmup policy should match workload variance.
- A benchmark that cannot express setup/teardown isolation is often a sign you are measuring the wrong level.

## Related Docs
- [AE-06-03: Build Systems](03-build-systems.md)
- [AE-08-04: Verification Methodology](../08-execution/04-verification-methodology.md)
- [AE-91-02: Benchmark Template](../_templates/02-benchmark-template.md)

## Sources used
- [SRC-019](https://github.com/google/benchmark/blob/main/docs/user_guide.md) - google/benchmark user guide. GitHub documentation. 2026-05-15 verified.
- [SRC-020](https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html) - Criterion.rs Timing Loops. Criterion.rs documentation. 2026-05-08 verified.
- [SRC-021](https://bheisler.github.io/criterion.rs/book/analysis.html) - Criterion.rs Analysis Process. Criterion.rs documentation. 2026-05-08 verified.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
