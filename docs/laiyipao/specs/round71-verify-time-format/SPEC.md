# R71 规范：验真页「记录时间」本地化（test-first RED）

## §0 背景与候选来源
- 来源：R70-A 只读侦察 docs/laiyipao/recon/ROUND70_A_TIME_NUMBER_RECON.md 候选A（四条件全中）。
- 缺口：laiyipao/miniapp/src/pages/verify/verify.vue:101 模板 `<view class="kv"><text class="muted">记录时间</text><text class="mono">{{ replayInfo.created_at }}</text></view>` 把后端 UTC RFC3339 字符串（如 2026-11-27T01:23:45Z）裸直出，玩家需自行换算时区，与同仓其他页格式化口径不一致。
- 证据：真 Chromium 390×844 / 768×1024 / 1280×800 三视口 XHR-patch mock 坐实 DOM=`记录时间2026-11-27T01:23:45Z`，errText=null，hOverflow=0（docs/laiyipao/audits/round70-a-time-number/）。
- 后端格式：laiyipao/server/internal/service/adminapi.go:690/:990、adminuserdetail.go:437、recompute_gate.go:834 共 4 处 createdAt.UTC().Format(time.RFC3339)。
- 数据来源：verify.vue:269 replayInfo=ref<ReplayInfo|null>、:452 replayInfo.value=await api.getReplay(id)；ReplayInfo laiyipao/miniapp/src/game/replay.ts:35-42（:42 created_at:string）；api/client.ts:1080-1082 getReplay。

## §1 权威口径与修复决策
- 权威 helper 三处逐字一致（new Date(iso) → Number.isNaN(d.getTime()) 回退原串 → 本地日历字段 → padStart(2,0)）：
  - laiyipao/miniapp/src/pages/me/me.vue:877-886 fmtAccountTime：输出 `y-mm-dd hh:mi`。
  - laiyipao/miniapp/src/pages/announcements/announcements.vue:187-197 formatDate(iso:string|null)：输出 `y-mm-dd hh:mi`，:178-185 monthKey；announcements_month_group.test.ts 覆盖跨时区。
  - laiyipao/miniapp/src/pages/battles/battles.vue:810-818 fmtTime：输出 `mm/dd hh:mi`（同为本地日历+NaN 兜底口径）。
- 决策：在 verify.vue 内新增本地函数 fmtVerifyTime(iso: string): string，逐字采用 y-mm-dd hh:mi 口径（与 me.fmtAccountTime / announcements.formatDate 一致）；非法 ISO 回退原串，保证诚实展示不丢数据。
- 不复用跨页 helper：三处权威实现均为页面内本地函数、无共享 helper 可 import；照抄范式保持仓库惯例。

## §2 精确改点
- 唯一模板改点：laiyipao/miniapp/src/pages/verify/verify.vue:101
  - 改前：`<view class="kv"><text class="muted">记录时间</text><text class="mono">{{ replayInfo.created_at }}</text></view>`
  - 改后：`<view class="kv"><text class="muted">记录时间</text><text class="mono">{{ fmtVerifyTime(replayInfo.created_at) }}</text></view>`
  - 保留：kv 容器 class="kv"、muted label「记录时间」、mono class、与 :102-105 哈希行同构。
- 新增函数：同页格式化函数区新增 function fmtVerifyTime(iso: string): string，逐字复刻 me.vue:877-886（y-mm-dd hh:mi + Number.isNaN 回退）。
- 写域边界：仅 verify.vue；禁改 engine/replay/hash/score/settle/loot/tokens 与后端；禁改令牌；禁新增 r71 视觉令牌。

## §3 不变量（回归必须保持）
- fetchReplay（verify.vue:436-462）：const fetching=ref(false)、:440 if(fetching.value)return、:450 fetching=true、:451-452 try replayInfo.value=await api.getReplay(id)、:453-458 catch 置 errMsg + outcome.value=null（P2-11 失败清结论）、:459-461 finally fetching.value=false。
- 哈希展示：:104 <text class="mono hash">{{ pretty(replayInfo.expected_hash) }}</text> 保留。
- 快照卡片：:95 v-for="s in buildSnapshotCard" class="bs-item" 保留。
- r5AiVerify 本地复核逻辑保留。
- 令牌三锚点：styles/tokens.css:14 --lyp-token-version=lyp-tokens-v1、:15 --lyp-r3-version=lyp-tokens-v3-r3、:16 --lyp-r6-version=lyp-tokens-r6 保留。
- 业务源码无 --r71- 新增令牌。

## §4 契约清单
- laiyipao/miniapp/src/styles/r71VerifyTimeFormat.contract.test.ts（5 it，真实 readFileSync 读源码，禁内嵌实现副本）：
  - ① fmtVerifyTime 存在且含 new Date / Number.isNaN(d.getTime()) / return iso / 本地日历字段。
  - ② 记录时间行绑定 fmtVerifyTime(replayInfo.created_at) 且不含裸 {{ replayInfo.created_at }}。
  - ③ 记录时间行 kv 结构（class="kv" + muted「记录时间」+ mono）保留。
  - ④ fetchReplay 守卫/finally/清结论 + pretty(expected_hash) + buildSnapshotCard 不变量保留。
  - ⑤ 令牌三锚点保留且业务源码无 r71 视觉令牌。
- 范式：r66GemCacheGuard.contract.test.ts 的 blockAfter 大括号配对 + readSrc(join(__dirname,'..'))。

## §5 RED 预期
- 当前（裸直出）运行：Test Files 1 failed(1)，Tests 2 failed | 3 passed (5)。
- 失败项为缺口断言：①（fmtVerifyTime 不存在）与 ②（模板未绑定 fmtVerifyTime）；③④⑤ 为不变量，通过。
- 无解析错误 / 模块加载错误 / 超时；若出现属契约自身缺陷须修。

## §6 验证命令
- RED：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r71VerifyTimeFormat.contract.test.ts --reporter=verbose
- GREEN（实施后）：同上命令应 exit=0，Tests 5 passed (5)。
- 全门禁（实施阶段）：pnpm exec vitest run --reporter=verbose、pnpm exec vue-tsc --noEmit、pnpm run build:h5、git diff --check 全 RC0。

## §7 诚实边界
- 本轮只写规范 + test-first RED 契约，不改业务源码；GREEN 由后续实施任务完成。
- mock created_at=2026-11-27T01:23:45Z 为合成值，无真值；auth 四路径 ABSENT，未落盘任何凭据。
- 未写 git，未改 .gitignore，未引入 r71 业务令牌。
