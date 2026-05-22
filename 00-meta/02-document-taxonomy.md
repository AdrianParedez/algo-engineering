---
doc_id: AE-00-02
title: Document Taxonomy
status: active
last_verified: 2026-05-22
source_scope: taxonomy-and-classification
depends_on:
  - AE-00-01
see_also:
  - AE-00-03
  - AE-07-05
  - AE-90-01
---

# Document Taxonomy

## Scope
Classifies the corpus by document intent, evidence strength, temporal stability, and execution usefulness so new additions land in the right layer.

## Decision Matrix
| Axis | Classes | Interpretation |
| --- | --- | --- |
| Intent | reference, operational, research, execution | What the document is for. |
| Evidence | primary, synthesized, provisional | How directly claims map to sources. |
| Temporal stability | stable, medium churn, rapid churn | How often content should be revalidated. |
| Actionability | conceptual, tactical, operational | How close the document is to code or experiments. |

## Taxonomy Rules
- A reference document explains durable models and tradeoffs; an operational document specifies repeatable practice; a research document tracks the frontier; an execution document tells what to do next. [SRC-001] [SRC-003]
- Evidence strength should be explicit: primary for direct source-backed claims, synthesized for cross-source inference, provisional for hypotheses parked in tracking. [SRC-001]
- Rapid-churn topics include tooling, compiler behavior, profiler features, and distributed runtime defaults; they require date-stamped verification. [SRC-009] [SRC-022] [SRC-031]

## Related Docs
- [AE-00-03: Research Methodology](03-research-methodology.md)
- [AE-07-05: Reference Bibliography](../07-research/05-reference-bibliography.md)
- [AE-90-01: Topic Source Map](../_registry/01-topic-source-map.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
- [SRC-005](https://drops.dagstuhl.de/entities/document/10.4230/DagSemRep.353) - Experimental Algorithmics (Dagstuhl Seminar 02371). Dagstuhl Seminar Report. 2003-05-07.
