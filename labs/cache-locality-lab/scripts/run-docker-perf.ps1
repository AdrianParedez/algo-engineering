param(
    [string]$Variant = "split_soa_index_chase",
    [int]$Rounds = 128,
    [int]$WarmupRounds = 16,
    [string]$LinuxEvents = "cycles,instructions,cache-references,cache-misses,branches,branch-misses"
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot

Push-Location $projectRoot
try {
    & docker compose --profile perf build lab-perf
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }

    $command = @"
set -euo pipefail
export PATH="/usr/local/cargo/bin:`$PATH"
export PERF_EVENTS="$LinuxEvents"
export ROUNDS="$Rounds"
export WARMUP_ROUNDS="$WarmupRounds"
bash ./scripts/run-pointer-layout-perf-linux.sh "$Variant"
"@

    & docker compose --profile perf run --rm lab-perf bash -c $command
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
