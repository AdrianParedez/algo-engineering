# Commit Conventions

Last verified: 2026-05-22

## Purpose

This repository uses the scoped conventional-commit style already visible across the maintainer's recent GitHub history. The goal is not textbook purity. The goal is readable, high-signal history for a mixed documentation, tooling, and benchmark repository.

## Default Format

```text
type(scope): subject
```

Examples:

- `docs(workspace): establish algorithm engineering corpus`
- `feat(lab): scaffold cache locality benchmark suite`
- `perf(pointer-layout): separate allocator scatter from dependency chasing`
- `ops(repo): prepare initial publication scaffold`
- `ci(github): add Rust and docs validation workflows`

## Rules

- Use an imperative subject line.
- Keep the subject specific and compact.
- Add a body for any nontrivial change.
- Use the body to explain mechanism, rationale, and evidence.
- If a change affects benchmarking, mention the measurement path or artifact location.

## Core Types

- `feat`: new user-facing or developer-facing capability
- `fix`: correctness fix or defect repair
- `docs`: documentation or research-corpus changes
- `perf`: benchmark-driven performance improvement
- `test`: test coverage, regression harnesses, or validation extensions
- `ci`: workflow and automation changes
- `chore`: low-level maintenance without stronger semantic meaning
- `ops`: repository operations, publishing prep, housekeeping with operational intent
- `refactor`: structural change without intended behavioral change

## Allowed Extended Types

These are acceptable when they are clearer than `chore`:

- `config`
- `init`
- `improvement`
- `protocol`
- `signal`
- `patch`

## Preferred Scopes For This Repository

- `repo`
- `workspace`
- `readme`
- `github`
- `lab`
- `pointer-layout`
- `matrix-blocking`
- `simd-scan`
- `docker`
- `benchmarking`
- `profiling`
- `security`

## Body Guidance

Use a body when the change is substantial, architectural, benchmark-sensitive, or spans multiple surfaces.

Good body content:

- what was added, changed, or removed
- why the change was necessary
- how the mechanism works
- what evidence or validation supports it

Avoid:

- repeating the subject in sentence form
- filler like "minor updates"
- unexplained benchmark claims

## First Commit Recommendation

For the initial repository import, use:

```text
ops(repo): prepare initial publication scaffold
```

Recommended body:

```text
Add the algorithm engineering workspace, benchmark lab, repository
health files, CI workflows, and publication runbooks.

Keep benchmark evidence standards, security posture, and Docker perf
workflow explicit before the first remote push.
```

