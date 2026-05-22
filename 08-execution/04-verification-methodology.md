---
doc_id: AE-08-04
title: Verification Methodology
status: active
last_verified: 2026-05-22
source_scope: reproducible-experiment-standard
depends_on:
  - AE-03-06
see_also:
  - AE-06-02
  - AE-06-05
  - AE-91-03
---

# Verification Methodology

## Scope
Reproducible experimentation workflow for this workspace, from hypothesis and environment capture through result interpretation and archival.

## Decision Matrix
| Step | Required artifact | Hard failure |
| --- | --- | --- |
| Hypothesis | Explicit claim and predicted mechanism | No mechanism, only expected speedup. |
| Environment capture | Host, build, affinity, workload metadata | Cannot reconstruct setup. |
| Execution | Raw results plus benchmark config | Only summarized numbers retained. |
| Interpretation | Signal-backed explanation | Change cannot be linked to evidence. |
| Archival | Update log or reference doc delta | Result cannot be found later. |

## Standard
- Every experiment should answer three questions: what changed, how was it measured, and why should the effect exist. [SRC-021] [SRC-022]
- Verification requires both numerical evidence and mechanism evidence; a benchmark delta without profile or counter support is incomplete. [SRC-025]

## Artifact Requirements
- Retain raw samples or raw benchmark output, not only spreadsheets or screenshots. [SRC-021] [SRC-022]
- Record build profile, compiler version, preset/profile name, allocator policy, and input corpus identity. [SRC-019] [SRC-022]

## Related Docs
- [AE-06-02: Benchmark Frameworks](../06-tooling/02-benchmark-frameworks.md)
- [AE-06-05: Performance Workflows](../06-tooling/05-performance-workflows.md)
- [AE-91-03: Experiment Template](../_templates/03-experiment-template.md)

## Sources used
- [SRC-019](https://github.com/google/benchmark/blob/main/docs/user_guide.md) - google/benchmark user guide. GitHub documentation. 2026-05-15 verified.
- [SRC-021](https://bheisler.github.io/criterion.rs/book/analysis.html) - Criterion.rs Analysis Process. Criterion.rs documentation. 2026-05-08 verified.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
- [SRC-025](https://nnethercote.github.io/perf-book/benchmarking.html) - The Rust Performance Book: Benchmarking. Rust performance guide. 2026-05-08 verified.
