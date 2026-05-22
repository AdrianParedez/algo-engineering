---
doc_id: AE-04-06
title: Randomized Algorithms
status: active
last_verified: 2026-05-22
source_scope: probabilistic-choice-in-algorithm-design
depends_on:
  - AE-01-05
see_also:
  - AE-02-04
  - AE-04-05
  - AE-07-01
---

# Randomized Algorithms

## Scope
Guide to using randomness for load spreading, balancing, sketching, sampling, and robustness against adversarial structure.

## Decision Matrix
| Mode | Why use it | What to watch |
| --- | --- | --- |
| Las Vegas | Always correct, variable runtime | Tail behavior and retries. |
| Monte Carlo | Fixed budget, probabilistic error | Error monitoring and confidence policy. |
| Randomized balancing | Simple high-probability structure | Seed control and adversarial dependence. |

## Theory
- Randomization is often an implementation simplifier as much as a proof technique; skip lists and cuckoo families are classic examples. [SRC-047] [SRC-050]
- The practical question is not “is it randomized?” but what kind of uncertainty it injects: runtime, placement, or output error.

## Production Reality
- Randomization helps defeat structured worst cases, but production systems still need reproducibility hooks for debugging and regression analysis.
- Pseudo-random choice that is insufficiently mixed can collapse the assumed balance guarantees.

## Optimization Patterns
- Expose seed control in experiments and benchmark both median and tail outcomes across seed sweeps when variability matters.
- Use randomized preprocessing or placement when it meaningfully reduces hot spots or rebalancing complexity.

## Failure Modes
- A good average over random seeds can hide catastrophic rare placements that matter in always-on services.
- Randomization can mask deterministic design flaws during testing if the test corpus is too small or too uniform.

## Related Docs
- [AE-02-04: Probabilistic Structures](../02-data-structures/04-probabilistic-structures.md)
- [AE-04-05: Approximation Algorithms](05-approximation-algorithms.md)
- [AE-07-01: Landmark Papers](../07-research/01-landmark-papers.md)

## Sources used
- [SRC-047](https://colab.ws/articles/10.1016%2Fj.jalgor.2003.12.002) - Cuckoo hashing. Journal of Algorithms article metadata. 2004-05-01.
- [SRC-050](https://www.cs.ucdavis.edu/~amenta/w04/skiplists.pdf) - Skip lists: a probabilistic alternative to balanced trees. Communications of the ACM article PDF mirror. 1990-06-01.
