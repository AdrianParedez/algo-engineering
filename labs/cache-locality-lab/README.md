# Cache Locality Lab

Focused Rust benchmark lab for three initial algorithm-engineering comparisons:

- pointer-chasing vs flat contiguous traversal
- naive matrix traversal vs cache-blocked multiplication
- scalar byte scan vs compiler-vectorized candidate vs explicit SIMD

The pointer-layout family is intentionally split into allocator, dependency, and
layout subcases:

- `boxed_pointer_chain`: separate heap boxes plus pointer dependency
- `arena_pointer_chain`: contiguous node arena plus pointer dependency
- `packed_index_chase`: contiguous `{value,next}` nodes with integer chasing
- `split_soa_index_chase`: split `values[]` and `next[]` arrays with integer chasing
- `flat_sequential`: no dependency-chain baseline

## Structure

- `src/kernels/`: implementations only
- `src/experiments/`: deterministic inputs, case metadata, and correctness checks
- `benches/`: Criterion entry points
- `scripts/`: repeatable run and perf wrappers
- `docs/`: runbooks and operational notes
- `reports/`: generated benchmark summaries and artifacts

## Run

```powershell
cd X:\agent\algorithm-engineering\labs\cache-locality-lab
cargo bench --bench cache_locality
cargo run --bin criterion_summary -- --criterion-root target\criterion --json-out reports\latest-summary.json --markdown-out reports\latest-summary.md
```

For a single family during iteration:

```powershell
cargo bench --bench cache_locality pointer_layout
```

For an isolated pointer-layout probe outside Criterion:

```powershell
cargo run --release --bin pointer_layout_probe -- --variant split_soa_index_chase --rounds 64 --warmup-rounds 8
```

To archive probe outputs for all pointer variants:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\profile-pointer-layout.ps1
```

## Docker

Build and run the lab in a Linux container:

```powershell
docker compose build lab
docker compose run --rm lab cargo test
```

Run the normal Criterion workflow inside Docker:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run-docker-bench.ps1 -BenchFilter pointer_layout
```

Run the perf-oriented pointer-layout probe inside Docker:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run-docker-perf.ps1 -Variant split_soa_index_chase
```

Run all four tightened pointer-layout variants through the Docker perf profile and generate a comparison summary:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\profile-pointer-layout-docker.ps1
```

## Notes

- Criterion defaults are left intact for warm-up, sample collection, and variance estimates.
- Kernels use `black_box` at bench call sites instead of ad-hoc anti-optimization tricks.
- The explicit SIMD path uses stable `std::arch` AVX2 intrinsics on `x86`/`x86_64` and falls back cleanly when unavailable.
- Perf-counter collection has two optional paths:
  - Linux `perf` via `scripts/run-perf-linux.sh` and `scripts/run-perf.ps1`
  - Dockerized Linux `perf` via `compose.yaml`, `scripts/run-pointer-layout-perf-linux.sh`, and `scripts/run-docker-perf.ps1`
  - Windows Performance Toolkit PMC capture for the pointer-layout probe via `scripts/profile-pointer-layout.ps1` when running in an elevated shell
- The Docker perf profile grants `CAP_PERFMON`, `CAP_SYS_PTRACE`, `CAP_IPC_LOCK`, and `seccomp=unconfined`. On this host, that path is currently the working counter workflow and produces usable `perf stat` output under Docker Desktop.
- Native Windows WPT PMC capture is still present for comparison, but on this host it remains unhelpful under the active Hyper-V / VBS stack. See `reports/pointer-layout/pmc-status.txt`.

Methodology and interpretation guidance live in [METHODOLOGY.md](./METHODOLOGY.md).

Current operational docs:

- [docs/docker-perf-runbook.md](./docs/docker-perf-runbook.md)
- [reports/pointer-layout/README.md](./reports/pointer-layout/README.md)
- [reports/pointer-layout/2026-05-22-container-perf-analysis.md](./reports/pointer-layout/2026-05-22-container-perf-analysis.md)
