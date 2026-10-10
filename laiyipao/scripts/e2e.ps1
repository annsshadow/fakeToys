# 端到端验收脚本：启动真实服务端 + 真实 PostgreSQL，跑通完整玩家链路。
# 前置：本地 PostgreSQL 15 在 127.0.0.1:5432 可连（trust 认证），用户 postgres。
# 用法：pwsh -File scripts/e2e.ps1
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$serverDir = Join-Path $root 'server'

function Step($msg) { Write-Host "`n=== $msg ===" -ForegroundColor Cyan }
function Ok($msg)   { Write-Host "  OK  $msg" -ForegroundColor Green }
function Bad($msg)  { Write-Host "  !!  $msg" -ForegroundColor Red; $script:failed++ }

$script:failed = 0
$env:GOFLAGS = '-mod=mod'

Step "0. 重建数据库（保证从干净状态验收）"
& psql -h 127.0.0.1 -p 5432 -U postgres -c "DROP DATABASE IF EXISTS laiyipao;" 2>&1 | Out-Null
& psql -h 127.0.0.1 -p 5432 -U postgres -c "CREATE DATABASE laiyipao ENCODING 'UTF8';" 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) { Bad "无法创建数据库"; exit 1 }
Ok "数据库 laiyipao 已重建"

Step "1. 迁移 + 种子数据"
Push-Location $serverDir
& go run ./cmd/seed 2>&1 | Select-Object -Last 8
if ($LASTEXITCODE -ne 0) { Bad "种子写入失败"; Pop-Location; exit 1 }
Pop-Location
Ok "数据库就绪"

Step "2. 编译并启动服务端"
Push-Location $serverDir
& go build -o "$env:TEMP\lyp-server.exe" ./cmd/api
if ($LASTEXITCODE -ne 0) { Bad "编译失败"; Pop-Location; exit 1 }
Pop-Location

$env:ADDR = ':8080'
$proc = Start-Process -FilePath "$env:TEMP\lyp-server.exe" -PassThru `
  -RedirectStandardOutput "$env:TEMP\lyp-api.out" -RedirectStandardError "$env:TEMP\lyp-api.err"
Start-Sleep -Seconds 4

$base = 'http://127.0.0.1:8080'
try {
  $h = Invoke-RestMethod "$base/healthz" -TimeoutSec 8
  Ok "healthz = $($h.ok)"
} catch { Bad "服务端未启动：$($_.Exception.Message)"; Get-Content "$env:TEMP\lyp-api.err"; Stop-Process -Id $proc.Id -Force; exit 1 }

Step "3. 游客登录"
$login = Invoke-RestMethod "$base/api/v1/auth/guest" -Method Post -ContentType 'application/json' `
  -Body '{"guest_token":"","nickname":"验收玩家"}' -TimeoutSec 10
$token = $login.access_token
if (-not $token) { Bad "未拿到 access_token"; Stop-Process -Id $proc.Id -Force; exit 1 }
Ok "user_id=$($login.user.id)  游客=$($login.user.is_guest)"
$hdr = @{ Authorization = "Bearer $token" }

Step "4. 拉取游戏配置（100 关 / 敌人 / 技能 / 反应表）"
$cfg = Invoke-RestMethod "$base/api/v1/config" -TimeoutSec 20
Ok "关卡=$($cfg.levels.Count) 敌人=$($cfg.enemies.Count) 技能=$($cfg.skills.Count) 合成=$($cfg.composite_skills.Count) 反应=$($cfg.reactions.Count) 专精系=$($cfg.mastery_families.Count)"
$terrainLvls = ($cfg.levels | Where-Object { $_.terrain.Count -gt 0 }).Count
Ok "地形关 = $terrainLvls"
if ($cfg.levels.Count -ne 100) { Bad "关卡数不是 100" }
if ($cfg.reactions.Count -ne 7) { Bad "反应链数不是 7" }

Step "5. 钱包与初始进度"
$w = Invoke-RestMethod "$base/api/v1/wallet" -Headers $hdr -TimeoutSec 10
Ok "金币=$($w.coin) 钻石=$($w.gem) 体力=$($w.energy) 钥匙=$($w.keys)"

Step "6. 构筑评分（I-7）"
$me = Invoke-RestMethod "$base/api/v1/me" -Headers $hdr -TimeoutSec 10
Ok "构筑评分=$($me.build_rating.total)  元素覆盖=$($me.build_rating.element_coverage)/5  反应覆盖=$($me.build_rating.reaction_coverage)/7  战力=$($me.power)"
if ($me.build_rating.weaknesses.Count -gt 0) { Ok "短板提示：$($me.build_rating.weaknesses -join ' / ')" }

Step "7. 开局取战斗凭证（扣体力 + 种子）"
$bt = Invoke-RestMethod "$base/api/v1/battle/token" -Method Post -Headers $hdr `
  -ContentType 'application/json' -Body '{"level_id":1}' -TimeoutSec 10
Ok "token_id=$($bt.token_id) seed=$($bt.seed) 波次=$($bt.level.wave_count) 敌人总数=$(($bt.level.waves.spawns | ForEach-Object { $_.count } | Measure-Object -Sum).Sum)"
$w2 = Invoke-RestMethod "$base/api/v1/wallet" -Headers $hdr -TimeoutSec 10
Ok "体力已扣：$($w.energy) → $($w2.energy)"

Step "8. 结算校验 —— 重放同一 token 必须被拒"
# 回放哈希必须真实存在才能测 I-6：用与服务端一致的 FNV-1a 对种子做哈希
# FNV-1a 64 位。用 [System.Numerics.BigInteger] 做模 2^64 运算 ——
# PowerShell 的 uint64 字面量会被解释成有符号 int，0xcbf29ce484222325 直接溢出。
function Fnv1a64Hex([string]$s) {
  $mod = [System.Numerics.BigInteger]::Pow(2, 64)
  $hash = [System.Numerics.BigInteger]::Parse('14695981039346656037')
  $prime = [System.Numerics.BigInteger]::Parse('1099511628211')
  foreach ($b in [System.Text.Encoding]::UTF8.GetBytes($s)) {
    $hash = (($hash -bxor $b) * $prime) % $mod
  }
  return $hash.ToString('x16')
}
$total = 0
foreach ($w in $bt.level.waves) { foreach ($s in $w.spawns) { $total += $s.count } }
$replayHash = Fnv1a64Hex "battle:$($bt.token_id):$($bt.seed):$total"
$settle = @{
  token_id = $bt.token_id; result = 'win'; score = $bt.level.star_targets[0]
  kills = $total; wave_reached = $bt.level.wave_count; duration_ms = 120000
  shots = 400; hits = 320; reactions = 90; heat_max = 88; hp_left = 500
  elements_used = @{ fire = 40; ice = 30; lightning = 25; corrosion = 10; kinetic = 12 }
  reactions_used = @{ steam_burst = 18; overheat = 12; superconduct = 15; burn_cloud = 8 }
  replay_hash = $replayHash
}
Ok "本局回放哈希 = $replayHash"
$r1 = Invoke-RestMethod "$base/api/v1/battle/settle" -Method Post -Headers $hdr `
  -ContentType 'application/json' -Body ($settle | ConvertTo-Json -Depth 6) -TimeoutSec 15
Ok "首次结算：胜利=$($r1.win) 星级=$($r1.stars) 分数=$($r1.score) 掉落金币=$($r1.loot.coin)"

try {
  Invoke-RestMethod "$base/api/v1/battle/settle" -Method Post -Headers $hdr `
    -ContentType 'application/json' -Body ($settle | ConvertTo-Json -Depth 6) -TimeoutSec 15 | Out-Null
  Bad "重放同一 token 竟然成功了 —— 防重放失效"
} catch {
  $code = $_.Exception.Response.StatusCode.value__
  Ok "重放被拒 HTTP $code （防重放生效）"
}

Step "9. 超量击杀必须被拒"
$bt2 = Invoke-RestMethod "$base/api/v1/battle/token" -Method Post -Headers $hdr `
  -ContentType 'application/json' -Body '{"level_id":2}' -TimeoutSec 10
$total2 = 0
foreach ($w in $bt2.level.waves) { foreach ($s in $w.spawns) { $total2 += $s.count } }
$cheat = @{
  token_id = $bt2.token_id; result = 'win'; score = 999999999
  kills = ($total2 + 500); wave_reached = $bt2.level.wave_count; duration_ms = 30000
  shots = 100; hits = 100; reactions = 1
}
try {
  Invoke-RestMethod "$base/api/v1/battle/settle" -Method Post -Headers $hdr `
    -ContentType 'application/json' -Body ($cheat | ConvertTo-Json -Depth 5) -TimeoutSec 15 | Out-Null
  Bad "超量击杀竟然通过了 —— 防作弊失效"
} catch {
  Ok "超量击杀被拒 HTTP $($_.Exception.Response.StatusCode.value__)"
}

Step "10. 星级由服务端重算（客户端谎报 3 星）"
# 第 3 关尚未解锁（关卡门禁生效），这里用已解锁的第 2 关验证星级重算
$bt3 = Invoke-RestMethod "$base/api/v1/battle/token" -Method Post -Headers $hdr `
  -ContentType 'application/json' -Body '{"level_id":2}' -TimeoutSec 10
$total3 = 0
foreach ($w in $bt3.level.waves) { foreach ($s in $w.spawns) { $total3 += $s.count } }
$liar = @{
  token_id = $bt3.token_id; result = 'win'; score = $bt3.level.star_targets[0]
  stars = 3; kills = $total3; wave_reached = $bt3.level.wave_count; duration_ms = 60000
  shots = 300; hits = 250; reactions = 40
  replay_hash = (Fnv1a64Hex "battle:$($bt3.token_id):$($bt3.seed):$total3")
}
$r3 = Invoke-RestMethod "$base/api/v1/battle/settle" -Method Post -Headers $hdr `
  -ContentType 'application/json' -Body ($liar | ConvertTo-Json -Depth 5) -TimeoutSec 15
Ok "客户端报 3 星 → 服务端判定 $($r3.stars) 星（已修正=$($r3.clamped)）"
if ($r3.stars -ne 1) { Bad "星级未被正确重算" }

Step "10b. 未解锁关卡必须被拒（关卡门禁）"
try {
  Invoke-RestMethod "$base/api/v1/battle/token" -Method Post -Headers $hdr `
    -ContentType 'application/json' -Body '{"level_id":50}' -TimeoutSec 10 | Out-Null
  Bad "未解锁的第 50 关竟可开战"
} catch { Ok "未解锁关卡被拒 HTTP $($_.Exception.Response.StatusCode.value__)" }

Step "11. 签到 / 任务 / 商城 / 兑换码"
$sign = Invoke-RestMethod "$base/api/v1/signin" -Method Post -Headers $hdr -TimeoutSec 10
Ok "签到第 $($sign.day_index) 天，奖励金币 $($sign.reward.coin)"
$tasks = Invoke-RestMethod "$base/api/v1/tasks?scope=daily" -Headers $hdr -TimeoutSec 10
Ok "每日任务 $($tasks.items.Count) 个：$(($tasks.items | ForEach-Object { $_.name + ' ' + $_.progress + '/' + $_.target }) -join ' | ')"
$shop = Invoke-RestMethod "$base/api/v1/shop" -Headers $hdr -TimeoutSec 10
Ok "商城 $($shop.items.Count) 项"
$rd = Invoke-RestMethod "$base/api/v1/redeem" -Method Post -Headers $hdr `
  -ContentType 'application/json' -Body '{"code":"ELEMENT5"}' -TimeoutSec 10
Ok "兑换码 ELEMENT5 → $($rd.reward.gem) 钻石"

Step "12. 专精点分配约束（每层 4 选 2）"
$m0 = Invoke-RestMethod "$base/api/v1/mastery" -Headers $hdr -TimeoutSec 10
Ok "初始专精点 = $($m0.points)，节点总数 = $($m0.families.nodes.Count)"
$flameNodes = ($m0.families | Where-Object { $_.family -eq 'flame' }).nodes
$try3 = @($flameNodes | Where-Object { $_.layer -eq 1 } | Select-Object -First 3)
# 前两个应成功，第三个必须被拒 —— 这正是"每层 4 选 2"
$accepted = 0
$rejected = 0
foreach ($n in $try3) {
  try {
    Invoke-RestMethod "$base/api/v1/mastery/allocate" -Method Post -Headers $hdr `
      -ContentType 'application/json' -Body "{`"node_id`":$($n.id)}" -TimeoutSec 10 | Out-Null
    $accepted++
  } catch { $rejected++ }
}
if ($accepted -eq 2 -and $rejected -eq 1) {
  Ok "前 2 个成功、第 3 个被拒 —— 每层 4 选 2 生效"
} else {
  Bad "每层 2 个限制行为异常：成功 $accepted / 拒绝 $rejected（预期 2 / 1）"
}

# 换一系（frost）应能正常分配，证明不是把所有请求都拒了
$frostNodes = ($m0.families | Where-Object { $_.family -eq 'frost' }).nodes
$okNode = $frostNodes | Where-Object { $_.layer -eq 1 } | Select-Object -First 1
try {
  Invoke-RestMethod "$base/api/v1/mastery/allocate" -Method Post -Headers $hdr `
    -ContentType 'application/json' -Body "{`"node_id`":$($okNode.id)}" -TimeoutSec 10 | Out-Null
  Ok "换系分配成功（frost / $($okNode.name)）"
} catch { Bad "换系分配被拒：$($_.ErrorDetails.Message)" }

Step "13. 排行榜 / 防线 / 诊断"
$lb = Invoke-RestMethod "$base/api/v1/leaderboard?type=power" -TimeoutSec 10
Ok "战力榜 $($lb.items.Count) 人，榜首 = $($lb.items[0].nickname) ($($lb.items[0].score))"
$def = Invoke-RestMethod "$base/api/v1/defenses" -Headers $hdr -TimeoutSec 10
Ok "防线候选 $($def.candidates.Count) 条（挑战次数上限 $($def.attempt_limit)）"
$save = Invoke-RestMethod "$base/api/v1/defenses/save" -Method Post -Headers $hdr `
  -ContentType 'application/json' -Body '{"name":"验收防线","skills":[1,2,3,4],"equipment":[1,10],"works":["slow_belt","block_wall","tesla_grid"],"shield_hours":24}' -TimeoutSec 15
Ok "防线已保存：id=$($save.defense.id) 战力=$($save.defense.power) 快照哈希=$($save.defense.snapshot_hash)"
$diag = Invoke-RestMethod "$base/api/v1/diagnose?level_id=5&failed_times=3" -Headers $hdr -TimeoutSec 10
Ok "诊断[$($diag.stage)]：$($diag.title)"

Step "14. 管理端登录与看板"
$al = Invoke-RestMethod "$base/api/v1/admin/login" -Method Post -ContentType 'application/json' `
  -Body '{"username":"admin","password":"admin12345"}' -TimeoutSec 10
$ahdr = @{ Authorization = "Bearer $($al.access_token)" }
Ok "管理员登录成功：$($al.admin.username) / $($al.admin.role)"
$dash = Invoke-RestMethod "$base/api/v1/admin/dashboard" -Headers $ahdr -TimeoutSec 20
Ok "看板：用户=$($dash.users.total) 战斗=$($dash.battles.total) 胜率=$([math]::Round($dash.battles.wins / [math]::Max($dash.battles.total,1) * 100, 1))% 漏斗关卡数=$($dash.stage_funnel.Count)"
$lv = Invoke-RestMethod "$base/api/v1/admin/levels" -Headers $ahdr -TimeoutSec 20
Ok "后台关卡列表 $($lv.total) 条"
$upd = Invoke-RestMethod "$base/api/v1/admin/levels/1" -Method Put -Headers $ahdr `
  -ContentType 'application/json' -Body '{"name":"边境哨站 · 验收改名"}' -TimeoutSec 10
Ok "关卡改名成功 → $($upd.level.name)"
$users = Invoke-RestMethod "$base/api/v1/admin/users?limit=5" -Headers $ahdr -TimeoutSec 10
Ok "玩家列表 total=$($users.total)"
$logs = Invoke-RestMethod "$base/api/v1/admin/audit-logs?limit=5" -Headers $ahdr -TimeoutSec 10
Ok "审计日志 $($logs.items.Count) 条（最近：$($logs.items[0].action)）"

Step "15. 验真接口（I-6）"
$battles = Invoke-RestMethod "$base/api/v1/admin/battles?limit=1" -Headers $ahdr -TimeoutSec 10
if ($battles.items.Count -gt 0) {
  $bid = $battles.items[0].id
  $replay = Invoke-RestMethod "$base/api/v1/battle/$bid/replay" -Headers $hdr -TimeoutSec 10
  $eh = $replay.expected_hash
  if ([string]::IsNullOrEmpty($eh)) { $eh = '0' * 16 }
  Ok "取到复现信息：battle=$bid level=$($replay.level_id) seed=$($replay.seed) 期望哈希=$eh"
  # 用对象构造请求体再序列化，避免字符串插值把空哈希变成非法 JSON
  $bodyOk   = @{ battle_id = $bid; replay_hash = $eh } | ConvertTo-Json
  $bodyBad  = @{ battle_id = $bid; replay_hash = ('0' * 16) } | ConvertTo-Json
  $v  = Invoke-RestMethod "$base/api/v1/battle/verify" -Method Post -Headers $hdr `
        -ContentType 'application/json' -Body $bodyOk -TimeoutSec 10
  Ok "提交正确哈希 → matched=$($v.matched)"
  if (-not $v.matched) { Bad "正确哈希竟未通过验真" }
  $v2 = Invoke-RestMethod "$base/api/v1/battle/verify" -Method Post -Headers $hdr `
        -ContentType 'application/json' -Body $bodyBad -TimeoutSec 10
  Ok "提交错误哈希 → matched=$($v2.matched)（成功证伪）"
  if ($v2.matched) { Bad "错误哈希竟然验真通过" }
}

Step "16. 技能升级（消耗金币 → 等级 +1 → 进 build 快照）"
# ⚠️ 这一步是**唯一**能覆盖 `UpgradeSkill` 事务逻辑的地方。
# Go 单元测试碰不到 SQL：FOR UPDATE、条件扣费（`WHERE coin >= $3`）、
# 满级短路、未拥有拒绝 —— 四条都只能在这里验。
#
# 判据按「行为 > 反射」：不比「level 字段变成 2」，
# 而是比**扣了多少金币** + **重放哈希是否随之改变**。
# 只比字段的话，一个「只加字段不扣钱」的 bug 也能通过。
$wBefore = Invoke-RestMethod "$base/api/v1/wallet" -Headers $hdr -TimeoutSec 10
$costExpect = 100   # CostFrom(1) = baseCost × 当前等级 = 100
try {
  $up = Invoke-RestMethod "$base/api/v1/me/skills/1/upgrade" -Method Post -Headers $hdr -TimeoutSec 10
  Ok "技能 1 升级 → level=$($up.level)，余额 $($wBefore.coin) → $($up.wallet.coin)"
  if ($up.level -ne 2) { Bad "升级后期望 level=2，实得 $($up.level)" }
  $spent = $wBefore.coin - $up.wallet.coin
  if ($spent -ne $costExpect) { Bad "扣费期望 $costExpect，实扣 $spent" }

  # 等级必须真的进 build 快照 —— 否则客户端拿到的还是 1 级
  $bt2 = Invoke-RestMethod "$base/api/v1/battle/token" -Method Post -Headers $hdr `
    -ContentType 'application/json' -Body '{"level_id":1}' -TimeoutSec 10
  $lv1 = $bt2.build.skills.'1'
  if ($null -eq $lv1) {
    Bad "build 快照里找不到技能 1"
  } elseif ($lv1.level -ne 2) {
    Bad "build 快照里技能 1 的 level=$($lv1.level)，期望 2（等级没进快照 = 客户端算不出伤害）"
  } else {
    Ok "build 快照已带等级：skill 1 level=$($lv1.level)（客户端按它缩放伤害）"
  }

  # 未拥有的技能必须被拒（升级别人的/不存在的技能 = 白送一级）
  try {
    Invoke-RestMethod "$base/api/v1/me/skills/99999/upgrade" -Method Post -Headers $hdr -TimeoutSec 10 | Out-Null
    Bad "升级未拥有的技能 99999 竟然成功"
  } catch { Ok "升级未拥有的技能被拒 HTTP $($_.Exception.Response.StatusCode.value__)" }
} catch {
  Bad "技能升级失败：$($_.ErrorDetails.Message)"
}

Step "17. 未认证访问必须被拒"
try {
  Invoke-RestMethod "$base/api/v1/me" -TimeoutSec 10 | Out-Null
  Bad "未带令牌竟能访问 /me"
} catch { Ok "未认证被拒 HTTP $($_.Exception.Response.StatusCode.value__)" }

Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue

Write-Host ""
if ($script:failed -eq 0) {
  Write-Host "===== 端到端验收全部通过 =====" -ForegroundColor Green
  exit 0
} else {
  Write-Host "===== 失败 $script:failed 项 =====" -ForegroundColor Red
  exit 1
}
