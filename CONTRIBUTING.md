# Contributing

## Scope

This repository contains two coupled layers:

- a documentation-first algorithm-engineering workspace
- a runnable Rust benchmark lab at `labs/cache-locality-lab`

Changes should preserve both documentation integrity and experiment integrity.

## Ground Rules

- Prefer small pull requests with a single clear objective.
- Keep claims falsifiable. If a change implies a performance conclusion, include the mechanism and the measurement path.
- Do not merge benchmark-driven conclusions without raw artifacts or a reproducible rerun path.
- Do not treat GitHub-hosted runner timings as authoritative performance evidence.

## Commit Convention

- Default format: `type(scope): subject`
- Use imperative subject lines.
- Add a body for any nontrivial change.
- Prefer meaningful scopes such as `repo`, `workspace`, `github`, `lab`, `pointer-layout`, or `profiling`.
- Extended types beyond the basic conventional set are acceptable when they are clearer than `chore`.

Full policy and examples live in [docs/commit-conventions.md](docs/commit-conventions.md).

## Before Opening a Pull Request

Run these commands from the repository root:

```powershell
python .\scripts\validate_repo.py
cd .\labs\cache-locality-lab
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo bench --bench cache_locality --no-run
```

If you changed benchmark kernels, experiments, or methodology, also rerun the relevant probe or benchmark path and retain the raw outputs under `labs/cache-locality-lab/reports/`.

## Documentation Standards

- Numbered corpus documents and support-layer documents must keep the required YAML front matter.
- Use relative links for repository-internal references.
- Keep writing dense and implementation-oriented. Remove filler before opening the PR.
- If a claim is time-sensitive, state the verification date explicitly.

## Benchmark Evidence Standards

For any claimed performance effect, include:

- workload and input identity
- host OS and CPU context
- build profile and toolchain
- command used to run the benchmark or probe
- raw output path
- interpretation of the mechanism, not just the delta

For this repository, the preferred local counter workflow is the Docker perf path documented in `labs/cache-locality-lab/docs/docker-perf-runbook.md`.

## Pull Request Review Expectations

- Link the issue or problem statement.
- Explain what changed and what evidence supports it.
- Call out any remaining risks, unsupported environments, or unverified assumptions.
- If placeholders were introduced for repository publication, mark them clearly in the PR description.
