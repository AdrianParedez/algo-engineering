param(
    [string]$BenchFilter = "",
    [string]$LinuxEvents = "cycles,instructions,cache-references,cache-misses,branches,branch-misses"
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot

if (-not (Get-Command wsl -ErrorAction SilentlyContinue)) {
    Write-Error "Linux perf collection requires WSL or a native Linux environment."
}

$wslProjectRoot = (& wsl wslpath ($projectRoot -replace '\\', '\\')).Trim()
if (-not $wslProjectRoot) {
    Write-Error "Failed to translate the project path into a WSL path."
}

$escapedFilter = $BenchFilter.Replace('"', '\"')
$escapedEvents = $LinuxEvents.Replace('"', '\"')
$command = "cd `"$wslProjectRoot`" && PERF_EVENTS=`"$escapedEvents`" ./scripts/run-perf-linux.sh `"$escapedFilter`""

& wsl bash -lc $command
exit $LASTEXITCODE
