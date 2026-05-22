---
doc_id: AE-07-05
title: Reference Bibliography
status: active
last_verified: 2026-05-22
source_scope: canonical-bibliography
depends_on:
  - AE-90-00
see_also:
  - AE-07-01
  - AE-90-01
  - AE-90-02
---

# Reference Bibliography

## Scope
Canonical bibliography for the workspace. It consolidates the source registry into a durable reading list and citation target set.

## Decision Matrix
| Cluster | Examples | Use |
| --- | --- | --- |
| Methodology | SRC-001 to SRC-005 | Evidence and evaluation practice. |
| Memory and layout | SRC-006 to SRC-008 | Hierarchy-aware reasoning. |
| Compiler and SIMD | SRC-009 to SRC-018 | Vectorization and optimization internals. |
| Measurement and tooling | SRC-019 to SRC-045 | Benchmarking, profiling, observability, and systems. |
| Probabilistic and frontier | SRC-046 onward | Approximate structures and recent research. |

## Usage Note
- Use the registry for operational metadata and this document for reading and citation convenience.
- If a source is cited in a document, it should exist both here and in AE-90-00.

## Canonical Entries

- **SRC-001**. Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25. <https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download>.
- **SRC-002**. 10261 Executive Summary – Algorithm Engineering. Dagstuhl Seminar Proceedings. 2010-11-23. <https://drops.dagstuhl.de/entities/document/10.4230/DagSemProc.10261.2>.
- **SRC-003**. Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17. <https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169>.
- **SRC-004**. Selecting Problems for Algorithm Evaluation. LNCS workshop paper. 1999-07-19. <https://www.nzdr.ru/data/media/biblio/kolxoz/Cs/CsLn/Algorithm%20Engineering%2C%203%20conf.%2C%20WAE%20%2799%28LNCS1668%2C%20Springer%2C%201999%29%28ISBN%203540664270%29%28368s%29.pdf>.
- **SRC-005**. Experimental Algorithmics (Dagstuhl Seminar 02371). Dagstuhl Seminar Report. 2003-05-07. <https://drops.dagstuhl.de/entities/document/10.4230/DagSemRep.353>.
- **SRC-006**. The Input/Output Complexity of Sorting and Related Problems. Communications of the ACM article. 1988-09-01. <https://doi.org/10.1145/48529.48535>.
- **SRC-007**. Cache-Oblivious Algorithms. FOCS paper PDF. 1999-05-01. <https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf>.
- **SRC-008**. When Are Cache-Oblivious Algorithms Cache Adaptive? A Case Study of Matrix Multiplication and Sorting. ESA 2022 paper. 2022-09-01. <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.16>.
- **SRC-009**. Auto-Vectorization in LLVM. LLVM documentation. 2026-05-08 verified. <https://llvm.org/docs/Vectorizers.html>.
- **SRC-010**. Clang Compiler User’s Manual: Optimization Remarks. Clang documentation. 2026-05-16 verified. <https://clang.llvm.org/docs/UsersManual.html>.
- **SRC-011**. GCC Developer Options. GCC documentation. 2026-05-22 verified. <https://gcc.gnu.org/onlinedocs/gcc-7.2.0/gcc/Developer-Options.html>.
- **SRC-012**. MLIR Rationale. MLIR documentation. 2026-05-15 verified. <https://mlir.llvm.org/docs/Rationale/Rationale/>.
- **SRC-013**. Affine Dialect. MLIR documentation. 2026-05-16 verified. <https://mlir.llvm.org/docs/Dialects/Affine/>.
- **SRC-014**. MLIR Passes. MLIR documentation. 2026-05-08 verified. <https://mlir.llvm.org/docs/Passes/>.
- **SRC-015**. OpenMP Specifications. OpenMP specifications index. 2026-05-16 verified. <https://www.openmp.org/specifications/>.
- **SRC-016**. simd Construct. OpenMP 5.2 specification section. 2026-05-17 verified. <https://www.openmp.org/spec-html/5.2/openmpse60.html>.
- **SRC-017**. SIMD vectorization in LLVM and GCC for Intel CPUs and GPUs. Intel technical article. 2026-05-22 verified. <https://www.intel.com/content/www/us/en/developer/articles/technical/vectorization-llvm-gcc-cpus-gpus.html>.
- **SRC-018**. Software optimization resources. Agner Fog optimization portal. 2026-05-22 verified. <https://agner.org/optimize/>.
- **SRC-019**. google/benchmark user guide. GitHub documentation. 2026-05-15 verified. <https://github.com/google/benchmark/blob/main/docs/user_guide.md>.
- **SRC-020**. Criterion.rs Timing Loops. Criterion.rs documentation. 2026-05-08 verified. <https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html>.
- **SRC-021**. Criterion.rs Analysis Process. Criterion.rs documentation. 2026-05-08 verified. <https://bheisler.github.io/criterion.rs/book/analysis.html>.
- **SRC-022**. Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified. <https://pyperf.readthedocs.io/en/stable/system.html>.
- **SRC-023**. Python support for the Linux perf profiler. Python documentation. 2026-05-21 verified. <https://docs.python.org/3.12/howto/perf_profiling.html>.
- **SRC-024**. The Rust Performance Book: Profiling. Rust performance guide. 2026-05-08 verified. <https://nnethercote.github.io/perf-book/profiling.html>.
- **SRC-025**. The Rust Performance Book: Benchmarking. Rust performance guide. 2026-05-08 verified. <https://nnethercote.github.io/perf-book/benchmarking.html>.
- **SRC-026**. Memory Access Analysis for Cache Misses and High Bandwidth Issues. Intel VTune documentation. 2025-07-01 verified. <https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/current/memory-access-analysis.html>.
- **SRC-027**. Top-down Microarchitecture Analysis Method. Intel VTune cookbook. 2026-05-20 verified. <https://www.intel.com/content/www/us/en/docs/vtune-profiler/cookbook/2024-0/top-down-microarchitecture-analysis-method.html>.
- **SRC-028**. The USE Method. Brendan Gregg methodology page. 2026-05-22 verified. <https://www.brendangregg.com/usemethod.html>.
- **SRC-029**. Theory of operation. Linux PREEMPT_RT documentation. 2026-05-08 verified. <https://docs.kernel.org/core-api/real-time/theory.html>.
- **SRC-030**. CPU Idle Time Management / PM QoS for CPUs. Linux kernel documentation. 2026-05-22 verified. <https://www.kernel.org/doc/html/latest/admin-guide/pm/cpuidle.html>.
- **SRC-031**. FAQ: General run-time tuning. Open MPI FAQ. 2026-05-10 verified. <https://www.open-mpi.org/faq/?category=tuning>.
- **SRC-032**. The tuned collective component. Open MPI main documentation. 2026-05-22 verified. <https://docs.open-mpi.org/en/main/tuning-apps/collectives/tuned.html>.
- **SRC-033**. NVIDIA GB200 NVL Multi-Node Tuning Guide: NCCL. NVIDIA documentation. 2025-04-01 verified. <https://docs.nvidia.com/multi-node-nvlink-systems/multi-node-tuning-guide/nccl.html>.
- **SRC-034**. Execution Format. DuckDB internals documentation. 2026-05-15 verified. <https://duckdb.org/docs/current/internals/vector.html>.
- **SRC-035**. What is vectorised query execution?. ClickHouse engineering article. 2026-05-15 verified. <https://clickhouse.com/resources/engineering/vectorised-query-execution>.
- **SRC-036**. Performance Tuning Guide. PyTorch tutorial. 2025-07-09 updated. <https://docs.pytorch.org/tutorials/recipes/recipes/tuning_guide.html>.
- **SRC-037**. rayon crate documentation. Docs.rs crate documentation. 2025-10-18 verified. <https://docs.rs/rayon/latest/rayon/index.html>.
- **SRC-038**. mimalloc README. GitHub repository documentation. 2026-04-29 release. <https://github.com/microsoft/mimalloc>.
- **SRC-039**. jemalloc Background. GitHub wiki. 2023-06-01 verified. <https://github.com/jemalloc/jemalloc/wiki/Background>.
- **SRC-040**. Profiles. Cargo reference. 2026-05-18 verified. <https://doc.rust-lang.org/cargo/reference/profiles.html>.
- **SRC-041**. cmake-presets(7). CMake documentation. 2026-05-22 verified. <https://cmake.org/cmake/help/latest/manual/cmake-presets.7.html>.
- **SRC-042**. Meson built-in options. Meson documentation. 2026-05-08 verified. <https://mesonbuild.com/Builtin-options.html>.
- **SRC-043**. Signals. OpenTelemetry concepts. 2026-03-10 modified. <https://opentelemetry.io/docs/concepts/signals/>.
- **SRC-044**. Grafana Pyroscope overview. Grafana / Pyroscope documentation. 2026-05-16 verified. <https://grafana.com/docs/pyroscope/latest/>.
- **SRC-045**. What is eBPF?. eBPF documentation. 2026-05-22 verified. <https://ebpf.io/what-is-ebpf/>.
- **SRC-046**. Space/Time Trade-Offs in Hash Coding with Allowable Errors. Communications of the ACM article metadata. 1970-07-01. <https://www.scirp.org/reference/referencespapers?referenceid=1303589>.
- **SRC-047**. Cuckoo hashing. Journal of Algorithms article metadata. 2004-05-01. <https://colab.ws/articles/10.1016%2Fj.jalgor.2003.12.002>.
- **SRC-048**. An improved data stream summary: the count-min sketch and its applications. Journal of Algorithms article. 2005-04-01. <https://www.sciencedirect.com/science/article/pii/S0196677403001913>.
- **SRC-049**. HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm. AofA paper PDF mirror. 2007-06-01. <https://docslib.org/doc/7666800/hyperloglog-the-analysis-of-a-near-optimal-cardinality-estimation-algorithm>.
- **SRC-050**. Skip lists: a probabilistic alternative to balanced trees. Communications of the ACM article PDF mirror. 1990-06-01. <https://www.cs.ucdavis.edu/~amenta/w04/skiplists.pdf>.
- **SRC-051**. Simple Dynamic Spanners with Near-Optimal Recourse Against an Adaptive Adversary. ESA 2022 paper. 2022-09-01. <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2022.17>.
- **SRC-052**. Funnelselect: Cache-Oblivious Multiple Selection. ESA 2023 paper. 2023-09-01. <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ESA.2023.25>.
- **SRC-053**. Deterministic Cache-Oblivious Funnelselect. SWAT 2024 paper. 2024-07-01. <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SWAT.2024.17>.
- **SRC-054**. SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. arXiv paper. 2024-06-10. <https://arxiv.org/abs/2406.06900>.
- **SRC-055**. OpenMP API 5.2 Reference Guide. OpenMP reference card. 2024-04-01. <https://www.openmp.org/wp-content/uploads/OpenMPRefGuide-5.2-Web-2024.pdf>.

## Related Docs
- [AE-07-01: Landmark Papers](01-landmark-papers.md)
- [AE-90-01: Topic Source Map](../_registry/01-topic-source-map.md)
- [AE-90-02: Citation Style Guide](../_registry/02-citation-style-guide.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-015](https://www.openmp.org/specifications/) - OpenMP Specifications. OpenMP specifications index. 2026-05-16 verified.
