---
doc_id: AE-07-04
title: Frontier Directions
status: active
last_verified: 2026-05-22
source_scope: verified-trends-plus-synthesis
depends_on:
  - AE-07-03
see_also:
  - AE-05-04
  - AE-06-04
  - AE-92-01
---

# Frontier Directions

## Scope
Short list of directions likely to matter over the next few years, explicitly separating verified trends from synthesis and hypotheses.

## Decision Matrix
| Direction | Status | Signal |
| --- | --- | --- |
| Continuous profiling as first-class signal | Verified trend | OTel profiles and Pyroscope maturation. |
| NUMA-aware concurrent structures | Verified trend | Recent structure-specific research and production pressure. |
| eBPF-assisted performance diagnostics | Verified trend | Broader production adoption and low-overhead introspection. |
| Learned or adaptive layout control | Synthesis | Strong plausibility, weaker standardization. |
| Disaggregated-memory-aware algorithms | Synthesis | Pressure is rising but practice is still fragmented. |

## Verified Trends
- Profiles are moving toward first-class observability status rather than being treated as an emergency-only debugging artifact. [SRC-043] [SRC-044]
- Kernel-level observability via eBPF is now a mainstream mechanism for low-overhead diagnostics. [SRC-045]
- NUMA-aware concurrent data structures are receiving focused algorithmic attention because topology increasingly shapes performance outcomes. [SRC-054]

## Synthesis and Hypotheses
- Layout adaptation driven by profile feedback is likely to grow, especially where stable hot paths and long-lived services make profile-guided restructuring economical.
- As remote and tiered memory become more operationally common, algorithm engineering will need better models for memory placement and migration under fluctuating availability.

## Caution
- These frontier bets should guide experiments and reading plans, not be treated as settled universal recommendations.

## Related Docs
- [AE-05-04: AI and ML Infrastructure](../05-systems/04-ai-and-ml-infrastructure.md)
- [AE-06-04: Observability](../06-tooling/04-observability.md)
- [AE-92-01: Open Questions](../_tracking/01-open-questions.md)

## Sources used
- [SRC-001](https://www.weizenbaum-library.de/bitstreams/3a58cc73-4a5f-4151-bd6e-b5e06769e17c/download) - Methodology of Algorithm Engineering. ACM Computing Surveys article. 2025-10-25.
- [SRC-043](https://opentelemetry.io/docs/concepts/signals/) - Signals. OpenTelemetry concepts. 2026-03-10 modified.
- [SRC-044](https://grafana.com/docs/pyroscope/latest/) - Grafana Pyroscope overview. Grafana / Pyroscope documentation. 2026-05-16 verified.
- [SRC-045](https://ebpf.io/what-is-ebpf/) - What is eBPF?. eBPF documentation. 2026-05-22 verified.
- [SRC-054](https://arxiv.org/abs/2406.06900) - SmartPQ: An Adaptive Concurrent Priority Queue for NUMA Architectures. arXiv paper. 2024-06-10.
