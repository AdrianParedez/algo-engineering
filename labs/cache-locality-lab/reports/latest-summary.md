# Criterion Summary

| Benchmark | Mean (ns) | Median (ns) | Std Dev (ns) | 95% CI Lower | 95% CI Upper | Throughput |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| matrix_blocking/blocked_32/n128-2097152-macs | 850677.00 | 863623.96 | 94695.16 | 832329.48 | 869155.28 | 2465274134.28 macs/s |
| matrix_blocking/naive_ijk/n128-2097152-macs | 2546971.19 | 2364354.76 | 537715.71 | 2450900.85 | 2661045.62 | 823390546.32 macs/s |
| pointer_layout/arena_pointer_chain/524288-elements-4194304-bytes | 28819563.00 | 26306200.00 | 6879347.65 | 27500077.20 | 30192264.25 | 145536696.72 bytes/s |
| pointer_layout/boxed_pointer_chain/524288-elements-4194304-bytes | 38254927.50 | 36723675.00 | 3406384.81 | 37617175.48 | 38944620.55 | 109640882.21 bytes/s |
| pointer_layout/flat_sequential/524288-elements-4194304-bytes | 114170.84 | 107914.80 | 14341.58 | 111471.75 | 117055.10 | 36737085358.45 bytes/s |
| pointer_layout/packed_index_chase/524288-elements-4194304-bytes | 21588687.00 | 21040025.00 | 2217649.82 | 21178688.38 | 22044016.05 | 194282496.20 bytes/s |
| pointer_layout/split_soa_index_chase/524288-elements-4194304-bytes | 19366346.67 | 17872066.67 | 4171911.54 | 18616005.23 | 20240098.66 | 216576934.83 bytes/s |
| simd_scan/compiler_vectorized_candidate/1048576-bytes | 455059.62 | 432319.96 | 62409.79 | 443556.41 | 467874.30 | 2304260706.30 bytes/s |
| simd_scan/explicit_simd/1048576-bytes | 59137.80 | 57761.40 | 6224.97 | 57956.96 | 60389.33 | 17731061643.69 bytes/s |
| simd_scan/scalar_branchy/1048576-bytes | 450975.28 | 441546.02 | 46705.64 | 442107.00 | 460392.37 | 2325129674.35 bytes/s |
