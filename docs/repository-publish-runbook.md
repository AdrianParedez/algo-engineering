# Repository Publish Runbook

Last verified: 2026-05-22

## Purpose

This runbook records the publication sequence for this repository and the current post-publication baseline.

## Local State Already Prepared

- repository license file (`Apache-2.0`)
- root ignore and editor normalization files
- community health files
- issue and PR templates
- CI workflow for docs and Rust correctness
- CodeQL workflow for Rust and Actions
- Dependabot for Cargo, GitHub Actions, and Docker
- repository validation script

## Current Public State

Recommended repository name: `algo-engineering`

- Repository visibility: public
- Private vulnerability reporting: enabled
- Secret scanning: enabled
- Secret scanning push protection: enabled
- Dependabot alerts: enabled
- Dependabot security updates: enabled
- `main` ruleset: active
  - pull requests required
  - 1 approving review required
  - required checks: `workspace-docs-integrity`, `cache-locality-rust-ci-linux`, `cache-locality-rust-ci-windows`, `codeql-analyze`

## Optional Post-Publish Hardening

- Add repository topics so GitHub classification and discovery are less generic.
- Add a homepage URL if you later publish a standalone docs site, paper page, or project landing page.
- Decide whether to enable Discussions for community Q&A or keep Issues-only triage.
- Add an explicit off-platform security contact if you want a fallback beyond GitHub advisory intake.

## Create The Repository

From `X:\agent\algorithm-engineering`:

```powershell
git init -b main
git add .
git commit
gh repo create <owner>/algo-engineering --source . --remote origin --push
```

If you do not use `gh`, create the remote in GitHub first and then run:

```powershell
git remote add origin https://github.com/<owner>/algo-engineering.git
git push -u origin main
```

Use the repository commit template for the first commit. Recommended subject:

```text
ops(repo): prepare initial publication scaffold
```

## GitHub-Side Settings Baseline

### Enabled

- Dependabot alerts
- Dependabot security updates
- Secret scanning
- Private vulnerability reporting
- Code scanning through the included CodeQL workflow

### Recommended Ruleset For `main`

- require status checks before merge
- require pull requests
- require 1 approving review
- optionally add linear-history, force-push, and branch-deletion restrictions if you want a stricter merge policy later

Use these required status checks after the first successful workflow runs:

- `workspace-docs-integrity`
- `cache-locality-rust-ci-linux`
- `cache-locality-rust-ci-windows`
- `codeql-analyze` when CodeQL is enabled for the repository

`CODEOWNERS` is now configured. Only enable “require review from Code Owners” after you confirm the owner mapping is the one you want.

## Repository Features

- Enable Issues.
- Keep Discussions optional. Turn them on only if you want open-ended design conversation in the repo itself.
- Keep Actions enabled.

## Benchmark Policy After Publication

- Do not accept or reject performance changes based only on GitHub-hosted runner timings.
- Use CI for correctness, linting, and benchmark compilation.
- Use the local Docker perf path or a controlled self-hosted runner for performance evidence.

## First Post-Publish Verification

Completed baseline checks for the public repository should include:

1. `Repository CI` completes on `main`.
2. Dependabot opens update PRs in the expected directories.
   Cargo updates should be patch/minor only unless the maintainers intentionally re-enable semver-major automation.
3. CodeQL uploads results successfully.
4. Issue forms and PR templates render correctly.
5. `CITATION.cff` is recognized by GitHub.

## Sources

- [GitHub Docs: Managing a branch protection rule or ruleset](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule)
- [GitHub Docs: About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)
- [GitHub Docs: About code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
- [GitHub Docs: Adding a security policy to your repository](https://docs.github.com/en/code-security/getting-started/adding-a-security-policy-to-your-repository)
