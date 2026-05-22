param(
    [string]$BenchFilter = "",
    [string]$CriterionRoot = "target\\criterion",
    [string]$JsonOut = "reports\\latest-summary.json",
    [string]$MarkdownOut = "reports\\latest-summary.md"
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot

Push-Location $projectRoot
try {
    $cargoArgs = @("bench", "--bench", "cache_locality")
    if ($BenchFilter) {
        $cargoArgs += $BenchFilter
    }

    & cargo @cargoArgs
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }

    & cargo run --bin criterion_summary -- --criterion-root $CriterionRoot --json-out $JsonOut --markdown-out $MarkdownOut
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
