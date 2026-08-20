# commit_batch_oa4rust.ps1
# 用途：在真机一键分批提交 oa4rust 工作区修改（两批）：
#   批次 1: oa4rust 测试代码（tests_generated 扩充 + preview/realtime/search/signature/sms 五个新 crate 挂载）
#   批次 2: docs 计划与脑暴文档
# 安全说明：不做 reset --hard、不 git add -A、不盲提交；
#           提交前检查 HEAD 可解析性与待提交内容，无内容则跳过该批次。
# 用法：在仓库根目录打开 PowerShell，执行  .\commit_batch_oa4rust.ps1
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
    Log "错误：当前不是 git 仓库，或 .git 不可用。请在仓库根目录运行。退出。"
    exit 1
}

# 1) HEAD 检查（不可解析时给出指引，不盲提交，防止产生全量根提交）
$head = & $git rev-parse --verify HEAD 2>&1
if ($head -notmatch '^[0-9a-f]{40}$') {
    Log "警告：HEAD 不可解析（分支 ref 丢失或损坏）。"
    Log "请先运行修复脚本恢复仓库（recover_repo_sync.ps1 或 recover_repo.ps1），再重新运行本脚本。"
    Log "（本地未推送提交可能仍以 dangling 对象形式存在于对象库，git fsck --lost-found 可找回）"
    exit 2
}
$branch = & $git rev-parse --abbrev-ref HEAD 2>&1
Log "当前分支: $branch ($head)"

# 2) 清理 index 污染（mixed reset 只对齐 index 到 HEAD，不动工作树文件）
& $git reset --mixed HEAD 2>&1 | Out-Null
Log "已重置 index 对齐 HEAD（mixed，不影响工作树文件）"

# 3) 批次 1：oa4rust 测试代码
$oa4rustChanged = (& $git status --porcelain -- oa4rust/ 2>&1 | Measure-Object).Count
if ($oa4rustChanged -gt 0) {
    Log "批次 1：提交 oa4rust 修改（$oa4rustChanged 项）..."
    & $git add oa4rust/ 2>&1 | Out-Null
    & $git commit -m "feat(oa4rust): 补齐 handler 级测试并挂载 preview/realtime/search/signature/sms 的 tests_generated 模块" 2>&1
    if ($LASTEXITCODE -eq 0) { Log "  批次 1 提交成功" }
    else { Log "  批次 1 提交失败（可能无实际差异），继续批次 2" }
} else {
    Log "批次 1：oa4rust 无待提交修改，跳过"
}

# 4) 批次 2：docs 文档
$docsChanged = (& $git status --porcelain -- docs/brainstorms/2026-08-20-oa4rust-remaining-gap-closure-requirements.md docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md 2>&1 | Measure-Object).Count
if ($docsChanged -gt 0) {
    Log "批次 2：提交 docs 文档（$docsChanged 项）..."
    & $git add docs/brainstorms/2026-08-20-oa4rust-remaining-gap-closure-requirements.md docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md 2>&1 | Out-Null
    & $git commit -m "docs(oa4rust): 新增剩余缺口补全计划与需求脑暴文档" 2>&1
    if ($LASTEXITCODE -eq 0) { Log "  批次 2 提交成功" }
    else { Log "  批次 2 提交失败" }
} else {
    Log "批次 2：docs 无待提交修改，跳过"
}

# 5) 验证
Log "最近 4 条提交："
& $git log --oneline -4 2>&1 | ForEach-Object { Log "  $_" }
Log "剩余工作区状态："
& $git status -sb 2>&1 | Select-Object -First 15 | ForEach-Object { Log "  $_" }
Log "完成。"
