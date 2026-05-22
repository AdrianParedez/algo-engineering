---
doc_id: AE-92-02
title: Expansion Modules
status: active
last_verified: 2026-05-22
source_scope: future-module-planning
depends_on:
  - AE-08-02
see_also:
  - AE-08-01
  - AE-08-03
---

# Expansion Modules

## Scope
Suggested future modules beyond the initial workspace, prioritized by likely value for elite engineering practice.

## Decision Matrix
| Module | Why add it |
| --- | --- |
| GPU kernel engineering | Deepen heterogeneous compute guidance. |
| External-memory / out-of-core systems | Expand storage-aware algorithmics. |
| ANN and vector search | High-value modern retrieval workloads. |
| Formal verification for performance-critical code | Strengthen correctness under unsafe optimization. |
| eBPF performance tooling recipes | Operationalize kernel-level observability. |

## Planning Rule
- Add a module only when the source base is strong enough to sustain durable reference material rather than one-off notes.

## Candidate Modules

| Priority | Module | Prerequisite |
| --- | --- | --- |
| 1 | GPU kernel engineering | Need a stronger accelerator profiler and kernel-pattern source base. |
| 2 | ANN and vector search | Need a curated source set spanning HNSW, PQ, and serving systems. |
| 3 | eBPF performance recipes | Need hands-on reproducible workflows across kernels and distros. |

## Related Docs
- [AE-08-01: Learning Roadmap](../08-execution/01-learning-roadmap.md)
- [AE-08-03: Implementation Backlog](../08-execution/03-implementation-backlog.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-045](https://ebpf.io/what-is-ebpf/) - What is eBPF?. eBPF documentation. 2026-05-22 verified.
