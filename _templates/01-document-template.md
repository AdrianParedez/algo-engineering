---
doc_id: AE-91-01
title: Document Template
status: active
last_verified: 2026-05-22
source_scope: authoring-template
depends_on:
  - AE-00-01
see_also:
  - AE-91-02
  - AE-91-03
---

# Document Template

## Scope
Reusable template for future reference documents that need full front matter, scope, matrix, and source discipline.

## Decision Matrix
| Section | Intent |
| --- | --- |
| Front matter | Stable ID and churn metadata. |
| Scope | What the document answers. |
| Decision Matrix | Fast comparison surface. |
| Core sections | Theory, reality, patterns, failures, notes. |
| Sources used | Local source closure. |

## Template Notes
- Prefer dense bullets over long exposition when the decision surface is already known.
- Do not omit failure modes; they are part of the engineering contract.

## Skeleton

```md
---
doc_id: AE-SS-FF
title: Title
status: active
last_verified: YYYY-MM-DD
source_scope: short-scope-tag
depends_on:
  - AE-XX-YY
see_also:
  - AE-XX-ZZ
---

# Title

## Scope
One-paragraph statement of what the document answers.

## Decision Matrix
| Column | Column |
| --- | --- |
| ... | ... |

## Theory
- ... [SRC-001]

## Production Reality
- ... [SRC-002]

## Optimization Patterns
- ... [SRC-003]

## Failure Modes
- ... [SRC-004]

## Related Docs
- [AE-XX-YY: Some Doc](../path.md)

## Sources used
- [SRC-001](https://example.com) - Title.
```

## Related Docs
- [AE-91-02: Benchmark Template](02-benchmark-template.md)
- [AE-91-03: Experiment Template](03-experiment-template.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
