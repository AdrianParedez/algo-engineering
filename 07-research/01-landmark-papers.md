---
doc_id: AE-07-01
title: Landmark Papers
status: active
last_verified: 2026-05-22
source_scope: foundational-bibliographic-curation
depends_on:
  - AE-07-05
see_also:
  - AE-01-01
  - AE-02-04
  - AE-07-02
---

# Landmark Papers

## Scope
Curated shortlist of papers that established the conceptual vocabulary of modern algorithm engineering and performance-aware data structure design.

## Decision Matrix
| Paper | Why it matters | Still relevant because |
| --- | --- | --- |
| Aggarwal-Vitter 1988 | Made data movement first-class | I/O still dominates at scale. |
| Frigo et al. 1999 | Introduced cache-oblivious design | Portability across hierarchies remains valuable. |
| Bloom 1970 | Canonical approximate membership | False-positive engineering is ubiquitous. |
| Cormode-Muthukrishnan 2005 | Practical streaming summary | Mergeable approximate counts remain core. |
| Flajolet et al. 2007 | Cardinality at tiny memory cost | Distinct counting is everywhere. |

## Selection Heuristic
- A landmark paper earns inclusion if it changed either the cost model, the methodology, or a data-structure family that still shapes production systems. [SRC-001]
- The goal is not exhaustiveness; it is to identify works that still repay close rereading when designing or reviewing systems.

## Recommended Reading Order
- Methodology first: SRC-001, SRC-002, SRC-004.
- Movement and layout second: SRC-006, SRC-007.
- Approximate structures third: SRC-046 through SRC-050.

## Related Docs
- [AE-01-01: Field Overview](../01-foundations/01-field-overview.md)
- [AE-02-04: Probabilistic Structures](../02-data-structures/04-probabilistic-structures.md)
- [AE-07-02: Modern Breakthroughs](02-modern-breakthroughs.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-006](https://doi.org/10.1145/48529.48535) - The Input/Output Complexity of Sorting and Related Problems. Communications of the ACM article. 1988-09-01.
- [SRC-007](https://www.cs.cornell.edu/courses/cs612/2006sp/papers/frigo99.pdf) - Cache-Oblivious Algorithms. FOCS paper PDF. 1999-05-01.
- [SRC-046](https://www.scirp.org/reference/referencespapers?referenceid=1303589) - Space/Time Trade-Offs in Hash Coding with Allowable Errors. Communications of the ACM article metadata. 1970-07-01.
- [SRC-047](https://colab.ws/articles/10.1016%2Fj.jalgor.2003.12.002) - Cuckoo hashing. Journal of Algorithms article metadata. 2004-05-01.
- [SRC-048](https://www.sciencedirect.com/science/article/pii/S0196677403001913) - An improved data stream summary: the count-min sketch and its applications. Journal of Algorithms article. 2005-04-01.
- [SRC-049](https://docslib.org/doc/7666800/hyperloglog-the-analysis-of-a-near-optimal-cardinality-estimation-algorithm) - HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm. AofA paper PDF mirror. 2007-06-01.
- [SRC-050](https://www.cs.ucdavis.edu/~amenta/w04/skiplists.pdf) - Skip lists: a probabilistic alternative to balanced trees. Communications of the ACM article PDF mirror. 1990-06-01.
