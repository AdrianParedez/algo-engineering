---
doc_id: AE-90-00
title: Source Catalog
status: active
last_verified: 2026-05-22
source_scope: canonical-source-metadata
depends_on:
  - none
see_also:
  - AE-90-01
  - AE-07-05
  - AE-90-02
---

# Source Catalog

## Scope
Canonical registry of source IDs, URLs, evidence class, topic tags, and notes. This is the metadata authority for the entire workspace.

## Decision Matrix
| Field | Meaning |
| --- | --- |
| SRC-ID | Stable citation handle used inline in docs. |
| Kind | What class of evidence the source represents. |
| Date | Publication or verification date used for churn control. |
| Topics | Primary subject tags used by the topic-source map. |

## Registry Rules
- Add a source here before citing it anywhere else in the workspace.
- Preserve exact dates for unstable tooling, runtime, or product-behavior claims.

## Catalog

| ID | Title | Kind | Date | Topics |
| --- | --- | --- | --- | --- |
| SRC-001 | Methodology of Algorithm Engineering | ACM Computing Surveys article | 2025-10-25 | algorithm-engineering, methodology |
| SRC-002 | 10261 Executive Summary – Algorithm Engineering | Dagstuhl Seminar Proceedings | 2010-11-23 | algorithm-engineering, history |
| SRC-003 | Algorithm Engineering (Dagstuhl Seminar 13391) | Dagstuhl Reports article | 2014-01-17 | algorithm-engineering, case-studies |
| SRC-004 | Selecting Problems for Algorithm Evaluation | LNCS workshop paper | 1999-07-19 | benchmarking, experimental-methodology |
| SRC-005 | Experimental Algorithmics (Dagstuhl Seminar 02371) | Dagstuhl Seminar Report | 2003-05-07 | experimental-algorithmics |
| SRC-006 | The Input/Output Complexity of Sorting and Related Problems | Communications of the ACM article | 1988-09-01 | external-memory, complexity |
| SRC-007 | Cache-Oblivious Algorithms | FOCS paper PDF | 1999-05-01 | cache-oblivious, memory-hierarchy |
| SRC-008 | When Are Cache-Oblivious Algorithms Cache Adaptive? A Case Study of Matrix Multiplication and Sorting | ESA 2022 paper | 2022-09-01 | cache-oblivious, empirical-study |
| SRC-009 | Auto-Vectorization in LLVM | LLVM documentation | 2026-05-08 verified | llvm, vectorization |
| SRC-010 | Clang Compiler User’s Manual: Optimization Remarks | Clang documentation | 2026-05-16 verified | clang, optimization-remarks |
| SRC-011 | GCC Developer Options | GCC documentation | 2026-05-22 verified | gcc, vectorization-diagnostics |
| SRC-012 | MLIR Rationale | MLIR documentation | 2026-05-15 verified | mlir, compiler-ir |
| SRC-013 | Affine Dialect | MLIR documentation | 2026-05-16 verified | mlir, affine, polyhedral |
| SRC-014 | MLIR Passes | MLIR documentation | 2026-05-08 verified | mlir, compiler-passes |
| SRC-015 | OpenMP Specifications | OpenMP specifications index | 2026-05-16 verified | openmp, parallelism |
| SRC-016 | simd Construct | OpenMP 5.2 specification section | 2026-05-17 verified | openmp, simd |
| SRC-017 | SIMD vectorization in LLVM and GCC for Intel CPUs and GPUs | Intel technical article | 2026-05-22 verified | simd, compiler-optimization |
| SRC-018 | Software optimization resources | Agner Fog optimization portal | 2026-05-22 verified | cpu-optimization, microarchitecture |
| SRC-019 | google/benchmark user guide | GitHub documentation | 2026-05-15 verified | benchmarking, c++ |
| SRC-020 | Criterion.rs Timing Loops | Criterion.rs documentation | 2026-05-08 verified | benchmarking, rust |
| SRC-021 | Criterion.rs Analysis Process | Criterion.rs documentation | 2026-05-08 verified | benchmarking, statistics |
| SRC-022 | Tune the system for benchmarks | pyperf documentation | 2026-05-22 verified | benchmarking, python, environment-control |
| SRC-023 | Python support for the Linux perf profiler | Python documentation | 2026-05-21 verified | python, profiling |
| SRC-024 | The Rust Performance Book: Profiling | Rust performance guide | 2026-05-08 verified | rust, profiling |
| SRC-025 | The Rust Performance Book: Benchmarking | Rust performance guide | 2026-05-08 verified | rust, benchmarking |
| SRC-026 | Memory Access Analysis for Cache Misses and High Bandwidth Issues | Intel VTune documentation | 2025-07-01 verified | vtune, memory-analysis |
| SRC-027 | Top-down Microarchitecture Analysis Method | Intel VTune cookbook | 2026-05-20 verified | vtune, top-down-analysis |
| SRC-028 | The USE Method | Brendan Gregg methodology page | 2026-05-22 verified | performance-methodology, observability |
| SRC-029 | Theory of operation | Linux PREEMPT_RT documentation | 2026-05-08 verified | low-latency, kernel-scheduling |
| SRC-030 | CPU Idle Time Management / PM QoS for CPUs | Linux kernel documentation | 2026-05-22 verified | low-latency, power-management |
| SRC-031 | FAQ: General run-time tuning | Open MPI FAQ | 2026-05-10 verified | mpi, hpc, affinity |
| SRC-032 | The tuned collective component | Open MPI main documentation | 2026-05-22 verified | mpi, collectives |
| SRC-033 | NVIDIA GB200 NVL Multi-Node Tuning Guide: NCCL | NVIDIA documentation | 2025-04-01 verified | nccl, multi-gpu, distributed-training |
| SRC-034 | Execution Format | DuckDB internals documentation | 2026-05-15 verified | database, vectorized-execution |
| SRC-035 | What is vectorised query execution? | ClickHouse engineering article | 2026-05-15 verified | database, vectorization |
| SRC-036 | Performance Tuning Guide | PyTorch tutorial | 2025-07-09 updated | ai-infrastructure, pytorch, performance |
| SRC-037 | rayon crate documentation | Docs.rs crate documentation | 2025-10-18 verified | rust, parallelism |
| SRC-038 | mimalloc README | GitHub repository documentation | 2026-04-29 release | allocator, low-latency, memory |
| SRC-039 | jemalloc Background | GitHub wiki | 2023-06-01 verified | allocator, memory |
| SRC-040 | Profiles | Cargo reference | 2026-05-18 verified | rust, build-systems, optimization |
| SRC-041 | cmake-presets(7) | CMake documentation | 2026-05-22 verified | cmake, build-systems |
| SRC-042 | Meson built-in options | Meson documentation | 2026-05-08 verified | meson, build-systems |
| SRC-043 | Signals | OpenTelemetry concepts | 2026-03-10 modified | observability, telemetry |
| SRC-044 | Grafana Pyroscope overview | Grafana / Pyroscope documentation | 2026-05-16 verified | continuous-profiling, observability |
| SRC-045 | What is eBPF? | eBPF documentation | 2026-05-22 verified | ebpf, observability, kernel |
| SRC-046 | Space/Time Trade-Offs in Hash Coding with Allowable Errors | Communications of the ACM article metadata | 1970-07-01 | probabilistic-data-structures, bloom-filter |
| SRC-047 | Cuckoo hashing | Journal of Algorithms article metadata | 2004-05-01 | probabilistic-data-structures, hashing |
| SRC-048 | An improved data stream summary: the count-min sketch and its applications | Journal of Algorithms article | 2005-04-01 | streaming, sketches |
| SRC-049 | HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm | AofA paper PDF mirror | 2007-06-01 | streaming, cardinality-estimation |
| SRC-050 | Skip lists: a probabilistic alternative to balanced trees | Communications of the ACM article PDF mirror | 1990-06-01 | probabilistic-data-structures, search-structures |
| SRC-051 | Simple Dynamic Spanners with Near-Optimal Recourse Against an Adaptive Adversary | ESA 2022 paper | 2022-09-01 | graph-algorithms, dynamic-algorithms |
| SRC-052 | Funnelselect: Cache-Oblivious Multiple Selection | ESA 2023 paper | 2023-09-01 | cache-oblivious, selection |
| SRC-053 | Deterministic Cache-Oblivious Funnelselect | SWAT 2024 paper | 2024-07-01 | cache-oblivious, selection |
| SRC-054 | SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures | arXiv paper | 2024-06-10 | concurrency, numa, priority-queues |
| SRC-055 | OpenMP API 5.2 Reference Guide | OpenMP reference card | 2024-04-01 | openmp, reference |

## Notes

- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. Open-access survey used as the primary methodological anchor.
- [SRC-002](https://drops.dagstuhl.de/entities/document/10.4230/DagSemProc.10261.2) - 10261 Executive Summary – Algorithm Engineering. Short definition and research-directions summary.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Highlights success stories and the design-analysis-implementation-experiment cycle.
- [SRC-004](https://www.nzdr.ru/data/media/biblio/kolxoz/Cs/CsLn/Algorithm%20Engineering%2C%203%20conf.%2C%20WAE%20%2799%28LNCS1668%2C%20Springer%2C%201999%29%28ISBN%203540664270%29%28368s%29.pdf) - Selecting Problems for Algorithm Evaluation. Classic guidance on test-set construction and evaluation design.
- [SRC-005](https://drops.dagstuhl.de/entities/document/10.4230/DagSemRep.353) - Experimental Algorithmics (Dagstuhl Seminar 02371). Historical background for experimental methods.
- [SRC-006](https://doi.org/10.1145/48529.48535) - The Input/Output Complexity of Sorting and Related Problems. Canonical external-memory complexity reference.
- [SRC-007](https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf) - Cache-Oblivious Algorithms. Seminal cache-oblivious model and algorithm paper.
- [SRC-008](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.16) - When Are Cache-Oblivious Algorithms Cache Adaptive? A Case Study of Matrix Multiplication and Sorting. Recent empirical study on cache adaptivity.
- [SRC-009](https://llvm.org/docs/Vectorizers.html) - Auto-Vectorization in LLVM. Primary source for Loop Vectorizer and SLP Vectorizer behavior.
- [SRC-010](https://clang.llvm.org/docs/UsersManual.html) - Clang Compiler User’s Manual: Optimization Remarks. Authoritative reference for -Rpass, -Rpass-missed, and -Rpass-analysis.
- [SRC-011](https://gcc.gnu.org/onlinedocs/gcc-7.2.0/gcc/Developer-Options.html) - GCC Developer Options. Reference for -fopt-info and vectorization diagnostics.
- [SRC-012](https://mlir.llvm.org/docs/Rationale/Rationale/) - MLIR Rationale. Design rationale for multi-level IR and polyhedral-style transformations.
- [SRC-013](https://mlir.llvm.org/docs/Dialects/Affine/) - Affine Dialect. Primary source for affine loops, vector loads/stores, and dependence-friendly IR.
- [SRC-014](https://mlir.llvm.org/docs/Passes/) - MLIR Passes. Primary source for affine tiling, fusion, unroll, and parallelization passes.
- [SRC-015](https://www.openmp.org/specifications/) - OpenMP Specifications. Landing page for current and prior specifications and examples.
- [SRC-016](https://www.openmp.org/spec-html/5.2/openmpse60.html) - simd Construct. Normative definition of the simd construct.
- [SRC-017](https://www.intel.com/content/www/us/en/developer/articles/technical/vectorization-llvm-gcc-cpus-gpus.html) - SIMD vectorization in LLVM and GCC for Intel CPUs and GPUs. Practical discussion of auto-vectorization and programmer-guided SIMD.
- [SRC-018](https://agner.org/optimize/) - Software optimization resources. Reference hub for x86 optimization manuals and instruction tables.
- [SRC-019](https://github.com/google/benchmark/blob/main/docs/user_guide.md) - google/benchmark user guide. Primary guide for Google Benchmark.
- [SRC-020](https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html) - Criterion.rs Timing Loops. Operational details for timing-loop selection and overhead.
- [SRC-021](https://bheisler.github.io/criterion.rs/book/analysis.html) - Criterion.rs Analysis Process. Primary explanation of warmup, sampling, outliers, and bootstrap analysis.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. Operational guide for benchmark environment stabilization.
- [SRC-023](https://docs.python.org/3.12/howto/perf_profiling.html) - Python support for the Linux perf profiler. Official CPython perf-profiling guide.
- [SRC-024](https://nnethercote.github.io/perf-book/profiling.html) - The Rust Performance Book: Profiling. Primary guidance for profiling Rust release builds.
- [SRC-025](https://nnethercote.github.io/perf-book/benchmarking.html) - The Rust Performance Book: Benchmarking. Workload and measurement advice for Rust performance work.
- [SRC-026](https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/current/memory-access-analysis.html) - Memory Access Analysis for Cache Misses and High Bandwidth Issues. Primary source for VTune memory-access analysis.
- [SRC-027](https://www.intel.com/content/www/us/en/docs/vtune-profiler/cookbook/2024-0/top-down-microarchitecture-analysis-method.html) - Top-down Microarchitecture Analysis Method. Primary source for front-end/core/memory/bad-speculation analysis.
- [SRC-028](https://www.brendangregg.com/usemethod.html) - The USE Method. Resource-oriented performance triage method.
- [SRC-029](https://docs.kernel.org/core-api/real-time/theory.html) - Theory of operation. Primary source for PREEMPT_RT behavior and latency reduction model.
- [SRC-030](https://www.kernel.org/doc/html/latest/admin-guide/pm/cpuidle.html) - CPU Idle Time Management / PM QoS for CPUs. Primary source for cpu_dma_latency and PM QoS behavior.
- [SRC-031](https://www.open-mpi.org/faq/?category=tuning) - FAQ: General run-time tuning. Primary reference for process and memory affinity in Open MPI.
- [SRC-032](https://docs.open-mpi.org/en/main/tuning-apps/collectives/tuned.html) - The tuned collective component. Primary source for tuned collectives and decision trees.
- [SRC-033](https://docs.nvidia.com/multi-node-nvlink-systems/multi-node-tuning-guide/nccl.html) - NVIDIA GB200 NVL Multi-Node Tuning Guide: NCCL. Primary source for modern NCCL tuning guidance.
- [SRC-034](https://duckdb.org/docs/current/internals/vector.html) - Execution Format. Primary description of DuckDB vectors and data chunks.
- [SRC-035](https://clickhouse.com/resources/engineering/vectorised-query-execution) - What is vectorised query execution?. Concise distinction between vectorization as execution model and SIMD as inner-loop mechanism.
- [SRC-036](https://docs.pytorch.org/tutorials/recipes/recipes/tuning_guide.html) - Performance Tuning Guide. Official performance guide for training and inference.
- [SRC-037](https://docs.rs/rayon/latest/rayon/index.html) - rayon crate documentation. Primary API reference for data parallelism in Rust.
- [SRC-038](https://github.com/microsoft/mimalloc) - mimalloc README. Production allocator design and tuning notes.
- [SRC-039](https://github.com/jemalloc/jemalloc/wiki/Background) - jemalloc Background. Allocator history, intended use, and adoption notes.
- [SRC-040](https://doc.rust-lang.org/cargo/reference/profiles.html) - Profiles. Official profile tuning reference for Cargo.
- [SRC-041](https://cmake.org/cmake/help/latest/manual/cmake-presets.7.html) - cmake-presets(7). Authoritative description of presets and workflow files.
- [SRC-042](https://mesonbuild.com/Builtin-options.html) - Meson built-in options. Official guide to optimization, debug, and LTO options.
- [SRC-043](https://opentelemetry.io/docs/concepts/signals/) - Signals. Primary reference for traces, metrics, logs, and profiles as signals.
- [SRC-044](https://grafana.com/docs/pyroscope/latest/) - Grafana Pyroscope overview. Primary source for continuous profiling architecture and signal integration.
- [SRC-045](https://ebpf.io/what-is-ebpf/) - What is eBPF?. Primary source for eBPF capabilities and execution model.
- [SRC-046](https://www.scirp.org/reference/referencespapers?referenceid=1303589) - Space/Time Trade-Offs in Hash Coding with Allowable Errors. DOI-backed metadata for the original Bloom filter paper.
- [SRC-047](https://colab.ws/articles/10.1016%2Fj.jalgor.2003.12.002) - Cuckoo hashing. Metadata for the original cuckoo hashing paper.
- [SRC-048](https://www.sciencedirect.com/science/article/pii/S0196677403001913) - An improved data stream summary: the count-min sketch and its applications. Primary count-min sketch paper landing page.
- [SRC-049](https://docslib.org/doc/7666800/hyperloglog-the-analysis-of-a-near-optimal-cardinality-estimation-algorithm) - HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm. Accessible copy of the original HyperLogLog paper.
- [SRC-050](https://www.cs.ucdavis.edu/~amenta/w04/skiplists.pdf) - Skip lists: a probabilistic alternative to balanced trees. Accessible copy of the original skip list paper.
- [SRC-051](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.17) - Simple Dynamic Spanners with Near-Optimal Recourse Against an Adaptive Adversary. Representative recent dynamic graph result.
- [SRC-052](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2023.25) - Funnelselect: Cache-Oblivious Multiple Selection. Representative modern cache-oblivious result.
- [SRC-053](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SWAT.2024.17) - Deterministic Cache-Oblivious Funnelselect. Follow-on deterministic result.
- [SRC-054](https://arxiv.org/abs/2406.06900) - SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. Representative recent NUMA-aware concurrent data structure paper.
- [SRC-055](https://www.openmp.org/wp-content/uploads/OpenMPRefGuide-5.2-Web-2024.pdf) - OpenMP API 5.2 Reference Guide. Compact operational reference for clauses and constructs.

## Related Docs
- [AE-90-01: Topic Source Map](01-topic-source-map.md)
- [AE-07-05: Reference Bibliography](../07-research/05-reference-bibliography.md)
- [AE-90-02: Citation Style Guide](02-citation-style-guide.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-015](https://www.openmp.org/specifications/) - OpenMP Specifications. OpenMP specifications index. 2026-05-16 verified.
