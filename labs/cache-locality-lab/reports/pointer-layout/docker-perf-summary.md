# Docker perf summary

Generated from containerized perf stat runs of the four tightened pointer-layout variants.

Run parameters:

| setting | value |
| --- | --- |
| rounds | 32 |
| warmup_rounds | 4 |
| events | `cycles,instructions,cache-references,cache-misses,branches,branch-misses` |

## Comparison

| variant | interpretation | ns/round | rel to fastest | cycles | instructions | IPC | cache miss % | branch miss % | L1D miss % | footprint pages |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| packed_index_chase | contiguous allocation + integer dependency + packed node payload | 22,116,042 | 1.00x | 3,670,603,836 | 589,201,579 | 0.16 | 47.76% | 0.66% | 25.67% | 2,049 |
| split_soa_index_chase | contiguous allocation + integer dependency + split value/next arrays | 24,480,931 | 1.11x | 3,589,081,550 | 588,760,319 | 0.16 | 49.44% | 0.59% | 40.02% | 2,050 |
| arena_pointer_chain | contiguous allocation + pointer dependency + packed node payload | 27,994,108 | 1.27x | 4,074,024,971 | 466,700,646 | 0.11 | 60.58% | 0.96% | 25.03% | 2,049 |
| boxed_pointer_chain | allocator scatter + pointer dependency + packed node payload | 35,858,791 | 1.62x | 5,666,341,627 | 467,156,248 | 0.08 | 72.56% | 1.25% | 40.53% | 4,097 |

## Findings

- Fastest variant: `packed_index_chase` at 22,116,042 ns/round.
- Slowest variant: `boxed_pointer_chain` at 35,858,791 ns/round.
- `boxed_pointer_chain -> arena_pointer_chain`: 1.28x speedup from removing allocator scatter while keeping pointer dependency.
- `arena_pointer_chain -> packed_index_chase`: 1.27x speedup from replacing raw pointer chasing with integer chasing in a packed node layout.
- `boxed_pointer_chain -> split_soa_index_chase`: 1.46x speedup from removing both allocator scatter and raw pointer indirection.
- Branch miss rates stay low across all four variants, so the dominant signal on this host is memory behavior rather than control-flow instability.
- The boxed layout spans roughly twice as many 4 KiB pages as the contiguous layouts, matching the allocator-scatter hypothesis from the tightened design.
- `boxed_pointer_chain` also shows the highest overall cache-miss rate in this run, which is consistent with its slower time and larger page footprint.

