---
doc_id: AE-05-04
title: AI and ML Infrastructure
status: active
last_verified: 2026-05-22
source_scope: model-serving-and-training-systems
depends_on:
  - AE-05-02
see_also:
  - AE-05-03
  - AE-06-04
  - AE-07-04
---

# AI and ML Infrastructure

## Scope
Algorithm-engineering patterns for training and inference systems: batching, fusion, placement, communication overlap, and profiling across heterogeneous devices.

## Decision Matrix
| Subsystem | Primary bottleneck | Usual lever |
| --- | --- | --- |
| Input pipeline | Host-side stalls | Prefetching and overlap. |
| Kernel execution | Underutilized vector/Tensor units | Fusion, layout, and specialized kernels. |
| Distributed training | All-reduce and synchronization | Topology-aware collectives and overlap. |
| Serving | Tail latency under batch pressure | Adaptive batching and model placement. |

## Theory
- AI infrastructure is dominated by dataflow graphs, dense linear algebra kernels, and communication-heavy synchronization loops; structure matters at every layer. [SRC-012] [SRC-033]
- The effective algorithm is the composition of graph rewrite, kernel selection, memory placement, and communication schedule.

## Production Reality
- Input staging and host orchestration frequently cap throughput before the accelerator is saturated. [SRC-036]
- Distributed training performance depends on collective choice, interconnect topology, and the ability to overlap communication with compute. [SRC-033]

## Optimization Patterns
- Batch only until it improves effective device utilization without violating latency or memory targets.
- Push transformations upward when possible: graph-level fusion and layout normalization often unlock more than kernel-local tinkering. [SRC-012] [SRC-036]

## Failure Modes
- Using one benchmark shape or one model architecture as a universal proxy leads to brittle capacity planning and mis-tuned serving behavior.
- Ignoring observability at span or profile level makes it hard to distinguish kernel inefficiency from orchestration inefficiency.

## Related Docs
- [AE-05-03: Compiler Optimization](03-compiler-optimization.md)
- [AE-06-04: Observability](../06-tooling/04-observability.md)
- [AE-07-04: Frontier Directions](../07-research/04-frontier-directions.md)

## Sources used
- [SRC-033](https://docs.nvidia.com/multi-node-nvlink-systems/multi-node-tuning-guide/nccl.html) - NVIDIA GB200 NVL Multi-Node Tuning Guide: NCCL. NVIDIA documentation. 2025-04-01 verified.
- [SRC-036](https://docs.pytorch.org/tutorials/recipes/recipes/tuning_guide.html) - Performance Tuning Guide. PyTorch tutorial. 2025-07-09 updated.
- [SRC-012](https://mlir.llvm.org/docs/Rationale/Rationale/) - MLIR Rationale. MLIR documentation. 2026-05-15 verified.
