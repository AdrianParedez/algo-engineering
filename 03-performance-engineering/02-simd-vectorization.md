---
doc_id: AE-03-02
title: SIMD Vectorization
status: active
last_verified: 2026-05-22
source_scope: data-parallel-kernel-optimization
depends_on:
  - AE-03-01
see_also:
  - AE-01-03
  - AE-05-03
  - AE-06-01
---

# SIMD Vectorization

## Scope
Engineering guide to vectorization through auto-vectorizers, directives, and explicit SIMD implementations.

## Decision Matrix
| Approach | Best use | Main risk |
| --- | --- | --- |
| Auto-vectorization | Regular loops with clear dependencies | Silent misses or unhelpful cost model decisions. |
| Directives / pragmas | Legally vectorizable loops needing hints | Incorrect assumptions about aliasing or alignment. |
| Intrinsics / explicit SIMD | Hot kernels requiring exact control | Portability and maintenance overhead. |

## Theory
- SIMD works when the computation can be expressed as concurrent lane-wise work with manageable divergence, reductions, and memory patterns. [SRC-009] [SRC-016]
- Loop vectorization and SLP vectorization attack different shapes: across loop iterations versus within scalar instruction bundles. [SRC-009]

## Production Reality
- Gather/scatter, masks, and mixed types make vectorization legal more often than before, but profitability still depends heavily on access regularity. [SRC-009] [SRC-017]
- Directives such as OpenMP simd or declare simd are valuable when you know something the compiler cannot infer safely. [SRC-015] [SRC-016]

## Optimization Patterns
- Write vectorizable scalar code first: simple loops, restricted aliasing, aligned data, and obvious reductions give compilers room to succeed. [SRC-017]
- Use compiler diagnostics aggressively: LLVM remarks and GCC opt-info should be part of the normal workflow for hot kernels. [SRC-010] [SRC-011]

## Failure Modes
- Forcing vector width without checking remainder handling, masks, and memory legality can produce slower code than the scalar baseline.
- Wider vectors are not automatically better; frequency throttling, bandwidth limits, or extra shuffles can erase the nominal advantage.

## Benchmark/Profiling Notes
- Measure both throughput and instruction mix; successful vectorization that becomes memory-bound may still be the correct result, but it changes the next bottleneck. [SRC-027]
- Inspect generated code or optimization remarks before attributing speed changes to SIMD; perceived vectorization is often imaginary. [SRC-009] [SRC-010]

## Related Docs
- [AE-01-03: Computational Models](../01-foundations/03-computational-models.md)
- [AE-05-03: Compiler Optimization](../05-systems/03-compiler-optimization.md)
- [AE-06-01: Profiler Stack](../06-tooling/01-profiler-stack.md)

## Sources used
- [SRC-009](https://llvm.org/docs/Vectorizers.html) - Auto-Vectorization in LLVM. LLVM documentation. 2026-05-08 verified.
- [SRC-016](https://www.openmp.org/spec-html/5.2/openmpse60.html) - simd Construct. OpenMP 5.2 specification section. 2026-05-17 verified.
- [SRC-017](https://www.intel.com/content/www/us/en/developer/articles/technical/vectorization-llvm-gcc-cpus-gpus.html) - SIMD vectorization in LLVM and GCC for Intel CPUs and GPUs. Intel technical article. 2026-05-22 verified.
- [SRC-055](https://www.openmp.org/wp-content/uploads/OpenMPRefGuide-5.2-Web-2024.pdf) - OpenMP API 5.2 Reference Guide. OpenMP reference card. 2024-04-01.
