---
doc_id: AE-08-02
title: Project Roadmap
status: active
last_verified: 2026-05-22
source_scope: workspace-application-roadmap
depends_on:
  - AE-08-01
see_also:
  - AE-08-03
  - AE-08-04
  - AE-92-00
---

# Project Roadmap

## Scope
Turns the knowledge system into an execution program: benchmark harnesses, reference implementations, profiling playbooks, and research replication tracks.

## Decision Matrix
| Milestone | Deliverable | Why it matters |
| --- | --- | --- |
| M1 | Reference microbenchmark suite | Builds measurement discipline early. |
| M2 | Cross-language algorithm kernels | Exposes layout and runtime differences. |
| M3 | Systems case studies | Connects algorithms to production architectures. |
| M4 | Research replications | Tests frontier ideas against local evidence. |

## Roadmap Logic
- Start with measurement infrastructure before large implementation campaigns so later work accumulates comparable evidence.
- Prefer narrow vertical slices that go from algorithm choice through benchmark and profile capture to final documentation. [SRC-001] [SRC-003]

## Related Docs
- [AE-08-03: Implementation Backlog](03-implementation-backlog.md)
- [AE-08-04: Verification Methodology](04-verification-methodology.md)
- [AE-92-00: Update Log](../_tracking/00-update-log.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
