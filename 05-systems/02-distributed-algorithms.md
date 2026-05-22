---
doc_id: AE-05-02
title: Distributed Algorithms
status: active
last_verified: 2026-05-22
source_scope: cluster-scale-execution-and-communication
depends_on:
  - AE-01-03
see_also:
  - AE-05-04
  - AE-06-04
  - AE-06-05
---

# Distributed Algorithms

## Scope
Practical view of partitioning, collectives, topology awareness, batching, and coordination overhead in cluster and multi-node environments.

## Decision Matrix
| Concern | Good default | Escalate when |
| --- | --- | --- |
| Partitioning | Hash or range partition by dominant access key | Skew or hotspot shards dominate. |
| Collectives | Runtime-chosen tuned algorithms | Message size or topology repeatedly defeats defaults. |
| Affinity / topology | Respect sockets, NUMA, and link domains | Cross-domain traffic dominates runtime. |
| Backpressure | Bound queues and batch sizes | Tail latency or memory blowup appears. |

## Theory
- Distributed algorithms pay in communication what they save in local work; partitioning and collective structure become part of the algorithm, not just the runtime. [SRC-031] [SRC-032]
- Topology awareness matters because bandwidth and latency are not uniform across sockets, nodes, or accelerator interconnects. [SRC-033]

## Production Reality
- The fastest local kernel can be irrelevant if batch boundaries, serialization, or collective mismatch dominate end-to-end time.
- Runtime defaults are often strong baselines; tune only after measuring where the default decision tree is wrong. [SRC-032] [SRC-033]

## Optimization Patterns
- Exploit mergeable partial results and hierarchical reductions to match network topology.
- Instrument queue depths, batch sizes, retry paths, and cross-domain traffic, not only task completion time.

## Failure Modes
- Uniform partitioning assumptions fail under key skew, stragglers, or changing hot sets.
- Over-tuning transport parameters without measuring topology-level symptoms leads to configuration folklore.

## Related Docs
- [AE-05-04: AI and ML Infrastructure](04-ai-and-ml-infrastructure.md)
- [AE-06-04: Observability](../06-tooling/04-observability.md)
- [AE-06-05: Performance Workflows](../06-tooling/05-performance-workflows.md)

## Sources used
- [SRC-031](https://www.open-mpi.org/faq/?category=tuning) - FAQ: General run-time tuning. Open MPI FAQ. 2026-05-10 verified.
- [SRC-032](https://docs.open-mpi.org/en/main/tuning-apps/collectives/tuned.html) - The tuned collective component. Open MPI main documentation. 2026-05-22 verified.
- [SRC-033](https://docs.nvidia.com/multi-node-nvlink-systems/multi-node-tuning-guide/nccl.html) - NVIDIA GB200 NVL Multi-Node Tuning Guide: NCCL. NVIDIA documentation. 2025-04-01 verified.
