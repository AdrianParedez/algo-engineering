---
doc_id: AE-00-01
title: Workspace Architecture
status: active
last_verified: 2026-05-22
source_scope: workspace-conventions-and-structure
depends_on:
  - none
see_also:
  - AE-00-02
  - AE-00-04
  - AE-91-01
---

# Workspace Architecture

## Scope
Defines directory roles, document-ID semantics, cross-reference rules, and the separation between stable reference material and evolving tracking artifacts.

## Decision Matrix
| Namespace | ID band | Role |
| --- | --- | --- |
| README | AE-00-00 | Top-level landing page. |
| Numbered corpus | AE-SS-FF | Stable research and execution documents. |
| _registry | AE-90-* | Canonical source metadata and citation rules. |
| _templates | AE-91-* | Reusable authoring and experiment templates. |
| _tracking | AE-92-* | Mutable operational follow-up material. |

## Structural Rules
- Top-level numbered directories are immutable conceptual buckets; support directories absorb process churn so the numbered corpus can remain durable. [SRC-001]
- Document IDs encode section and file order for the numbered corpus; support layers use reserved bands so links remain stable even if support files change. [SRC-041]
- All cross-references should use both the ID and the relative link target so readers can navigate even if rendered outside an IDE or local wiki. [SRC-041] [SRC-042]

## Mutation Policy
- Reference documents should change only when evidence or operational standards change; open questions and future ideas belong in tracking documents instead of being mixed into stable pages. [SRC-001]
- New material enters through the ingestion pipeline: source registration, topic mapping, synthesis, cross-linking, and validation. [SRC-004]
- Production claims without a reproducible measurement path belong in tracking until they can be upgraded into stable reference material. [SRC-022] [SRC-028]

## Related Docs
- [AE-00-02: Document Taxonomy](02-document-taxonomy.md)
- [AE-00-04: Ingestion Pipeline](04-ingestion-pipeline.md)
- [AE-91-01: Document Template](../_templates/01-document-template.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-041](https://cmake.org/cmake/help/latest/manual/cmake-presets.7.html) - cmake-presets(7). CMake documentation. 2026-05-22 verified.
- [SRC-042](https://mesonbuild.com/Builtin-options.html) - Meson built-in options. Meson documentation. 2026-05-08 verified.
