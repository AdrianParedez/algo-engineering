---
doc_id: AE-03-01
title: CPU Optimization
status: active
last_verified: 2026-05-22
source_scope: core-execution-and-microarchitecture
depends_on:
  - AE-01-04
see_also:
  - AE-03-02
  - AE-03-06
  - AE-06-01
---

# CPU Optimization

## Scope
Microarchitectural tuning reference for front-end pressure, branch behavior, retiring efficiency, and instruction-level parallelism.

## Decision Matrix
| Bottleneck class | Symptom | First intervention |
| --- | --- | --- |
| Front-end bound | Low decode throughput, I-cache issues | Reduce code footprint and indirect control flow. |
| Bad speculation | Branch miss or misexpect fallout | Simplify branch shape or use data-parallel forms. |
| Core bound | Execution-unit saturation | Unroll, reassociate, or increase ILP. |
| Memory bound | Long-latency stalls | Fix layout and locality before instruction tuning. |

## Theory
- CPU optimization starts from bottleneck taxonomy, not from a bag of tricks. Without knowing whether the core is front-end, execution, or memory limited, tuning is mostly noise. [SRC-027]
- Instruction count alone is insufficient; issue width, dependencies, branch predictability, and cache residency determine realized throughput. [SRC-018]

## Production Reality
- Branchless code helps only when it reduces bad speculation without creating worse dependency chains or excess memory traffic.
- Inlining, outlining cold paths, and shrinking working code size often matter as much as arithmetic instruction selection. [SRC-018] [SRC-024]

## Optimization Patterns
- Use top-down analysis to classify the limit, then tune in bottleneck order: algorithm and layout first, vectorization second, micro-ops last. [SRC-027]
- Prefer PGO or profile-informed branch layout when behavior is stable enough to justify it; static hints without measurement are fragile. [SRC-010]

## Failure Modes
- Manual unrolling can increase pressure on instruction cache and registers enough to lose overall performance.
- Microbenchmarks that fit in L1 may reward transformations that regress real workloads with larger footprints.

## Language Notes
- In C and C++, aliasing promises and object layout directly influence auto-vectorization and scheduling freedom. [SRC-010] [SRC-011]
- Rust annotations such as inline attributes can help, but should always be validated with profiling because code size tradeoffs are real. [SRC-024]

## Related Docs
- [AE-03-02: SIMD Vectorization](02-simd-vectorization.md)
- [AE-03-06: Profiling and Benchmarking](06-profiling-and-benchmarking.md)
- [AE-06-01: Profiler Stack](../06-tooling/01-profiler-stack.md)

## Sources used
- [SRC-017](https://www.intel.com/content/www/us/en/developer/articles/technical/vectorization-llvm-gcc-cpus-gpus.html) - SIMD vectorization in LLVM and GCC for Intel CPUs and GPUs. Intel technical article. 2026-05-22 verified.
- [SRC-018](https://agner.org/optimize/) - Software optimization resources. Agner Fog optimization portal. 2026-05-22 verified.
- [SRC-027](https://www.intel.com/content/www/us/en/docs/vtune-profiler/cookbook/2024-0/top-down-microarchitecture-analysis-method.html) - Top-down Microarchitecture Analysis Method. Intel VTune cookbook. 2026-05-20 verified.
