---
doc_id: AE-91-03
title: Experiment Template
status: active
last_verified: 2026-05-22
source_scope: experiment-lifecycle-template
depends_on:
  - AE-08-04
see_also:
  - AE-91-02
  - AE-92-00
---

# Experiment Template

## Scope
Reusable template for experiments that combine benchmark data, profiles, counters, and explanatory claims.

## Decision Matrix
| Block | Purpose |
| --- | --- |
| Hypothesis | State predicted mechanism and direction. |
| Change set | Define exactly what varied. |
| Measurement plan | Benchmark and profiler stack. |
| Results | Raw and summarized evidence. |
| Interpretation | Mechanism-backed conclusion and next action. |

## Template Notes
- The interpretation section should be able to survive skeptical review by someone who did not run the experiment.
- Rejected ideas belong here too; they save time later.

## Skeleton

```md
# Experiment Record

## Hypothesis
- Predicted mechanism:
- Expected direction:

## Change Set
- Variable under test:
- Constants held fixed:

## Measurement Plan
- Benchmarks:
- Profiles/counters:

## Results
- Raw output:
- Summary:

## Interpretation
- Did the mechanism show up?
- Next action:
```

## Related Docs
- [AE-91-02: Benchmark Template](02-benchmark-template.md)
- [AE-92-00: Update Log](../_tracking/00-update-log.md)

## Sources used
- [SRC-021](https://bheisler.github.io/criterion.rs/book/analysis.html) - Criterion.rs Analysis Process. Criterion.rs documentation. 2026-05-08 verified.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
- [SRC-025](https://nnethercote.github.io/perf-book/benchmarking.html) - The Rust Performance Book: Benchmarking. Rust performance guide. 2026-05-08 verified.
