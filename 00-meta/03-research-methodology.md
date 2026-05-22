---
doc_id: AE-00-03
title: Research Methodology
status: active
last_verified: 2026-05-22
source_scope: research-method-and-evidence-ranking
depends_on:
  - AE-00-02
see_also:
  - AE-00-04
  - AE-90-02
  - AE-08-04
---

# Research Methodology

## Scope
Explains how this workspace turns heterogeneous sources into implementation-grade conclusions. It is the method contract for future updates.

## Decision Matrix
| Rank | Source class | Default use |
| --- | --- | --- |
| 1 | specs, official docs, peer-reviewed papers | Definitions, semantics, and hard claims. |
| 2 | production-grade repository docs and engineering internals | Operational behavior and deployment tradeoffs. |
| 3 | secondary explainers | Gap-filling only when the primary layer is insufficient. |

## Method
- Start with the problem frame, not the tool. Ask which resource is likely binding, which model is appropriate, and which workloads matter before selecting an implementation strategy. [SRC-001] [SRC-028]
- Search multiple phrasings and compare independent sources before concluding; conflicting claims should be resolved by preferring stronger primary evidence or by explicitly preserving the disagreement. [SRC-001] [SRC-004]
- Treat theory and production evidence as complementary but non-substitutable: asymptotic optimality does not certify cache behavior, contention behavior, or measurement stability. [SRC-003] [SRC-005]

## Synthesis Rules
- Inline citations mark verified claims; unlabeled design guidance should be read as synthesis informed by the cited source cluster and practical engineering constraints. [SRC-001]
- If a source is time-sensitive, the exact verification or publication date should appear either in the prose or in source metadata. [SRC-022] [SRC-041]
- If a claim cannot yet be made rigorous enough for stable reference material, move it to AE-92-01 or AE-92-02. [SRC-001]

## Related Docs
- [AE-00-04: Ingestion Pipeline](04-ingestion-pipeline.md)
- [AE-90-02: Citation Style Guide](../_registry/02-citation-style-guide.md)
- [AE-08-04: Verification Methodology](../08-execution/04-verification-methodology.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-004](https://www.nzdr.ru/data/media/biblio/kolxoz/Cs/CsLn/Algorithm%20Engineering%2C%203%20conf.%2C%20WAE%20%2799%28LNCS1668%2C%20Springer%2C%201999%29%28ISBN%203540664270%29%28368s%29.pdf) - Selecting Problems for Algorithm Evaluation. LNCS workshop paper. 1999-07-19.
- [SRC-005](https://drops.dagstuhl.de/entities/document/10.4230/DagSemRep.353) - Experimental Algorithmics (Dagstuhl Seminar 02371). Dagstuhl Seminar Report. 2003-05-07.
- [SRC-028](https://www.brendangregg.com/usemethod.html) - The USE Method. Brendan Gregg methodology page. 2026-05-22 verified.
