---
doc_id: AE-91-02
title: Benchmark Template
status: active
last_verified: 2026-05-22
source_scope: benchmark-design-template
depends_on:
  - AE-03-06
see_also:
  - AE-91-03
  - AE-08-04
---

# Benchmark Template

## Scope
Reusable template for defining microbenchmarks and macrobenchmarks with environment, workload, and interpretation metadata.

## Decision Matrix
| Field | Required content |
| --- | --- |
| Work unit | Exact measured operation or scenario. |
| Setup isolation | What is excluded from timing and how. |
| Environment | Hardware, software, affinity, allocator, compiler. |
| Metrics | Wall time, counters, throughput, distribution. |
| Validity notes | Threats and known blind spots. |

## Template Notes
- Use this template when a benchmark may need to survive comparison across branches, machines, or releases.
- If setup isolation cannot be stated cleanly, the benchmark level is probably wrong.

## Skeleton

```md
# Benchmark Record

- Work unit:
- Harness:
- Build profile/preset:
- Hardware and affinity:
- Input corpus:
- Warmup policy:
- Measurement mode:
- Counters:
- Raw artifact path:
- Threats to validity:
```

## Related Docs
- [AE-91-03: Experiment Template](03-experiment-template.md)
- [AE-08-04: Verification Methodology](../08-execution/04-verification-methodology.md)

## Sources used
- [SRC-019](https://github.com/google/benchmark/blob/main/docs/user_guide.md) - google/benchmark user guide. GitHub documentation. 2026-05-15 verified.
- [SRC-021](https://bheisler.github.io/criterion.rs/book/analysis.html) - Criterion.rs Analysis Process. Criterion.rs documentation. 2026-05-08 verified.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
