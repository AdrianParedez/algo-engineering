---
doc_id: AE-08-01
title: Learning Roadmap
status: active
last_verified: 2026-05-22
source_scope: sequenced-study-plan
depends_on:
  - AE-00-00
see_also:
  - AE-08-05
  - AE-07-01
  - AE-92-02
---

# Learning Roadmap

## Scope
Sequenced path for building expert capability from foundations through measurement discipline, systems practice, and frontier work.

## Decision Matrix
| Phase | Primary outcome | Minimum evidence of completion |
| --- | --- | --- |
| 1. Foundations | Cost-model literacy | Can explain why asymptotics are insufficient in a target workload. |
| 2. Data and layout | Locality intuition | Can redesign a data path around movement costs. |
| 3. Measurement | Trustworthy experiments | Can produce a reproducible before/after benchmark package. |
| 4. Systems | Cross-layer reasoning | Can explain system bottlenecks with signals and models. |
| 5. Frontier | Research judgment | Can identify promising, testable open questions. |

## Roadmap
- Read methodology and measurement standards first; without them later optimization work becomes anecdotal. [SRC-001]
- Alternate theory-heavy documents with implementation-heavy case studies so abstractions are grounded quickly. [SRC-003]
- Every phase should end with one small experiment or code-reading exercise that produces a concrete artifact.

## Related Docs
- [AE-08-05: Skill Progression Matrix](05-skill-progression-matrix.md)
- [AE-07-01: Landmark Papers](../07-research/01-landmark-papers.md)
- [AE-92-02: Expansion Modules](../_tracking/02-expansion-modules.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
- [SRC-024](https://nnethercote.github.io/perf-book/profiling.html) - The Rust Performance Book: Profiling. Rust performance guide. 2026-05-08 verified.
