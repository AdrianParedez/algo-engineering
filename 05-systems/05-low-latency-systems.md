---
doc_id: AE-05-05
title: Low-Latency Systems
status: active
last_verified: 2026-05-22
source_scope: tail-latency-and-jitter-control
depends_on:
  - AE-03-03
see_also:
  - AE-06-04
  - AE-06-05
  - AE-92-01
---

# Low-Latency Systems

## Scope
Engineering standards for predictable latency: scheduler behavior, IRQ/threading model, power-state control, memory placement, and allocator tuning.

## Decision Matrix
| Latency source | Symptom | Mitigation class |
| --- | --- | --- |
| Scheduler / preemption | Wakeup delay and run-queue inflation | Priority policy, affinity, PREEMPT_RT where justified. |
| Power management | Jitter from deep idle states | PM QoS and controlled power policy. |
| Allocator / memory | Pause spikes and remote accesses | Heap policy, locality, and bounded lifetimes. |
| Shared contention | Tail expansion under load | Sharding, batching, and admission control. |

## Theory
- Low latency is a resource-governance problem, not only a fast-code problem: determinism depends on scheduler, interrupts, power states, memory, and shared resources. [SRC-028] [SRC-029] [SRC-030]
- Tail behavior should be treated as a first-class metric because the median often remains flat while service quality collapses.

## Production Reality
- PREEMPT_RT reduces non-preemptible regions and threads interrupt handling, but it is not a blanket speed boost; it is a determinism trade and must be justified. [SRC-029]
- cpu_dma_latency and similar controls can reduce wakeup variance by constraining power management, but they trade efficiency for responsiveness. [SRC-030]

## Optimization Patterns
- Measure queueing delay separately from execution time so scheduler and admission problems are not misdiagnosed as CPU inefficiency. [SRC-028]
- Use allocator and lifetime designs that bound fragmentation and cross-thread free behavior. [SRC-038]

## Failure Modes
- Optimizing only the hot function while ignoring IRQ, queueing, or power-state jitter rarely moves p99 or p999 latency.
- Aggressive pinning and throttling controls can stabilize one service while destabilizing the host for everything else.

## Related Docs
- [AE-06-04: Observability](../06-tooling/04-observability.md)
- [AE-06-05: Performance Workflows](../06-tooling/05-performance-workflows.md)
- [AE-92-01: Open Questions](../_tracking/01-open-questions.md)

## Sources used
- [SRC-028](https://www.brendangregg.com/usemethod.html) - The USE Method. Brendan Gregg methodology page. 2026-05-22 verified.
- [SRC-029](https://docs.kernel.org/core-api/real-time/theory.html) - Theory of operation. Linux PREEMPT_RT documentation. 2026-05-08 verified.
- [SRC-030](https://www.kernel.org/doc/html/latest/admin-guide/pm/cpuidle.html) - CPU Idle Time Management / PM QoS for CPUs. Linux kernel documentation. 2026-05-22 verified.
- [SRC-038](https://github.com/microsoft/mimalloc) - mimalloc README. GitHub repository documentation. 2026-04-29 release.
