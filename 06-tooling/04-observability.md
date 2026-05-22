---
doc_id: AE-06-04
title: Observability
status: active
last_verified: 2026-05-22
source_scope: signals-for-performance-and-reliability
depends_on:
  - AE-03-06
see_also:
  - AE-06-01
  - AE-05-05
  - AE-92-01
---

# Observability

## Scope
How to combine metrics, logs, traces, profiles, and kernel observability so performance work remains diagnosable in production.

## Decision Matrix
| Signal | Answers | Does not answer well |
| --- | --- | --- |
| Metrics | What is changing? | Why code is slow. |
| Logs | What event happened? | Continuous resource attribution. |
| Traces | Where did request time go? | Which function burned the CPU. |
| Profiles | Why is code consuming resources? | Business-level intent without labels. |
| Kernel / eBPF events | What the host and runtime are doing? | Application semantics unless correlated. |

## Signal Model
- OpenTelemetry formalizes telemetry as signals and now explicitly includes profiles as an emerging signal alongside traces, metrics, and logs. [SRC-043]
- Continuous profiling fills the gap between service-level symptoms and code-level root cause. [SRC-044]

## Operational Rules
- Correlate signals rather than replacing one with another: use metrics to detect, traces to localize, profiles to explain, and kernel data to verify host-side causes. [SRC-028] [SRC-043] [SRC-045]
- Use eBPF when you need low-overhead visibility into kernel or runtime behavior without invasive instrumentation. [SRC-045]

## Failure Modes
- Too many undifferentiated metrics create alert volume without diagnosis capability.
- Tracing without profile or host context often points to the slow request but not the slow code path.

## Related Docs
- [AE-06-01: Profiler Stack](01-profiler-stack.md)
- [AE-05-05: Low-Latency Systems](../05-systems/05-low-latency-systems.md)
- [AE-92-01: Open Questions](../_tracking/01-open-questions.md)

## Sources used
- [SRC-043](https://opentelemetry.io/docs/concepts/signals/) - Signals. OpenTelemetry concepts. 2026-03-10 modified.
- [SRC-044](https://grafana.com/docs/pyroscope/latest/) - Grafana Pyroscope overview. Grafana / Pyroscope documentation. 2026-05-16 verified.
- [SRC-045](https://ebpf.io/what-is-ebpf/) - What is eBPF?. eBPF documentation. 2026-05-22 verified.
- [SRC-028](https://www.brendangregg.com/usemethod.html) - The USE Method. Brendan Gregg methodology page. 2026-05-22 verified.
