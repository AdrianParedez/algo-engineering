# GitHub Repository Standards

Last verified: 2026-05-22

## Purpose

This note records the GitHub and engineering-repository practices applied while preparing this workspace for publication.

## Applied Standards

### 1. Repository health files are explicit, not implied

The repository now includes:

- `README.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CODE_OF_CONDUCT.md`
- `CITATION.cff`
- PR and issue templates under `.github/`

This follows current GitHub guidance to make contribution, citation, and conduct expectations discoverable from the repository itself.

### 2. Automation uses least privilege by default

GitHub Actions workflows use read-only default permissions unless a workflow requires more. The CodeQL workflow scopes `security-events: write` only to the analysis job.

### 3. CI validates docs and code separately

This repository is not only a Rust crate. It is a documentation-heavy engineering workspace with one benchmark lab. CI therefore checks:

- repository documentation integrity
- Rust formatting and linting
- Rust tests
- benchmark compilation without treating shared-runner timings as authoritative

### 4. Dependency automation is grouped and bounded

Dependabot is configured for:

- Cargo dependencies in `labs/cache-locality-lab`
- GitHub Actions workflows
- the lab Dockerfile

Updates are grouped to reduce PR noise and review fragmentation. Cargo automation is intentionally limited to patch and minor updates; semver-major Rust dependency changes stay manual because they can require API migration, benchmark reinterpretation, and toolchain-policy review rather than a blind merge.

### 5. Repository naming and licensing stay simple

The selected publication defaults are:

- recommended GitHub repository name: `algo-engineering`
- repository license: `Apache-2.0`

This keeps the public name short enough for normal use while still legible to an external reader, and it keeps the licensing model simple for a mixed code-and-documentation repository.

### 6. Benchmark governance is stricter than general CI

GitHub-hosted runners are useful for build correctness and smoke checks, not for final performance claims. This repository therefore avoids CI rules that would infer benchmark regressions from hosted-runner timing noise.

### 7. Publication blockers stay explicit

The following items are intentionally left as pre-publication decisions rather than guessed locally:

- final security contact path
- GitHub-side rulesets, alerts, and vulnerability-reporting toggles

Those are tracked in `docs/repository-publish-runbook.md`.

## Source Notes

- GitHub repository best practices recommend clear community health files such as a README, license, citation file, contribution guidelines, and code of conduct. Verified 2026-05-22.
- GitHub Actions documentation recommends least-privilege `GITHUB_TOKEN` permissions and unique job names when those jobs become required status checks. Verified 2026-05-22.
- GitHub-hosted runners are ephemeral virtual machines. That is useful for clean builds, but it is a poor foundation for authoritative performance claims in a benchmark-heavy repository. Verified 2026-05-22.
- Criterion.rs documentation and the Rust Performance Book both reinforce that benchmark interpretation requires disciplined environment control and should not collapse into casual timing conclusions. Verified 2026-05-22.

## Sources

- [GitHub Docs: Best practices for repositories](https://docs.github.com/en/repositories/creating-and-managing-repositories/best-practices-for-repositories)
- [GitHub Docs: Workflow syntax for GitHub Actions](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax)
- [GitHub Docs: Automatic token authentication and permissions](https://docs.github.com/en/actions/security-guides/automatic-token-authentication)
- [GitHub Docs: About GitHub-hosted runners](https://docs.github.com/en/actions/concepts/runners/about-github-hosted-runners)
- [GitHub Docs: Building and testing Rust](https://docs.github.com/en/actions/how-tos/use-cases-and-examples/building-and-testing/building-and-testing-rust)
- [GitHub Docs: Configuring issue templates for your repository](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/configuring-issue-templates-for-your-repository)
- [GitHub Docs: Creating a default community health file](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file)
- [GitHub Docs: Configuring code scanning for compiled languages](https://docs.github.com/en/code-security/code-scanning/creating-an-advanced-setup-for-code-scanning/configuring-code-scanning-for-compiled-languages)
- [GitHub Docs: Configuring Dependabot version updates](https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file)
- [Criterion.rs: Analysis Process](https://bheisler.github.io/criterion.rs/book/analysis.html)
- [The Rust Performance Book: Benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html)
