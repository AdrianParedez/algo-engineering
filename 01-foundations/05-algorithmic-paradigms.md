---
doc_id: AE-01-05
title: Algorithmic Paradigms
status: active
last_verified: 2026-05-22
source_scope: paradigms-and-selection-criteria
depends_on:
  - AE-01-01
see_also:
  - AE-04-03
  - AE-04-05
  - AE-04-06
---

# Algorithmic Paradigms

## Scope
Selection guide for major algorithmic paradigms through an engineering lens: not just how they work, but when their structure matches real constraints and hardware.

## Decision Matrix
| Paradigm | Strength | Typical cost driver |
| --- | --- | --- |
| Divide and conquer | Locality and parallel decomposition | Recursion overhead and merge traffic. |
| Dynamic programming | Optimal substructure exploitation | State explosion and memory footprint. |
| Greedy | Simple fast decisions | Proof fragility and adversarial inputs. |
| Randomized | Load balancing and robustness | Variance and repeatability. |
| Approximation | Scales beyond exact methods | Solution-quality calibration. |
| Streaming / online | Bounded memory and immediacy | Approximation and irrevocability. |

## Theory
- Paradigms are reusable problem decompositions. Good algorithm engineering matches paradigm structure to bottleneck structure rather than treating paradigms as style preferences. [SRC-001] [SRC-003]
- Modern systems frequently combine paradigms: e.g., streaming sketches upstream, approximate search in the middle, exact verification downstream.

## Production Reality
- The best-performing paradigm is often the one that constrains memory traffic and synchronization, even when another paradigm wins on textbook asymptotics.
- Engineering simplicity matters: a slightly weaker paradigm with clearer failure modes is often preferable in long-lived production code. [SRC-004]

## Optimization Patterns
- Choose paradigms with explicit degradation behavior under skew, limited memory, or concurrency rather than relying on nominal average-case assumptions.
- Expose paradigm boundaries in code so different implementations can be benchmarked under the same workload harness. [SRC-004] [SRC-022]

## When Not To Use
- Do not force dynamic programming into problems whose state representation destroys locality or exceeds cache and memory budgets.
- Do not use randomized or approximate methods in correctness-critical paths unless verification or guardrail stages exist.

## Related Docs
- [AE-04-03: Streaming Algorithms](../04-algorithms/03-streaming-algorithms.md)
- [AE-04-05: Approximation Algorithms](../04-algorithms/05-approximation-algorithms.md)
- [AE-04-06: Randomized Algorithms](../04-algorithms/06-randomized-algorithms.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-002](https://drops.dagstuhl.de/entities/document/10.4230/DagSemProc.10261.2) - 10261 Executive Summary – Algorithm Engineering. Dagstuhl Seminar Proceedings. 2010-11-23.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
