# Repository Publish Runbook

Last verified: 2026-05-22

## Purpose

This runbook covers the remaining steps after local preparation and before or immediately after creating the GitHub remote.

## Local State Already Prepared

- repository license file (`Apache-2.0`)
- root ignore and editor normalization files
- community health files
- issue and PR templates
- CI workflow for docs and Rust correctness
- CodeQL workflow for Rust and Actions
- Dependabot for Cargo, GitHub Actions, and Docker
- repository validation script

## Blockers To Resolve Before Public Publication

Recommended repository name: `algo-engineering`

1. Enable GitHub private vulnerability reporting immediately after repo creation, or add an explicit maintainer security contact to `SECURITY.md`.
2. Decide whether the repository will be public or private.

## Optional Pre-Publish Hardening

- Replace `.github/CODEOWNERS.template` with a real `.github/CODEOWNERS` file if you want Code Owner review policy.
- Enable a ruleset for `main` after the first successful CI run.

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

## GitHub-Side Settings To Enable

### Recommended Immediately

- Dependabot alerts
- Dependabot security updates
- Secret scanning, if your plan supports it
- Private vulnerability reporting
- Code scanning with the included CodeQL workflow, if your repository visibility and plan support it

### Recommended Ruleset For `main`

- require pull requests
- require status checks before merge
- require linear history
- block force pushes
- block branch deletion

Use these required status checks after the first successful workflow runs:

- `workspace-docs-integrity`
- `cache-locality-rust-ci-linux`
- `cache-locality-rust-ci-windows`
- `codeql-analyze` when CodeQL is enabled for the repository

Only enable “require review from Code Owners” after a real `.github/CODEOWNERS` file is in place.

## Repository Features

- Enable Issues.
- Keep Discussions optional. Turn them on only if you want open-ended design conversation in the repo itself.
- Keep Actions enabled.

## Benchmark Policy After Publication

- Do not accept or reject performance changes based only on GitHub-hosted runner timings.
- Use CI for correctness, linting, and benchmark compilation.
- Use the local Docker perf path or a controlled self-hosted runner for performance evidence.

## First Post-Publish Verification

After the remote exists, confirm:

1. `Repository CI` completes on `main`.
2. Dependabot opens update PRs in the expected directories.
3. CodeQL uploads results if the repository plan supports it.
4. Issue forms and PR templates render correctly.
5. `CITATION.cff` is recognized by GitHub.

## Sources

- [GitHub Docs: Managing a branch protection rule or ruleset](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule)
- [GitHub Docs: About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)
- [GitHub Docs: About code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)
- [GitHub Docs: Adding a security policy to your repository](https://docs.github.com/en/code-security/getting-started/adding-a-security-policy-to-your-repository)
