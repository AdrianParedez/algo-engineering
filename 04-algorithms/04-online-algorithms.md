---
doc_id: AE-04-04
title: Online Algorithms
status: active
last_verified: 2026-05-22
source_scope: irrevocable-decision-making-under-arrival
depends_on:
  - AE-01-05
see_also:
  - AE-04-03
  - AE-05-05
  - AE-08-03
---

# Online Algorithms

## Scope
Engineering perspective on algorithms that must act before seeing the future: caching, admission, scheduling, rebalancing, and resource control.

## Decision Matrix
| Setting | Decision pressure | Useful lens |
| --- | --- | --- |
| Caching / admission | Hit rate vs churn | Competitive ratio plus trace replay. |
| Scheduling | Latency vs throughput | Tail-aware workload traces. |
| Load balancing | Placement without full future load | Regret, recourse, or spill cost. |
| Autoscaling / control | Reactive stability | Hysteresis and measurement lag. |

## Theory
- Online algorithms replace full-information optimality with bounded regret, competitive analysis, or structurally constrained recourse.
- The main engineering implication is that decision quality must be evaluated against arrival traces and rollback cost, not only steady-state averages.

## Production Reality
- Most production systems are online by default because arrivals, load, and failures are only partially observable.
- State transitions and recourse matter: a policy that is locally optimal but highly disruptive can still be globally harmful.

## Optimization Patterns
- Combine lightweight online decisions with periodic offline reoptimization when the system can tolerate asynchronous correction.
- Replay real traces before trusting analytical guarantees; trace shape often matters more than abstract adversaries. [SRC-028]

## Failure Modes
- Policies tuned on aggregate averages often collapse under burstiness and tail-heavy arrivals.
- Overreactive control logic creates oscillation and self-inflicted latency.

## Related Docs
- [AE-04-03: Streaming Algorithms](03-streaming-algorithms.md)
- [AE-05-05: Low-Latency Systems](../05-systems/05-low-latency-systems.md)
- [AE-08-03: Implementation Backlog](../08-execution/03-implementation-backlog.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-028](https://www.brendangregg.com/usemethod.html) - The USE Method. Brendan Gregg methodology page. 2026-05-22 verified.
