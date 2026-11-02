<#
.SYNOPSIS
    定期清理 Rust 编译产物，防止 target 无限膨胀。

.DESCRIPTION
    膨胀主因是 target\<profile>\incremental：每个 crate 每换一次编译配置
    （RUSTFLAGS / feature / llvm-cov 等）就会新开一棵增量树，旧的永不回收。
    本脚本优先只删增量缓存（不影响已编译的依赖，下次编译只是慢一点），
    只有体积仍然超标时才执行完整 cargo clean。

.PARAMETER SoftLimitGB
    超过该体积就先清 incremental 缓存。默认 30 GB。

.PARAMETER HardLimitGB
    清完 incremental 仍超过该体积，则执行 cargo clean。默认 80 GB。

.PARAMETER Force
    忽略体积阈值，强制执行一次 incremental 清理。

.PARAMETER DryRun
    只报告，不实际删除。

.EXAMPLE
    .\clean-target.ps1
    按默认阈值检查并清理。

.EXAMPLE
    .\clean-target.ps1 -DryRun
    只看当前占用，不动文件。
#>
[CmdletBinding()]
param(
    [int]$SoftLimitGB = 30,
    [int]$HardLimitGB = 80,
    [switch]$Force,
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'
$projectRoot = $PSScriptRoot

# target 目录可能被 .cargo/config.toml 的 [build] target-dir 改到别处，
# 因此优先用 cargo metadata 解析真实路径，取不到再回退到默认位置。
$targetDir = $null
try {
    Push-Location $projectRoot
    $meta = & cargo metadata --no-deps --format-version 1 2>$null
    if ($LASTEXITCODE -eq 0) {
        $targetDir = (($meta -join '') | ConvertFrom-Json).target_directory
    }
} catch {
} finally {
    Pop-Location
}
if ([string]::IsNullOrWhiteSpace($targetDir)) {
    $targetDir = Join-Path $projectRoot 'target'
}

$logFile     = Join-Path $targetDir '.clean-target.log'

function Get-DirSizeGB {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) { return 0 }
    $sum = (Get-ChildItem -LiteralPath $Path -Recurse -Force -File -ErrorAction SilentlyContinue |
            Measure-Object -Property Length -Sum).Sum
    if ($null -eq $sum) { return 0 }
    return [math]::Round($sum / 1GB, 2)
}

function Write-Log {
    param([string]$Message)
    $line = "[{0}] {1}" -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss'), $Message
    Write-Host $line
    if (-not $DryRun) {
        try {
            $parent = Split-Path -Parent $logFile
            if (-not (Test-Path -LiteralPath $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }
            Add-Content -LiteralPath $logFile -Value $line -Encoding utf8 -ErrorAction SilentlyContinue
        } catch { }
    }
}

if (-not (Test-Path -LiteralPath $targetDir)) {
    Write-Log "target 不存在，无需清理：$targetDir"
    return
}

$totalGB = Get-DirSizeGB $targetDir
Write-Log ("当前 target 体积：{0} GB（软阈值 {1} GB / 硬阈值 {2} GB）" -f $totalGB, $SoftLimitGB, $HardLimitGB)

if ($DryRun) {
    Get-ChildItem -LiteralPath $targetDir -Directory -Force -ErrorAction SilentlyContinue |
        ForEach-Object {
            $s = Get-DirSizeGB $_.FullName
            if ($s -gt 0) { Write-Host ("  {0,-28} {1,8} GB" -f $_.Name, $s) }
        }
    Write-Log "DryRun 模式，未做任何改动。"
    return
}

if (-not $Force -and $totalGB -le $SoftLimitGB) {
    Write-Log "未超过软阈值，跳过。"
    return
}

# 第一步：删除所有 profile 下的 incremental 缓存（代价最小，只丢增量编译状态）
$incrementalDirs = Get-ChildItem -LiteralPath $targetDir -Directory -Force -ErrorAction SilentlyContinue |
    ForEach-Object { Join-Path $_.FullName 'incremental' } |
    Where-Object { Test-Path -LiteralPath $_ }

if ($incrementalDirs.Count -eq 0) {
    Write-Log "未发现 incremental 目录。"
} else {
    foreach ($dir in $incrementalDirs) {
        $sizeGB = Get-DirSizeGB $dir
        Write-Log ("清理 incremental：{0}（{1} GB）" -f $dir, $sizeGB)
        try {
            [System.IO.Directory]::Delete($dir, $true)
            Write-Log "  完成。"
        } catch {
            Write-Log ("  部分失败：{0}" -f $_.Exception.Message)
        }
    }
}

# 第二步：清理后仍然超标，才做完整 cargo clean
$afterGB = Get-DirSizeGB $targetDir
Write-Log ("清理后 target 体积：{0} GB" -f $afterGB)

if ($afterGB -gt $HardLimitGB) {
    Write-Log "仍超过硬阈值 $HardLimitGB GB，执行 cargo clean ..."
    try {
        Push-Location $projectRoot
        try {
            & cargo clean 2>&1 | ForEach-Object { Write-Log $_ }
        } finally {
            Pop-Location
        }
        Write-Log ("cargo clean 完成，target 体积：{0} GB" -f (Get-DirSizeGB $targetDir))
    } catch {
        Write-Log ("cargo clean 失败：{0}" -f $_.Exception.Message)
    }
}

Write-Log "清理结束。"
