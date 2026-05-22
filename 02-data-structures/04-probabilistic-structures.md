---
doc_id: AE-02-04
title: Probabilistic Structures
status: active
last_verified: 2026-05-22
source_scope: approximate-and-randomized-data-structures
depends_on:
  - AE-01-05
see_also:
  - AE-04-03
  - AE-04-06
  - AE-05-01
---

# Probabilistic Structures

## Scope
Operational reference for Bloom filters, cuckoo hashing, count-min sketch, HyperLogLog, and skip lists, with emphasis on error budgets, mergeability, and deployment pitfalls.

## Decision Matrix
| Structure | Promise | Cost / risk |
| --- | --- | --- |
| Bloom filter | Fast approximate membership | False positives and rebuild tuning. |
| Cuckoo hashing | O(1) lookup with high occupancy | Insertion cycles and resize policy. |
| Count-min sketch | Sublinear frequency summary | Collision-induced overestimation. |
| HyperLogLog | Cardinality estimation with tiny state | Bias correction and small-range behavior. |
| Skip list | Simple randomized ordered set | Variance and pointer overhead. |

## Theory
- Probabilistic structures exchange exactness, determinism, or worst-case guarantees for lower memory, simpler updates, or better average placement. [SRC-046] [SRC-047] [SRC-048] [SRC-049] [SRC-050]
- The critical engineering parameter is usually the error budget or failure probability, not the nominal big-O bound.

## Production Reality
- Hash quality, merge semantics, and reset/rebuild policy matter as much as the textbook formulae once the structure is embedded in a service or pipeline.
- Approximate structures are most useful when the surrounding system can absorb false positives, bounded overestimation, or probabilistic balancing.

## Optimization Patterns
- Make the error budget explicit in API and monitoring terms: false-positive rate, relative cardinality error, or heavy-hitter miss rate.
- Exploit mergeability where available; sketches and estimators earn their keep when they compose across shards or time windows. [SRC-048] [SRC-049]

## Failure Modes
- Non-independent or poorly distributed hashes can invalidate practical error assumptions long before asymptotic bounds become relevant.
- Approximate answers often leak into exact decision paths unless there is a deliberate verification or fallback stage.

## Benchmark/Profiling Notes
- Benchmark accuracy and resource cost together; a faster sketch with an unstable error distribution is not an upgrade.
- Drive tests with adversarial key distributions and realistic merge/update patterns rather than only IID random inputs.

## Related Docs
- [AE-04-03: Streaming Algorithms](../04-algorithms/03-streaming-algorithms.md)
- [AE-04-06: Randomized Algorithms](../04-algorithms/06-randomized-algorithms.md)
- [AE-05-01: Database Algorithms](../05-systems/01-database-algorithms.md)

## Sources used
- [SRC-046](https://www.scirp.org/reference/referencespapers?referenceid=1303589) - Space/Time Trade-Offs in Hash Coding with Allowable Errors. Communications of the ACM article metadata. 1970-07-01.
- [SRC-047](https://colab.ws/articles/10.1016%2Fj.jalgor.2003.12.002) - Cuckoo hashing. Journal of Algorithms article metadata. 2004-05-01.
- [SRC-048](https://www.sciencedirect.com/science/article/pii/S0196677403001913) - An improved data stream summary: the count-min sketch and its applications. Journal of Algorithms article. 2005-04-01.
- [SRC-049](https://docslib.org/doc/7666800/hyperloglog-the-analysis-of-a-near-optimal-cardinality-estimation-algorithm) - HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm. AofA paper PDF mirror. 2007-06-01.
- [SRC-050](https://www.cs.ucdavis.edu/~amenta/w04/skiplists.pdf) - Skip lists: a probabilistic alternative to balanced trees. Communications of the ACM article PDF mirror. 1990-06-01.
