---
doc_id: AE-06-05
title: Performance Workflows
status: active
last_verified: 2026-05-22
source_scope: standard-operating-procedure-for-tuning
depends_on:
  - AE-06-01
  - AE-06-02
see_also:
  - AE-08-03
  - AE-08-04
  - AE-92-00
---

# Performance Workflows

## Scope
Standard operating workflow for algorithm and systems performance work in this workspace.

## Decision Matrix
| Stage | Primary output | Gate to proceed |
| --- | --- | --- |
| Triage | Bottleneck hypothesis | Signal evidence exists. |
| Measurement | Stable benchmark or profile baseline | Environment and workload recorded. |
| Change | Isolated intervention | Variable under test is explicit. |
| Verification | Before/after result with explanation | Result reproduces and aligns with signals. |
| Capture | Documented pattern or rejected idea | Knowledge archived. |

## Workflow
- Profile before optimizing, benchmark before and after, and archive both successful and rejected interventions. [SRC-001] [SRC-024]
- Keep one primary variable per run when possible: algorithm, layout, compiler setting, allocator, concurrency policy, or topology choice.
- If the explanation for a speedup is weaker than the measurement, treat the result as provisional.

## Review Standard
- A performance claim should be rejected if the benchmark is noisy, the environment is ambiguous, or the proposed mechanism is unsupported by counters or profiles. [SRC-022] [SRC-028]
- A result is not “done” until its lesson is promoted into stable reference material or tracked explicitly as an unresolved question.

## Related Docs
- [AE-08-03: Implementation Backlog](../08-execution/03-implementation-backlog.md)
- [AE-08-04: Verification Methodology](../08-execution/04-verification-methodology.md)
- [AE-92-00: Update Log](../_tracking/00-update-log.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
- [SRC-024](https://nnethercote.github.io/perf-book/profiling.html) - The Rust Performance Book: Profiling. Rust performance guide. 2026-05-08 verified.
- [SRC-028](https://www.brendangregg.com/usemethod.html) - The USE Method. Brendan Gregg methodology page. 2026-05-22 verified.
