# Baseline Methodology

## Assumptions About the Memory Hierarchy

- Modern performance is constrained more by data movement than nominal arithmetic cost once the working set escapes L1 and L2.
- Pointer-heavy structures amplify dependency chains, TLB pressure, and irregular prefetch behavior relative to flat arrays.
- Pointer-layout comparisons must separate allocator placement, node packing, and dependency structure. A single “linked vs flat” benchmark can easily hide the real cause of a delta.
- Matrix kernels are sensitive to loop order because row-major layouts reward contiguous inner loops and punish repeated column walks.
- SIMD gains depend on regular control flow, contiguous loads, and target-feature availability; branch-heavy or alias-heavy code can suppress vectorization.

## Measurement Rules

- Use Criterion defaults for warm-up, iteration calibration, and statistical reporting unless a later experiment has a measured reason to change them.
- Benchmark deterministic datasets so regressions come from implementation changes rather than input drift.
- Keep setup outside the hot path where possible, but do not hide algorithmically required work.
- Use `std::hint::black_box` only at observation boundaries to prevent elision without introducing undefined behavior or unrealistic fences.
- Report variance and confidence intervals alongside central tendency. Mean-only claims are insufficient for this lab.

## Expected Failure Modes

- Hardware prefetchers may partially rescue pointer-chasing on some allocators or traversal patterns, reducing the observed delta.
- Split SoA pointer-chasing can look “flat” while still paying two dependent loads per hop. Do not treat it as a pure contiguous-streaming baseline.
- False sharing can contaminate future concurrent extensions; it is not a direct factor in this first single-threaded lab but remains a known hazard for later work.
- Branch misprediction can dominate the scalar byte-scan path when the threshold splits the distribution near 50/50.
- Matrix sizes that fit entirely in a high cache level may understate the benefit of blocking.
- OS scheduler noise, background services, and frequency scaling can distort short-running microbenchmarks.

## Interpretation Guidance

- Treat `pointer_layout` as a locality and dependency benchmark, not a universal claim that linked structures are always wrong. Read the subcases in this order:
  - `boxed_pointer_chain` vs `arena_pointer_chain`: allocator/page scatter effect
  - `arena_pointer_chain` vs `packed_index_chase`: raw pointer chasing vs integer chasing with the same packed node layout
  - `packed_index_chase` vs `split_soa_index_chase`: packed AoS node payload vs split SoA payload under the same integer dependency chain
  - any dependency-chasing case vs `flat_sequential`: dependency-chain cost vs streaming baseline
- Treat `matrix_blocking` as evidence about traversal order and cache reuse. It is not a replacement for tuned BLAS kernels.
- Treat `simd_scan` as a lane-utilization benchmark. If the compiler-vectorized candidate matches explicit SIMD closely, the right conclusion may be “the compiler is already good enough here.”
- Use perf counters where available to separate “faster because fewer misses” from “faster because fewer branches” from “faster because higher IPC.”
- Compare confidence intervals before claiming small deltas. If the intervals overlap materially, gather more evidence before concluding.

## Profiling Hooks

- Linux `perf stat` is the first optional counter path for cycles, instructions, cache misses, and branch misses.
- On this host, the validated counter workflow is the Dockerized Linux `perf stat` path. It avoids the native Windows PMC limitation while keeping the benchmark harness unchanged.
- On Windows, use an elevated shell plus Windows Performance Toolkit PMC tracing for the pointer-layout probe. The lab includes a first-pass wrapper, but PMC collection can be blocked by missing elevation or hypervisor policy.
- Criterion HTML output is the default latency/throughput surface; the summary binary converts estimates into JSON and Markdown for archival.
- For deeper cycle accounting, prefer fixed-frequency runs, pinned cores, and frame pointers in a follow-up profiling pass rather than mixing those controls into the baseline workflow immediately.

## Cross-References

- [Memory Hierarchy](../../01-foundations/04-memory-hierarchy.md)
- [CPU Optimization](../../03-performance-engineering/01-cpu-optimization.md)
- [Profiling and Benchmarking](../../03-performance-engineering/06-profiling-and-benchmarking.md)
- [Verification Methodology](../../08-execution/04-verification-methodology.md)
