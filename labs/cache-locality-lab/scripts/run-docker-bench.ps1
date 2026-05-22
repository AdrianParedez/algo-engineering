param(
    [string]$BenchFilter = "",
    [string]$CriterionRoot = "target/criterion",
    [string]$JsonOut = "reports/latest-summary.json",
    [string]$MarkdownOut = "reports/latest-summary.md"
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot

Push-Location $projectRoot
try {
    & docker compose build lab
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }

    $benchExpr = "cargo bench --bench cache_locality"
    if ($BenchFilter) {
        $benchExpr += " $BenchFilter"
    }

    $command = @"
set -euo pipefail
export PATH="/usr/local/cargo/bin:`$PATH"
$benchExpr
cargo run --bin criterion_summary -- --criterion-root $CriterionRoot --json-out $JsonOut --markdown-out $MarkdownOut
"@

    & docker compose run --rm lab bash -c $command
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
