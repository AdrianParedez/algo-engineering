param(
    [string[]]$Variants = @(
        "boxed_pointer_chain",
        "arena_pointer_chain",
        "packed_index_chase",
        "split_soa_index_chase"
    ),
    [int]$Rounds = 64,
    [int]$WarmupRounds = 8,
    [switch]$EnableWindowsPmc
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot
$reportRoot = Join-Path $projectRoot "reports\\pointer-layout"
$xperf = "C:\\Program Files (x86)\\Windows Kits\\10\\Windows Performance Toolkit\\xperf.exe"
$windowsPmcCounters = "InstructionRetired,TotalCycles,LLCMisses,BranchMispredictsRetired"

function Test-IsAdministrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

New-Item -ItemType Directory -Force $reportRoot | Out-Null

Push-Location $projectRoot
try {
    if ($EnableWindowsPmc) {
        if (-not (Test-Path $xperf)) {
            Write-Error "xperf.exe was not found at the expected Windows Performance Toolkit path."
        }

        if (-not (Test-IsAdministrator)) {
            Write-Error "Windows PMC capture requires an elevated PowerShell session."
        }
    }

    & cargo build --release --bin pointer_layout_probe
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }

    foreach ($variant in $Variants) {
        $probePath = Join-Path $reportRoot "$variant-probe.json"
        & cargo run --release --bin pointer_layout_probe -- --variant $variant --rounds $Rounds --warmup-rounds $WarmupRounds | Out-File -FilePath $probePath -Encoding utf8
        if ($LASTEXITCODE -ne 0) {
            exit $LASTEXITCODE
        }

        if (-not $EnableWindowsPmc) {
            continue
        }

        $tracePath = Join-Path $reportRoot "$variant-pmc.etl"
        $pmcReportPath = Join-Path $reportRoot "$variant-pmc.txt"

        if (Test-Path $tracePath) { Remove-Item $tracePath -Force }
        if (Test-Path $pmcReportPath) { Remove-Item $pmcReportPath -Force }

        & $xperf -stop 2>$null
        & $xperf -on BASE+CSWITCH+PROC_THREAD+LOADER -pmc $windowsPmcCounters CSWITCH strict -f $tracePath
        if ($LASTEXITCODE -ne 0) {
            exit $LASTEXITCODE
        }

        & cargo run --release --bin pointer_layout_probe -- --variant $variant --rounds $Rounds --warmup-rounds $WarmupRounds | Out-Null
        $probeExit = $LASTEXITCODE
        & $xperf -d $tracePath
        if ($probeExit -ne 0) {
            exit $probeExit
        }

        & $xperf -i $tracePath -a pmc | Out-File -FilePath $pmcReportPath -Encoding utf8
        if ($LASTEXITCODE -ne 0) {
            exit $LASTEXITCODE
        }
    }
}
finally {
    Pop-Location
}
