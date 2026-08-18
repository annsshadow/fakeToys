# recover_repo_sync.ps1
# 用途：在本机（真机）一键修复 OA4Rust 仓库的 push/pull 同步卡点。
# 安全说明：本脚本不做 reset --hard、不盲目 git add -A；
#           遇到 rebase 冲突会暂停并给出指引，不会自动 abort；
#           所有写操作前都会先校验仓库状态。可重复运行（幂等友好）。
# 用法：在仓库根目录打开 PowerShell，执行  .\recover_repo_sync.ps1
param()

$git = 'git'
$ErrorActionPreference = 'Continue'
$repo = $PSScriptRoot
if (-not (Test-Path (Join-Path $repo '.git'))) { $repo = Get-Location }
Set-Location $repo

function Log($m){ Write-Host "[$(Get-Date -Format 'HH:mm:ss')] $m" }

Log "仓库目录: $repo"
Log "git 版本 : $(& $git --version 2>&1)"

# 0) 基本校验
$inside = & $git rev-parse --is-inside-work-tree 2>&1
if ($inside -ne 'true') {
    Log "错误：当前不是 git 仓库，或 .git 损坏。请在真机仓库根目录运行。退出。"
    exit 1
}
$branch = & $git rev-parse --abbrev-ref HEAD 2>&1
Log "当前分支: $branch"

# 1) 修复损坏的索引条目（index 引用了对象库中不存在的 blob）
Log "检查并修复索引中引用了丢失对象的条目 ..."
$lines = & $git ls-files -s 2>&1
$bad = 0
foreach ($line in $lines) {
    if ($line -match '^(\d+)\s+([0-9a-f]{40})\s+(\d+)\s+(.*)$') {
        $sha = $Matches[2]; $stage = $Matches[3]; $path = $Matches[4]
        if ($stage -eq '0') {
            & $git cat-file -e "$sha" 2>&1 | Out-Null
            if ($LASTEXITCODE -ne 0) {
                Log "  缺失对象 $sha 用于 $path -> 从工作树重建"
                & $git add -- "$path" 2>&1 | Out-Null
                if ($LASTEXITCODE -ne 0) {
                    Log "  git add 失败，改为强制移除索引项: $path"
                    & $git update-index --force-remove -- "$path" 2>&1 | Out-Null
                }
                $bad++
            }
        }
    }
}
Log "索引修复完成，重建/移除条目数: $bad"

Log "git status 摘要:"
& $git status -sb 2>&1 | Select-Object -First 30 | ForEach-Object { Log "  $_" }

# 2) fetch（带连接重置重试）
Log "fetch origin ..."
$fetched = $false
for ($i=1; $i -le 4; $i++) {
    $r = & $git fetch origin 2>&1
    if ($LASTEXITCODE -eq 0) { $fetched = $true; Log "  fetch 成功 (第 $i 次)"; break }
    Log "  fetch 失败 (第 $i 次): $($r -join ' ')"
    Start-Sleep -Seconds 2
}
if (-not $fetched) { Log "fetch 多次失败，请检查网络/凭证后重试。退出。"; exit 1 }

# 3) 计算 ahead / behind
$upstream = & $git rev-parse --abbrev-ref --symbolic-full-name "@{u}" 2>&1
if ($upstream -notmatch '^origin/') {
    Log "未设置上游，尝试设置 origin/$branch ..."
    & $git branch --set-upstream-to="origin/$branch" 2>&1 | Out-Null
    $upstream = "origin/$branch"
}
$ahead  = [int](& $git rev-list --count "HEAD..$upstream" 2>&1)
$behind = [int](& $git rev-list --count "$upstream..HEAD" 2>&1)
Log "相对 $upstream : ahead=$ahead behind=$behind"

# 4) 同步逻辑
if ($behind -gt 0 -and $ahead -gt 0) {
    Log "分叉：先 stash 再 rebase 到 $upstream ..."
    $stashMsg = "auto-sync-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
    & $git stash push -u -m $stashMsg 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) { Log "stash 失败，请手动处理索引/工作树后重试。退出。"; exit 1 }
    Log "已 stash ($stashMsg)"
    & $git rebase "$upstream" 2>&1
    if ($LASTEXITCODE -ne 0) {
        Log "rebase 出现冲突或失败。请手动解决冲突后执行 git rebase --continue；"
        Log "或执行 git rebase --abort 取消。脚本在此暂停，不自动 abort。"
        exit 2
    }
    Log "rebase 完成，推送 ..."
    & $git push 2>&1
    if ($LASTEXITCODE -ne 0) { Log "push 失败（可能令牌过期或需 force）。退出。"; exit 1 }
    Log "推送成功，恢复 stash ..."
    & $git stash pop 2>&1
} elseif ($ahead -gt 0) {
    Log "仅领先：直接推送 ..."
    & $git push 2>&1
    if ($LASTEXITCODE -ne 0) { Log "push 失败。退出。"; exit 1 }
} elseif ($behind -gt 0) {
    Log "仅落后：rebase 到 $upstream ..."
    & $git rebase "$upstream" 2>&1
    if ($LASTEXITCODE -ne 0) { Log "rebase 冲突，请手动处理。退出。"; exit 2 }
    & $git push 2>&1
} else {
    Log "已同步（ahead=0 behind=0），无需操作。"
}

Log "最终状态:"
& $git status -sb 2>&1 | Select-Object -First 30 | ForEach-Object { Log "  $_" }
Log "完成。"