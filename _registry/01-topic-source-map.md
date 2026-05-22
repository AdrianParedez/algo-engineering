---
doc_id: AE-90-01
title: Topic Source Map
status: active
last_verified: 2026-05-22
source_scope: doc-to-source-association-map
depends_on:
  - AE-90-00
see_also:
  - AE-00-00
  - AE-07-05
---

# Topic Source Map

## Scope
Maps major workspace topics and document clusters to the source sets that currently support them.

## Decision Matrix
| Map type | Purpose |
| --- | --- |
| Topic to sources | Find the evidence cluster for a subject quickly. |
| Doc to sources | Audit coverage and traceability. |
| Source reuse | See which sources anchor multiple areas. |

## Usage
- Use this map to check whether a topic is under-sourced before extending the corpus.
- A source reused across many docs is usually a conceptual anchor rather than a one-off fact reference.

## Topic to Source Clusters

| Topic bucket | Source IDs |
| --- | --- |
| 00-meta | SRC-001, SRC-003, SRC-004, SRC-005, SRC-022, SRC-028, SRC-041, SRC-042 |
| 01-foundations | SRC-001, SRC-002, SRC-003, SRC-006, SRC-007, SRC-015, SRC-021, SRC-026, SRC-027 |
| 02-data-structures | SRC-001, SRC-003, SRC-006, SRC-007, SRC-008, SRC-026, SRC-027, SRC-039, SRC-046, SRC-047, SRC-048, SRC-049, SRC-050, SRC-051 |
| 03-performance-engineering | SRC-009, SRC-016, SRC-017, SRC-018, SRC-019, SRC-020, SRC-021, SRC-022, SRC-024, SRC-025, SRC-026, SRC-027, SRC-029, SRC-031, SRC-037, SRC-038, SRC-039, SRC-054, SRC-055 |
| 04-algorithms | SRC-001, SRC-003, SRC-007, SRC-008, SRC-028, SRC-047, SRC-048, SRC-049, SRC-050, SRC-051 |
| 05-systems | SRC-009, SRC-010, SRC-011, SRC-012, SRC-013, SRC-014, SRC-028, SRC-029, SRC-030, SRC-031, SRC-032, SRC-033, SRC-034, SRC-035, SRC-036, SRC-038 |
| 06-tooling | SRC-001, SRC-019, SRC-020, SRC-021, SRC-022, SRC-023, SRC-024, SRC-026, SRC-027, SRC-028, SRC-040, SRC-041, SRC-042, SRC-043, SRC-044, SRC-045 |
| 07-research | SRC-001, SRC-006, SRC-007, SRC-008, SRC-015, SRC-043, SRC-044, SRC-045, SRC-046, SRC-047, SRC-048, SRC-049, SRC-050, SRC-051, SRC-052, SRC-053, SRC-054 |
| 08-execution | SRC-001, SRC-003, SRC-019, SRC-021, SRC-022, SRC-024, SRC-025 |
| README | SRC-001, SRC-003, SRC-004 |
| _registry | SRC-001, SRC-004, SRC-015 |
| _templates | SRC-001, SRC-019, SRC-021, SRC-022, SRC-025 |
| _tracking | SRC-001, SRC-045, SRC-054 |

## Document to Source Map

| Doc ID | Path | Source IDs |
| --- | --- | --- |
| AE-90-00 | _registry/00-source-catalog.md | SRC-001, SRC-015 |
| AE-90-01 | _registry/01-topic-source-map.md | SRC-001 |
| AE-90-02 | _registry/02-citation-style-guide.md | SRC-001, SRC-004 |
| AE-91-01 | _templates/01-document-template.md | SRC-001 |
| AE-91-02 | _templates/02-benchmark-template.md | SRC-019, SRC-021, SRC-022 |
| AE-91-03 | _templates/03-experiment-template.md | SRC-021, SRC-022, SRC-025 |
| AE-92-00 | _tracking/00-update-log.md | SRC-001 |
| AE-92-01 | _tracking/01-open-questions.md | SRC-001, SRC-054 |
| AE-92-02 | _tracking/02-expansion-modules.md | SRC-001, SRC-045 |
| AE-00-00 | 00-meta/00-index.md | SRC-001, SRC-003 |
| AE-00-01 | 00-meta/01-workspace-architecture.md | SRC-001, SRC-041, SRC-042 |
| AE-00-02 | 00-meta/02-document-taxonomy.md | SRC-001, SRC-003, SRC-005 |
| AE-00-03 | 00-meta/03-research-methodology.md | SRC-001, SRC-004, SRC-005, SRC-028 |
| AE-00-04 | 00-meta/04-ingestion-pipeline.md | SRC-001, SRC-004, SRC-022 |
| AE-01-01 | 01-foundations/01-field-overview.md | SRC-001, SRC-002, SRC-003 |
| AE-01-02 | 01-foundations/02-complexity-analysis.md | SRC-001, SRC-006, SRC-007, SRC-021 |
| AE-01-03 | 01-foundations/03-computational-models.md | SRC-001, SRC-006, SRC-007, SRC-015 |
| AE-01-04 | 01-foundations/04-memory-hierarchy.md | SRC-006, SRC-007, SRC-026, SRC-027 |
| AE-01-05 | 01-foundations/05-algorithmic-paradigms.md | SRC-001, SRC-002, SRC-003 |
| AE-02-01 | 02-data-structures/01-linear-structures.md | SRC-001, SRC-026, SRC-027 |
| AE-02-02 | 02-data-structures/02-tree-structures.md | SRC-001, SRC-006, SRC-039 |
| AE-02-03 | 02-data-structures/03-graph-structures.md | SRC-003, SRC-026, SRC-051 |
| AE-02-04 | 02-data-structures/04-probabilistic-structures.md | SRC-046, SRC-047, SRC-048, SRC-049, SRC-050 |
| AE-02-05 | 02-data-structures/05-cache-efficient-layouts.md | SRC-006, SRC-007, SRC-008 |
| AE-03-01 | 03-performance-engineering/01-cpu-optimization.md | SRC-017, SRC-018, SRC-027 |
| AE-03-02 | 03-performance-engineering/02-simd-vectorization.md | SRC-009, SRC-016, SRC-017, SRC-055 |
| AE-03-03 | 03-performance-engineering/03-memory-optimization.md | SRC-026, SRC-038, SRC-039 |
| AE-03-04 | 03-performance-engineering/04-concurrency.md | SRC-031, SRC-037, SRC-054 |
| AE-03-05 | 03-performance-engineering/05-lock-free-systems.md | SRC-054, SRC-029 |
| AE-03-06 | 03-performance-engineering/06-profiling-and-benchmarking.md | SRC-019, SRC-020, SRC-021, SRC-022, SRC-024, SRC-025, SRC-027 |
| AE-04-01 | 04-algorithms/01-sorting-and-searching.md | SRC-003, SRC-007, SRC-008 |
| AE-04-02 | 04-algorithms/02-graph-algorithms.md | SRC-003, SRC-051 |
| AE-04-03 | 04-algorithms/03-streaming-algorithms.md | SRC-048, SRC-049 |
| AE-04-04 | 04-algorithms/04-online-algorithms.md | SRC-001, SRC-028 |
| AE-04-05 | 04-algorithms/05-approximation-algorithms.md | SRC-001, SRC-003 |
| AE-04-06 | 04-algorithms/06-randomized-algorithms.md | SRC-047, SRC-050 |
| AE-05-01 | 05-systems/01-database-algorithms.md | SRC-034, SRC-035 |
| AE-05-02 | 05-systems/02-distributed-algorithms.md | SRC-031, SRC-032, SRC-033 |
| AE-05-03 | 05-systems/03-compiler-optimization.md | SRC-009, SRC-010, SRC-011, SRC-012, SRC-013, SRC-014 |
| AE-05-04 | 05-systems/04-ai-and-ml-infrastructure.md | SRC-033, SRC-036, SRC-012 |
| AE-05-05 | 05-systems/05-low-latency-systems.md | SRC-028, SRC-029, SRC-030, SRC-038 |
| AE-06-01 | 06-tooling/01-profiler-stack.md | SRC-023, SRC-024, SRC-026, SRC-027, SRC-045 |
| AE-06-02 | 06-tooling/02-benchmark-frameworks.md | SRC-019, SRC-020, SRC-021, SRC-022 |
| AE-06-03 | 06-tooling/03-build-systems.md | SRC-040, SRC-041, SRC-042 |
| AE-06-04 | 06-tooling/04-observability.md | SRC-043, SRC-044, SRC-045, SRC-028 |
| AE-06-05 | 06-tooling/05-performance-workflows.md | SRC-001, SRC-022, SRC-024, SRC-028 |
| AE-07-01 | 07-research/01-landmark-papers.md | SRC-001, SRC-006, SRC-007, SRC-046, SRC-047, SRC-048, SRC-049, SRC-050 |
| AE-07-02 | 07-research/02-modern-breakthroughs.md | SRC-001, SRC-008, SRC-051, SRC-052, SRC-053, SRC-054 |
| AE-07-03 | 07-research/03-open-problems.md | SRC-001, SRC-008, SRC-051, SRC-054 |
| AE-07-04 | 07-research/04-frontier-directions.md | SRC-001, SRC-043, SRC-044, SRC-045, SRC-054 |
| AE-07-05 | 07-research/05-reference-bibliography.md | SRC-001, SRC-015 |
| AE-08-01 | 08-execution/01-learning-roadmap.md | SRC-001, SRC-003, SRC-024 |
| AE-08-02 | 08-execution/02-project-roadmap.md | SRC-001, SRC-003 |
| AE-08-03 | 08-execution/03-implementation-backlog.md | SRC-001, SRC-022, SRC-024 |
| AE-08-04 | 08-execution/04-verification-methodology.md | SRC-019, SRC-021, SRC-022, SRC-025 |
| AE-08-05 | 08-execution/05-skill-progression-matrix.md | SRC-001, SRC-024 |
| AE-00-99 | README.md | SRC-001, SRC-003, SRC-004 |

## Related Docs
- [AE-00-00: Workspace Index](../00-meta/00-index.md)
- [AE-07-05: Reference Bibliography](../07-research/05-reference-bibliography.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
