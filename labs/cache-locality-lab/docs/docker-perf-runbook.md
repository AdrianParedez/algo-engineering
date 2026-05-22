# Docker Perf Runbook

This runbook defines the preferred local counter workflow for the cache-locality lab on the current Windows host.

## Purpose

Use the Docker `lab-perf` profile when you need hardware-counter-oriented comparisons for the pointer-layout probe without relying on native Windows PMC capture.

## Preconditions

- Docker Desktop is installed and running.
- The workspace path is mounted into Docker successfully.
- The lab image builds from [Dockerfile](../Dockerfile).
- The perf-enabled service definition in [compose.yaml](../compose.yaml) is unchanged:
  - `cap_add`: `PERFMON`, `SYS_PTRACE`, `IPC_LOCK`
  - `security_opt`: `seccomp=unconfined`
  - `memlock`: unlimited

## Standard Commands

Build the base image:

```powershell
docker compose --profile perf build lab-perf
```

Run one variant:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run-docker-perf.ps1 -Variant split_soa_index_chase
```

Run the full tightened sweep and generate summaries:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\profile-pointer-layout-docker.ps1
```

Run the normal Criterion workflow in Docker:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run-docker-bench.ps1 -BenchFilter pointer_layout
```

## Outputs

- Variant raw perf outputs: `reports/pointer-layout/*-perf-stat.txt`
- Sweep summary: `reports/pointer-layout/docker-perf-summary.md`
- Sweep JSON: `reports/pointer-layout/docker-perf-summary.json`

## Event Set

Default event set:

```text
cycles,instructions,cache-references,cache-misses,branches,branch-misses
```

The wrapper also captures any extra events that `perf stat -d` reports by default, such as `L1-dcache-loads` and `L1-dcache-load-misses`, when the container environment exposes them.

## Interpretation Rules

- Treat `ns/round` as the primary ranking metric for the probe.
- Use `IPC` to confirm whether a variant is stall-heavy. Low IPC in this workload usually means serialized memory wait, not branch overhead.
- Use `cache miss %` and `L1D miss %` to separate allocator/locality effects from pure dependency effects.
- Use `branch miss %` defensively. If it stays low, do not over-attribute the result to control flow.
- Unsupported events are an environment fact, not a benchmark failure. On this host, `LLC-loads` and `LLC-load-misses` are not currently available through the container path.

## Failure Modes

- Container starts but `perf` fails: check that Docker Desktop is running with the perf-enabled profile and that no local policy removed `PERFMON` or seccomp overrides.
- The script runs but counters are all zeros or unsupported: verify the host kernel path and compare against [reports/pointer-layout/pmc-status.txt](../reports/pointer-layout/pmc-status.txt).
- One run differs sharply from prior runs: rerun the full sweep before concluding. This lab is still sensitive to host noise and container scheduling.
- Criterion and probe timings disagree materially: check whether the probe rounds or warmup values were changed, and confirm the same variant set and binary profile were used.

## Current Host-Specific Notes

- Native Windows WPT PMC capture remains less useful on this machine under the active Hyper-V / VBS stack.
- The Dockerized Linux `perf stat` path does work on this host and is the preferred current local profiling workflow.
- The present result set is documented in [2026-05-22-container-perf-analysis.md](../reports/pointer-layout/2026-05-22-container-perf-analysis.md).
