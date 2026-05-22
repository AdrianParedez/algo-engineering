---
doc_id: AE-04-03
title: Streaming Algorithms
status: active
last_verified: 2026-05-22
source_scope: one-pass-and-sublinear-space-algorithms
depends_on:
  - AE-02-04
see_also:
  - AE-04-04
  - AE-05-01
  - AE-06-04
---

# Streaming Algorithms

## Scope
Reference for one-pass and bounded-memory algorithms over large or unending streams, with emphasis on sketch design, mergeability, and window semantics.

## Decision Matrix
| Goal | Typical tool | Main tradeoff |
| --- | --- | --- |
| Frequency estimation | Count-min sketch | Overestimation under collisions. |
| Distinct counting | HyperLogLog | Approximation and bias correction. |
| Membership prefiltering | Bloom filter family | False positives. |
| Sampling / quantiles | Reservoirs and summaries | Approximation vs adaptivity. |

## Theory
- Streaming algorithms matter when the data rate or retention horizon makes exact state impossible or unjustified.
- Mergeability is one of the most valuable structural properties because it converts local sketches into distributed summaries. [SRC-048] [SRC-049]

## Production Reality
- Reset and window policy are often more important than the core sketch formula because systems need time-bounded freshness, not eternal accumulation.
- Bounded memory is only useful if update cost stays small enough for the ingest rate and if error budgets are operationally meaningful.

## Optimization Patterns
- Treat hashing cost as part of the budget; at high rates, better batching or vectorized hashing can matter as much as the sketch itself.
- Make windowing explicit: tumbling, sliding, and decayed summaries support different operational decisions.

## Failure Modes
- Approximate summaries are often deployed without calibration against exact baselines on sampled windows, which makes drift invisible.
- Sketch mergeability does not imply semantic correctness if shards use inconsistent hash or window configuration.

## Related Docs
- [AE-04-04: Online Algorithms](04-online-algorithms.md)
- [AE-05-01: Database Algorithms](../05-systems/01-database-algorithms.md)
- [AE-06-04: Observability](../06-tooling/04-observability.md)

## Sources used
- [SRC-048](https://www.sciencedirect.com/science/article/pii/S0196677403001913) - An improved data stream summary: the count-min sketch and its applications. Journal of Algorithms article. 2005-04-01.
- [SRC-049](https://docslib.org/doc/7666800/hyperloglog-the-analysis-of-a-near-optimal-cardinality-estimation-algorithm) - HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm. AofA paper PDF mirror. 2007-06-01.
