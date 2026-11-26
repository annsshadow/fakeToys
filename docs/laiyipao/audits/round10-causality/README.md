# R10 养成前后因果视图与宝石佩戴收益（t101）

> 证据环境：本地回环 H5 真浏览器（agent-browser / Chromium，视口 390×844），API :8080。**非物理真机**，桌面浏览器模拟移动视口。未写入真实域名/IP/口令/私钥；未执行 git。
> 证据目录：`docs/laiyipao/audits/round10-causality/`。采集脚本 `capture-after.sh` / `recapture-gems.sh` / `capture-final.sh` / `diag-native.sh` / `fetch-battles-api.sh`。
> 会话说明：空态基线用 `lyp-r10c`；决定性实验（同账号两局真实结算）用 `lyp-r10final`。agent-browser 不同 session 是独立浏览器上下文（不同游客账号），跨 session 战报不互通——「共 1 场」假象的根因。

## 1. 三个缺口的落地

1. **养成前后可信差值（缺口②）**：`battles.vue` 战报详情展开区、`battle.vue` 结算面板新增 `.r10-growth` 卡。只认同 `level_id` 相邻两次真实结算（各带 `created_at`/`replay_hash`），对 `score/stars/kills/leaked/reactions/hp_left/wave_reached` 做确定性减法；缺一次结算显示「还缺养成前那局战报，先打一局同关 · 构筑变更点未上报，不做估算」。构筑变更点未上报，故不推算养成归因，不编造提升%。
2. **宝石佩戴收益（缺口③）**：`bag.vue` 佩戴/卸下真实写前后各取一次服务端 `/me` 的 `build.attacker` 权威快照，确定性差分后按 `user_id + gem_id` 缓存到本机 localStorage（缓存键不渲染到玩家可见文案）。`gems.vue` 图鉴读缓存展示 `.r10-gem-gain` 卡；无缓存显示「佩戴收益未上报，先看词条描述」，不把 `/config` 静态词条说明当已生效收益。
3. **推荐收益不可用的真实依据（captain 追加）**：`skills.vue`/`equipment.vue` 无 `id`/`replace` 候选时，错误位消费推荐卡 `insights[].basis`，显示服务端「为什么收益不可用」的依据，替代干瘪的「接口未提供真实预估」；不伪造 impact 参数、不放宽服务端写入校验。

## 2. RED → GREEN

- 新契约 `src/styles/r10GrowthGaps.contract.test.ts`（9 项文件/helper 检查 + 复用 battle.test.ts 58 项）。首轮 RED 8 failed|1 passed。
- 修正契约自身两处误判：`tpl()` 改用 `lastIndexOf('</template>')`（battle 模板内有嵌套 `<template>`）；script 层标记（如 `loadGrowthCompare`）改为检查整文件而非模板切片。
- 修正「渲染正确但源码不含字面」：动态插值文案同时在模板写静态字面（诚实空态两条），使契约可钉住真实可渲染文案，而非弱化断言。
- 截图暴露真实 bug 并修复：`gems.vue` 的 `.r10-basis-note` 曾把缓存键 `lyp:r10:gem-gain-<user_id>:0` 渲染给玩家，改为 HTML 注释 + 不含 id 的诚实说明；recapture 后目视干净。
- GREEN：契约 9/9 + battle.test 58/58；全量 `pnpm test` 182 files passed|1 skipped、1717 tests passed|2 skipped；`pnpm type-check` exit 0；`pnpm build:h5` DONE。

## 3. 截图证据（390×844，非物理真机，本地回环）

| 截图 | sha256 | 观察 |
| --- | --- | --- |
| `screenshots/after-01-home.png` | `a6ce10f0e8beacba5d0fd3eb4380b1b00b6cd9c911e53403b5caf85d4730eaba` | 首页首屏未回退 |
| `screenshots/after-02-battles.png` | `5d8bd9d19124492268eafdc115f8901c5dc4e79c34a272d0ce3f5b66f6a6a45c` | 空态会话 `lyp-r10c`：共 0 场，无同关两次结算故不显示差值（诚实空态） |
| `screenshots/after-03-gems.png` | `1d2fac07b4fbe9b996fb59c2a33b29a27077d699d58ef74d71a52395acad5fa0` | AI 因果卡 + 佩戴收益诚实空态 + 去商城/去背包双 CTA；无缓存键泄露 |
| `screenshots/after-04-skills.png` | `80d5796a68289e158eefa91efcf2015f346f6fd91e69262dd0f11e18fb6bc64b` | NO_HISTORY 空态 + AI 因果卡 + 「当前 vs 推荐」收益卡 + 服务端依据 |
| `screenshots/after-05-equipment.png` | `58da72e66584ad2decc57128a600e30a7d442788cb7d541bbac3d928af7921ec` | 同上（装备页） |
| `screenshots/after-06-bag.png` | `f0b11761e15fe2839173510934ff3e0778a26073847ec458800dc07a98e4e421` | 背包页签未回退 |
| `screenshots/after-07-settle-growth.png` | `abcc812cedc7afc942041bb176a3eb28c752d46d4ca2ec545ae271ae91bdaf20` | 决定性实验第一局结算：13120 分 / B / 85 反应 / hash `7d18a4ed…`；此刻只有一局，差值卡正确显示诚实空态 |
| `screenshots/after-08-settle-second.png` | `4ab2e40a625a90a622c14a342ccaa457308024e0e83899bd20277468ddd7de1d` | 同账号第二局结算：10614 分 / C / 71 反应 / hash `bd49af56…`；**差值卡显示有数据态**：分数 13120→10614（-2506）、星级 1→0、反应 85→71、两 hash 并列 |
| `screenshots/after-09-battles-growth.png` | `b37ed1616a55be64ca0bd51e0e860d1aecc6da7fccb010e75e5e1a29cb7f578f` | 战报详情页展开第一局：同一差值卡有数据态，DOM 探针 `growth-native.txt` 记录完整文本 |

## 4. 决定性实验：同关两次真实结算 → 差值卡有数据态（队长复核要求）

**结论：情况③——功能本可达，之前是抓图时序错位，未改任何业务逻辑。**

- 实验方法：同一 agent-browser session `lyp-r10final`（同一游客账号），`open` 第 1 关战斗 → 自动打完（8×、点「直接下一波」/手牌/「继续战斗」）到结算；用 `reload` 强制重开第二局（同 URL `open` 不会重新加载页面，这正是旧 attempt 两局 hash 相同的原因）；两局结算 hash 不同（`capture-final.sh` 输出 `DISTINCT_HASHES_OK`、`HAS_TWO=true`）。
- 展开详情必须用 agent-browser 原生 `click '.battle-row'`；JS `el.click()` 不触发 uni-app 的 `@click`（`diag-growth.sh` 得 `EXPANDED=0`，`diag-native.sh` 得 `EXPANDED=1`）。这是采集脚本问题，不是产品缺陷。
- API 原始证据 `battles-api-evidence.txt`（浏览器内带会话 token 请求 `GET /api/v1/me/battles?limit=20`，token 未落盘/未回显）：`httpStatus=200`、`count=2`，两条记录同为 `level_id=1` 且 `created_at` 为有效时间：
  - id=1983，hash `bd49af569c6ea194`，created_at `2026-11-22T22:47:27+08:00`，score 10614，kills 39；
  - id=1982，hash `7d18a4ed3fd7b40d`，created_at `2026-11-22T22:46:11+08:00`，score 13120，kills 39。
- 差值卡 DOM 探针 `growth-native.txt`：`同一关第 1 关 · 两次真实结算的可信差值（构筑变更点未上报）`，分数/星级/击杀/漏怪/反应数/剩余血量/到达波次 7 行 `before → after / delta`，`依据：两次结算 replay_hash：7d18a4ed… / bd49af56… · 同关同难度 · 构筑变更点未上报`。数值与 API 两条记录逐项一致，非编造。
- 时序说明（解释 captain 亲读到的旧图）：旧 attempt 的 after-07/08（sha `1fbed79a…`/`0f14c0e0…`）属跨 session 采集，两图同为 9729 分/hash `6275a60…`，after-09「共 1 场」。本轮 capture-final.sh 已用同一 session 重采并覆盖同名文件（新 sha 见第 3 节），第三张 after-09 差值卡有数据态。
