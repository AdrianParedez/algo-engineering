---
doc_id: AE-00-04
title: Ingestion Pipeline
status: active
last_verified: 2026-05-22
source_scope: source-ingestion-and-curation-process
depends_on:
  - AE-00-03
  - AE-90-00
see_also:
  - AE-90-01
  - AE-92-00
  - AE-91-03
---

# Ingestion Pipeline

## Scope
Operational process for extending the workspace with new sources, new findings, and new execution artifacts while preserving traceability and document stability.

## Decision Matrix
| Stage | Output | Failure condition |
| --- | --- | --- |
| Source registration | Registry entry with date, type, and topic tags | Untraceable source or unstable URL. |
| Topic mapping | Doc-to-source association | Source exists but no clear decision surface. |
| Synthesis | Updated reference text with inline citations | Claim cannot be separated into fact vs inference. |
| Validation | Passing structure/link/source checks | Broken IDs, links, or missing citations. |

## Pipeline Rules
- Every new source enters AE-90-00 before it influences stable reference material. This prevents orphan claims and makes re-verification possible. [SRC-001]
- Experimental data should not be summarized without environment metadata, workload definition, and a statement of what variable changed. [SRC-004] [SRC-022]
- When a new source only suggests a promising direction but does not yet justify a durable recommendation, capture it in tracking instead of promoting it immediately. [SRC-001]

## Related Docs
- [AE-90-01: Topic Source Map](../_registry/01-topic-source-map.md)
- [AE-92-00: Update Log](../_tracking/00-update-log.md)
- [AE-91-03: Experiment Template](../_templates/03-experiment-template.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-004](https://www.nzdr.ru/data/media/biblio/kolxoz/Cs/CsLn/Algorithm%20Engineering%2C%203%20conf.%2C%20WAE%20%2799%28LNCS1668%2C%20Springer%2C%201999%29%28ISBN%203540664270%29%28368s%29.pdf) - Selecting Problems for Algorithm Evaluation. LNCS workshop paper. 1999-07-19.
- [SRC-022](https://pyperf.readthedocs.io/en/stable/system.html) - Tune the system for benchmarks. pyperf documentation. 2026-05-22 verified.
