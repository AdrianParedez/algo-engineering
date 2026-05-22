# 2026-05-22 Container Perf Analysis

This note records the first complete containerized `perf stat` sweep of the tightened pointer-layout family on this host. It is the canonical interpretation of the current result set.

## Scope

- Workload: `pointer_layout_probe`
- Execution path: Docker `lab-perf` profile
- Rounds: `32`
- Warmup rounds: `4`
- Events: `cycles,instructions,cache-references,cache-misses,branches,branch-misses`
- Source artifacts:
  - [docker-perf-summary.md](./docker-perf-summary.md)
  - [docker-perf-summary.json](./docker-perf-summary.json)
  - [boxed_pointer_chain-perf-stat.txt](./boxed_pointer_chain-perf-stat.txt)
  - [arena_pointer_chain-perf-stat.txt](./arena_pointer_chain-perf-stat.txt)
  - [packed_index_chase-perf-stat.txt](./packed_index_chase-perf-stat.txt)
  - [split_soa_index_chase-perf-stat.txt](./split_soa_index_chase-perf-stat.txt)

## Result Table

| variant | ns/round | rel to fastest | IPC | cache miss % | branch miss % | L1D miss % | pages touched |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `packed_index_chase` | 22,116,042 | 1.00x | 0.16 | 47.76% | 0.66% | 25.67% | 2,049 |
| `split_soa_index_chase` | 24,480,931 | 1.11x | 0.16 | 49.44% | 0.59% | 40.02% | 2,050 |
| `arena_pointer_chain` | 27,994,108 | 1.27x | 0.11 | 60.58% | 0.96% | 25.03% | 2,049 |
| `boxed_pointer_chain` | 35,858,791 | 1.62x | 0.08 | 72.56% | 1.25% | 40.53% | 4,097 |

## Main Findings

- Allocator scatter is a first-order cost. `boxed_pointer_chain` touches about twice as many 4 KiB pages as the contiguous layouts and is the slowest variant by a wide margin.
- Contiguous pointer chasing is materially better than boxed pointer chasing. `boxed_pointer_chain -> arena_pointer_chain` yields about `1.28x` speedup without changing the dependency shape.
- Replacing raw pointer chasing with integer chasing helps again. `arena_pointer_chain -> packed_index_chase` yields another `1.27x` speedup while keeping a packed node layout.
- The fastest dependent layout on this host is `packed_index_chase`, not split SoA.

## Why Packed Beats Split SoA Here

The important comparison is `packed_index_chase` vs `split_soa_index_chase`.

- Both variants keep allocation contiguous.
- Both variants retain a serialized dependency chain.
- Both variants retire about the same number of instructions and reach the same IPC.
- Their overall cache-miss percentages are close.

The differentiator is L1 data behavior and access shape:

- `packed_index_chase` keeps `value` and `next` in the same node payload, so one dependent access tends to bring both fields into the same fetched line.
- `split_soa_index_chase` separates `values[index]` and `next_indices[index]` into different arrays. For this dependent random walk, that layout increases the chance that the next required field lives in a different cache line.
- The measured signal matches that expectation: `split_soa_index_chase` shows a much higher L1 data miss rate (`40.02%` vs `25.67%`) and loses about `11%` in wall time.

This is not a general argument against SoA. It is specific to this benchmark shape: one value and one next-hop fetched per dependent step, with little opportunity to amortize separate arrays across SIMD lanes or streaming reuse.

## What The Counters Say

- Branch behavior is not the bottleneck. Branch miss rates stay below `1.25%` for all four variants.
- IPC is low everywhere, which is typical for miss-dominated serialized traversals. The cores are frequently waiting on memory rather than retiring useful work.
- `boxed_pointer_chain` combines the worst page footprint, the worst cache miss percentage, and the lowest IPC. That is the cleanest evidence that the allocator effect was real.
- `arena_pointer_chain` removes most of the page-footprint penalty, but pointer dependency still leaves it slower than the integer-chasing variants.

## Interpretation Boundary

- Verified fact: on this host, under this containerized perf path, `packed_index_chase` is the fastest tightened dependent layout.
- Verified fact: allocator scatter materially hurts the boxed layout.
- Inference: the packed layout wins over split SoA because co-locating `value` and `next` improves line usefulness in this dependent random-walk pattern.
- Open question: whether `split_soa_index_chase` would recover under a wider workload that consumes multiple fields per node or exposes vectorizable batched traversals.

## Next Measurements

- Repeat the four-variant sweep across multiple sample counts and report spread across runs, not just a single sweep.
- Add per-variant core pinning if the host environment allows it cleanly.
- Test a batched multi-walker variant to see whether SoA improves once the access pattern exposes more memory-level parallelism.
- Add larger event sets if the container environment exposes them. `LLC-loads` and `LLC-load-misses` were not supported in the current path.
