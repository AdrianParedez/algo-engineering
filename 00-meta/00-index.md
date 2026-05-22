---
doc_id: AE-00-00
title: Workspace Index
status: active
last_verified: 2026-05-22
source_scope: generated-navigation-index
depends_on:
  - AE-00-01
see_also:
  - AE-00-99
  - AE-00-02
  - AE-90-01
---

# Workspace Index

## Scope
Canonical navigation map for the corpus. Use it to jump by topic, evidence type, or work phase rather than by directory name alone.

## Decision Matrix
| Navigation mode | Primary docs | Use when |
| --- | --- | --- |
| Architecture first | AE-00-02 to AE-00-04 | You are extending or refactoring the workspace itself. |
| Algorithmic topic first | AE-01-* to AE-04-* | You are comparing methods or choosing data structures and algorithms. |
| Systems/tooling first | AE-05-* to AE-06-* | You are tuning an implementation or building a measurement stack. |
| Execution first | AE-08-* | You need an action plan, backlog, or verification protocol. |

## Index Semantics
- The index groups documents by decision surface: conceptual grounding, implementation substrate, measurement stack, frontier research, and execution planning. [SRC-001] [SRC-003]
- Support documents live outside the numbered corpus but remain part of the canonical workspace and use reserved ID bands for stability. [SRC-001]

## Navigation Tables

### Landing Page
| ID | Title | Path |
| --- | --- | --- |
| AE-00-99 | Algorithm Engineering Workspace | `README.md` |

### Meta
| ID | Title | Path |
| --- | --- | --- |
| AE-00-00 | Workspace Index | `00-meta/00-index.md` |
| AE-00-01 | Workspace Architecture | `00-meta/01-workspace-architecture.md` |
| AE-00-02 | Document Taxonomy | `00-meta/02-document-taxonomy.md` |
| AE-00-03 | Research Methodology | `00-meta/03-research-methodology.md` |
| AE-00-04 | Ingestion Pipeline | `00-meta/04-ingestion-pipeline.md` |

### Foundations
| ID | Title | Path |
| --- | --- | --- |
| AE-01-01 | Field Overview | `01-foundations/01-field-overview.md` |
| AE-01-02 | Complexity Analysis | `01-foundations/02-complexity-analysis.md` |
| AE-01-03 | Computational Models | `01-foundations/03-computational-models.md` |
| AE-01-04 | Memory Hierarchy | `01-foundations/04-memory-hierarchy.md` |
| AE-01-05 | Algorithmic Paradigms | `01-foundations/05-algorithmic-paradigms.md` |

### Data Structures
| ID | Title | Path |
| --- | --- | --- |
| AE-02-01 | Linear Structures | `02-data-structures/01-linear-structures.md` |
| AE-02-02 | Tree Structures | `02-data-structures/02-tree-structures.md` |
| AE-02-03 | Graph Structures | `02-data-structures/03-graph-structures.md` |
| AE-02-04 | Probabilistic Structures | `02-data-structures/04-probabilistic-structures.md` |
| AE-02-05 | Cache-Efficient Layouts | `02-data-structures/05-cache-efficient-layouts.md` |

### Performance Engineering
| ID | Title | Path |
| --- | --- | --- |
| AE-03-01 | CPU Optimization | `03-performance-engineering/01-cpu-optimization.md` |
| AE-03-02 | SIMD Vectorization | `03-performance-engineering/02-simd-vectorization.md` |
| AE-03-03 | Memory Optimization | `03-performance-engineering/03-memory-optimization.md` |
| AE-03-04 | Concurrency | `03-performance-engineering/04-concurrency.md` |
| AE-03-05 | Lock-Free Systems | `03-performance-engineering/05-lock-free-systems.md` |
| AE-03-06 | Profiling and Benchmarking | `03-performance-engineering/06-profiling-and-benchmarking.md` |

### Algorithms
| ID | Title | Path |
| --- | --- | --- |
| AE-04-01 | Sorting and Searching | `04-algorithms/01-sorting-and-searching.md` |
| AE-04-02 | Graph Algorithms | `04-algorithms/02-graph-algorithms.md` |
| AE-04-03 | Streaming Algorithms | `04-algorithms/03-streaming-algorithms.md` |
| AE-04-04 | Online Algorithms | `04-algorithms/04-online-algorithms.md` |
| AE-04-05 | Approximation Algorithms | `04-algorithms/05-approximation-algorithms.md` |
| AE-04-06 | Randomized Algorithms | `04-algorithms/06-randomized-algorithms.md` |

### Systems
| ID | Title | Path |
| --- | --- | --- |
| AE-05-01 | Database Algorithms | `05-systems/01-database-algorithms.md` |
| AE-05-02 | Distributed Algorithms | `05-systems/02-distributed-algorithms.md` |
| AE-05-03 | Compiler Optimization | `05-systems/03-compiler-optimization.md` |
| AE-05-04 | AI and ML Infrastructure | `05-systems/04-ai-and-ml-infrastructure.md` |
| AE-05-05 | Low-Latency Systems | `05-systems/05-low-latency-systems.md` |

### Tooling
| ID | Title | Path |
| --- | --- | --- |
| AE-06-01 | Profiler Stack | `06-tooling/01-profiler-stack.md` |
| AE-06-02 | Benchmark Frameworks | `06-tooling/02-benchmark-frameworks.md` |
| AE-06-03 | Build Systems | `06-tooling/03-build-systems.md` |
| AE-06-04 | Observability | `06-tooling/04-observability.md` |
| AE-06-05 | Performance Workflows | `06-tooling/05-performance-workflows.md` |

### Research
| ID | Title | Path |
| --- | --- | --- |
| AE-07-01 | Landmark Papers | `07-research/01-landmark-papers.md` |
| AE-07-02 | Modern Breakthroughs | `07-research/02-modern-breakthroughs.md` |
| AE-07-03 | Open Problems | `07-research/03-open-problems.md` |
| AE-07-04 | Frontier Directions | `07-research/04-frontier-directions.md` |
| AE-07-05 | Reference Bibliography | `07-research/05-reference-bibliography.md` |

### Execution
| ID | Title | Path |
| --- | --- | --- |
| AE-08-01 | Learning Roadmap | `08-execution/01-learning-roadmap.md` |
| AE-08-02 | Project Roadmap | `08-execution/02-project-roadmap.md` |
| AE-08-03 | Implementation Backlog | `08-execution/03-implementation-backlog.md` |
| AE-08-04 | Verification Methodology | `08-execution/04-verification-methodology.md` |
| AE-08-05 | Skill Progression Matrix | `08-execution/05-skill-progression-matrix.md` |

### Registry
| ID | Title | Path |
| --- | --- | --- |
| AE-90-00 | Source Catalog | `_registry/00-source-catalog.md` |
| AE-90-01 | Topic Source Map | `_registry/01-topic-source-map.md` |
| AE-90-02 | Citation Style Guide | `_registry/02-citation-style-guide.md` |

### Templates
| ID | Title | Path |
| --- | --- | --- |
| AE-91-01 | Document Template | `_templates/01-document-template.md` |
| AE-91-02 | Benchmark Template | `_templates/02-benchmark-template.md` |
| AE-91-03 | Experiment Template | `_templates/03-experiment-template.md` |

### Tracking
| ID | Title | Path |
| --- | --- | --- |
| AE-92-00 | Update Log | `_tracking/00-update-log.md` |
| AE-92-01 | Open Questions | `_tracking/01-open-questions.md` |
| AE-92-02 | Expansion Modules | `_tracking/02-expansion-modules.md` |

## Related Docs
- [AE-00-99: Algorithm Engineering Workspace](../README.md)
- [AE-00-02: Document Taxonomy](02-document-taxonomy.md)
- [AE-90-01: Topic Source Map](../_registry/01-topic-source-map.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-003](https://drops.dagstuhl.de/entities/document/10.4230/DagRep.3.9.169) - Algorithm Engineering (Dagstuhl Seminar 13391). Dagstuhl Reports article. 2014-01-17.
