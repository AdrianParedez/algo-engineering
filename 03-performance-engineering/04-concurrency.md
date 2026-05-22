---
doc_id: AE-03-04
title: Concurrency
status: active
last_verified: 2026-05-22
source_scope: shared-memory-parallelism-and-scheduling
depends_on:
  - AE-01-03
see_also:
  - AE-03-05
  - AE-05-02
  - AE-06-05
---

# Concurrency

## Scope
Guide to thread-level parallelism, task decomposition, affinity, grain size, and work scheduling on multicore shared-memory systems.

## Decision Matrix
| Concern | Good default | Escalate when |
| --- | --- | --- |
| Task decomposition | Chunk by independent work | Skew or dependencies dominate. |
| Scheduling | Work-stealing or dynamic partitioning | NUMA and cache locality become the bottleneck. |
| Affinity | Pin only critical steady-state workers | Migration noise or remote traffic are measurable. |
| Sharing | Shard mutable state | Hot counters or queues serialize progress. |

## Theory
- Concurrency performance is bounded by available parallel work and by overheads introduced to expose it: scheduling, synchronization, communication, and locality loss.
- Work-stealing is a strong default for irregular parallelism, but locality-aware scheduling can dominate once NUMA becomes visible. [SRC-037] [SRC-054]

## Production Reality
- The hardest problems are usually load imbalance, false sharing, and queue contention, not missing threads.
- Library ergonomics can hide scheduling cost; parallel iterators or task pools remain subject to grain-size and memory-placement reality. [SRC-037]

## Optimization Patterns
- Make grain size an explicit tuning surface and benchmark it; too fine creates overhead, too coarse creates imbalance.
- Reduce shared mutable state before reaching for more advanced synchronization primitives.

## Failure Modes
- Parallel speedups that vanish under pinned cores or production-like memory pressure usually indicate locality or contention debt, not a bad CPU.
- Oversubscription can make apparent improvements disappear through scheduler noise and cache thrashing. [SRC-031]

## Related Docs
- [AE-03-05: Lock-Free Systems](05-lock-free-systems.md)
- [AE-05-02: Distributed Algorithms](../05-systems/02-distributed-algorithms.md)
- [AE-06-05: Performance Workflows](../06-tooling/05-performance-workflows.md)

## Sources used
- [SRC-031](https://www.open-mpi.org/faq/?category=tuning) - FAQ: General run-time tuning. Open MPI FAQ. 2026-05-10 verified.
- [SRC-037](https://docs.rs/rayon/latest/rayon/index.html) - rayon crate documentation. Docs.rs crate documentation. 2025-10-18 verified.
- [SRC-054](https://arxiv.org/abs/2406.06900) - SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. arXiv paper. 2024-06-10.
