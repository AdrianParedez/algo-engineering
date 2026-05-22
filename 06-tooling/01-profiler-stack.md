---
doc_id: AE-06-01
title: Profiler Stack
status: active
last_verified: 2026-05-22
source_scope: profiling-tool-selection
depends_on:
  - AE-03-06
see_also:
  - AE-06-04
  - AE-06-05
  - AE-08-04
---

# Profiler Stack

## Scope
Reference stack for selecting the right profiler by question type: hotspots, microarchitecture, allocations, concurrency, system noise, and production drift.

## Decision Matrix
| Question | Primary tool class | Why |
| --- | --- | --- |
| Where is CPU time going? | Sampling hotspot profiler | Low overhead and broad coverage. |
| Why is the CPU stalled? | Counter-driven microarchitecture profiler | Classifies front-end, core, and memory limits. |
| Where is memory allocated or leaked? | Heap / allocation profiler | Explains churn and footprint. |
| What is the system doing? | Kernel tracing / eBPF / perf | Sees scheduling, syscalls, and host-level effects. |
| Did production behavior drift? | Continuous profiler | Retains historical code-level resource usage. |

## Selection Rules
- Start with the lowest-overhead tool that can answer the current question. Do not begin with maximal tracing if a hotspot sample can eliminate most hypotheses. [SRC-024] [SRC-027]
- Use line-level or source-level detail only after the bottleneck class is established; otherwise profiling depth becomes noise.
- Continuous profiling is appropriate when the bottleneck is intermittent, deployment-sensitive, or visible only under production load. [SRC-044]

## Recommended Stack
- Linux CPU and system behavior: perf plus flamegraph-style visualization or a front-end such as Hotspot/Firefox Profiler. [SRC-023] [SRC-024]
- Deep CPU bottleneck analysis: VTune hotspots, memory access, and top-down views. [SRC-026] [SRC-027]
- Kernel and zero-instrumentation observability: eBPF-based tracing and profiling. [SRC-045]

## Failure Modes
- Profilers perturb reality differently; compare like with like when diffing before/after results.
- Sampling interval, unwind mode, and frame-pointer policy can materially change attribution quality.

## Related Docs
- [AE-06-04: Observability](04-observability.md)
- [AE-06-05: Performance Workflows](05-performance-workflows.md)
- [AE-08-04: Verification Methodology](../08-execution/04-verification-methodology.md)

## Sources used
- [SRC-023](https://docs.python.org/3.12/howto/perf_profiling.html) - Python support for the Linux perf profiler. Python documentation. 2026-05-21 verified.
- [SRC-024](https://nnethercote.github.io/perf-book/profiling.html) - The Rust Performance Book: Profiling. Rust performance guide. 2026-05-08 verified.
- [SRC-026](https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/current/memory-access-analysis.html) - Memory Access Analysis for Cache Misses and High Bandwidth Issues. Intel VTune documentation. 2025-07-01 verified.
- [SRC-027](https://www.intel.com/content/www/us/en/docs/vtune-profiler/cookbook/2024-0/top-down-microarchitecture-analysis-method.html) - Top-down Microarchitecture Analysis Method. Intel VTune cookbook. 2026-05-20 verified.
- [SRC-045](https://ebpf.io/what-is-ebpf/) - What is eBPF?. eBPF documentation. 2026-05-22 verified.
