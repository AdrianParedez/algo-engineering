---
doc_id: AE-00-99
title: Algorithm Engineering Workspace
status: active
last_verified: 2026-05-22
source_scope: workspace-navigation-and-methodology
depends_on:
  - AE-00-01
  - AE-00-03
see_also:
  - AE-00-00
  - AE-08-01
  - AE-90-00
---

# Algorithm Engineering Workspace

## Scope
Entry point for the long-horizon algorithm-engineering knowledge system. It defines what the workspace is for, how to navigate it, and how to treat its contents as a decision support layer for implementation and experimentation rather than as a passive notes dump.

## Decision Matrix
| Need | Start here | Why |
| --- | --- | --- |
| Orient to the corpus | AE-00-01 | Central index and navigation graph. |
| Understand evidence standards | AE-00-03 | Explains source ranking, synthesis rules, and validity concerns. |
| Run experiments | AE-08-04 | Operational verification workflow and reproducibility rules. |
| Expand the workspace | AE-92-02 | Future modules and extension priorities. |

## Design Principles
- The workspace treats algorithm engineering as a cycle of design, analysis, implementation, and experiment, with falsifiable hypotheses and realistic workloads driving iteration. [SRC-001] [SRC-003]
- Each document is optimized for implementation decisions: what to choose, why it works, what breaks first, and how to measure whether the choice was actually correct. [SRC-001] [SRC-004]
- Time-sensitive tooling and production-practice claims are frozen to the verification date in front matter so later updates can be appended deliberately instead of silently drifting. [SRC-022] [SRC-041]

## Workspace Layout
| Layer | Purpose |
| --- | --- |
| 00-meta | Navigation, taxonomy, methodology, and ingestion rules. |
| 01-foundations to 04-algorithms | Core conceptual and algorithmic reference material. |
| 05-systems and 06-tooling | Applied engineering patterns and operational stacks. |
| 07-research and 08-execution | Research frontier, roadmap, backlog, and verification workflows. |
| _registry, _templates, _tracking | Support metadata, reusable templates, and ongoing maintenance. |


## Operating Rules
- Prefer the smallest model that captures the dominant cost: wall time alone is rarely enough once cache, branch, NUMA, synchronization, or I/O effects dominate. [SRC-006] [SRC-007] [SRC-027]
- Do not treat benchmark results as portable facts unless the workload, compiler, allocator, topology, and affinity policy are recorded. [SRC-004] [SRC-019] [SRC-022]
- Use the registry and bibliography as the canonical source layer; if a claim matters and is not traceable there, it should be considered provisional. [SRC-001]

## Related Docs
- [AE-00-00: Workspace Index](00-meta/00-index.md)
- [AE-08-01: Learning Roadmap](08-execution/01-learning-roadmap.md)
- [AE-90-00: Source Catalog](_registry/00-source-catalog.md)

## Repository Operations
- Contribution workflow: [CONTRIBUTING.md](CONTRIBUTING.md)
- Security reporting policy: [SECURITY.md](SECURITY.md)
- GitHub publication standards: [docs/github-repository-standards.md](docs/github-repository-standards.md)
- Remote creation and post-publish checklist: [docs/repository-publish-runbook.md](docs/repository-publish-runbook.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
- [SRC-004](https://www.nzdr.ru/data/media/biblio/kolxoz/Cs/CsLn/Algorithm%20Engineering%2C%203%20conf.%2C%20WAE%20%2799%28LNCS1668%2C%20Springer%2C%201999%29%28ISBN%203540664270%29%28368s%29.pdf) - Selecting Problems for Algorithm Evaluation. LNCS workshop paper. 1999-07-19.
