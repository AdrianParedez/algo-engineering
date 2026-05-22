---
doc_id: AE-90-02
title: Citation Style Guide
status: active
last_verified: 2026-05-22
source_scope: citation-and-synthesis-conventions
depends_on:
  - AE-90-00
see_also:
  - AE-00-03
  - AE-07-05
---

# Citation Style Guide

## Scope
Defines inline source-ID usage, how to distinguish verified claims from synthesis, and how to cite evolving tooling or runtime behavior.

## Decision Matrix
| Case | Style rule |
| --- | --- |
| Direct claim | End the sentence or bullet with one or more SRC IDs. |
| Cross-source synthesis | Cite the cluster and make inference explicit in wording. |
| Time-sensitive claim | Include exact date in prose or rely on dated source metadata. |
| Open question | Do not fake certainty; move the claim to tracking if needed. |

## Rules
- Inline citations use compact source IDs such as [SRC-019]; the bibliography and registry hold the full URL and metadata.
- Use wording such as “suggests”, “indicates”, or “likely” when the text is synthesis rather than a direct source claim. [SRC-001]
- If a claim is operationally important and date-sensitive, state the date explicitly or make sure the linked source metadata contains it.

## Related Docs
- [AE-00-03: Research Methodology](../00-meta/03-research-methodology.md)
- [AE-07-05: Reference Bibliography](../07-research/05-reference-bibliography.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-004](https://www.nzdr.ru/data/media/biblio/kolxoz/Cs/CsLn/Algorithm%20Engineering%2C%203%20conf.%2C%20WAE%20%2799%28LNCS1668%2C%20Springer%2C%201999%29%28ISBN%203540664270%29%28368s%29.pdf) - Selecting Problems for Algorithm Evaluation. LNCS workshop paper. 1999-07-19.
