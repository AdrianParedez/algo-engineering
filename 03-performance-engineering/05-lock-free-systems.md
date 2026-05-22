---
doc_id: AE-03-05
title: Lock-Free Systems
status: active
last_verified: 2026-05-22
source_scope: nonblocking-data-structures-and-atomics
depends_on:
  - AE-03-04
see_also:
  - AE-02-04
  - AE-05-05
  - AE-07-04
---

# Lock-Free Systems

## Scope
Engineering reference for nonblocking data structures, progress guarantees, reclamation, and the practical limits of lock-free design.

## Decision Matrix
| Property | Value | Engineering cost |
| --- | --- | --- |
| Lock-free | System-wide progress | High atomic and memory-order complexity. |
| Wait-free | Per-thread progress bound | Very high design complexity and specialization. |
| Obstruction-free | Progress in isolation | Often insufficient under real contention. |

## Theory
- Progress guarantees describe liveness, not speed; a lock-free structure can still perform badly under coherence traffic or retry storms.
- Memory reclamation is part of the algorithm, not an implementation afterthought, because safety and throughput couple tightly in nonblocking systems.

## Production Reality
- The real enemy is often contention topology: a logically lock-free design can bottleneck on one cache line or one highest-priority element. [SRC-054]
- Priority inversion and non-preemptible regions still matter at the system level, especially in latency-sensitive paths that interact with kernel scheduling. [SRC-029]

## Optimization Patterns
- Use lock-free designs where blocking latency is the real problem, not as a default replacement for competent sharded-lock designs.
- Sharding, batching, elimination, or relaxed semantics often beat heroic fully nonblocking structures for production throughput.

## Failure Modes
- ABA, reclamation bugs, and memory-order mistakes are common and can remain latent under light test conditions.
- A lock-free fast path with pathological tail behavior is a poor fit for predictable low-latency services.

## Related Docs
- [AE-02-04: Probabilistic Structures](../02-data-structures/04-probabilistic-structures.md)
- [AE-05-05: Low-Latency Systems](../05-systems/05-low-latency-systems.md)
- [AE-07-04: Frontier Directions](../07-research/04-frontier-directions.md)

## Sources used
- [SRC-054](https://arxiv.org/abs/2406.06900) - SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. arXiv paper. 2024-06-10.
- [SRC-029](https://docs.kernel.org/core-api/real-time/theory.html) - Theory of operation. Linux PREEMPT_RT documentation. 2026-05-08 verified.
