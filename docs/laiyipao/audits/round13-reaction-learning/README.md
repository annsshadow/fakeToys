# R13 玩法实测：元素反应教学性与术语一致性

> 环境：本地回环 H5 + 本地回环 API，真实 Chrome 浏览器自动化点击；**非物理真机**。
> 证据截图均位于 `screenshots/`，不写真域名、真 IP、口令或令牌。

## 1. 实测链路与方法

从新玩家视角依次走：反应图鉴 → 战斗 HUD 因果链 → 战报复盘。
浏览器 CHOKIDAR 轮询 H5 服务，API 从本地回环读取 `/api/v1/config` 与 `/api/v1/me`。
为触发“电+毒=腐蚀扩散”，在测试库把腐蚀弹加入测试账号技能表并把出战槽临时改为 `[7,16,0,0]`；实测结束后恢复 `[1,4,7,0]`。这些是本地测试数据，不涉及仓库真值。

## 2. 实测发现的真实问题

### D1 图鉴把“层数传播半径”写成了“伤害溅射半径”

首屏图鉴 DOM 逐字：腐蚀扩散说明为“造成反应伤害，并把 1 层腐蚀传播给半径 120 像素内的其他敌人（传播本身不触发新反应）”，但其统计标签是“溅射半径 120”；机制速查表头只有“驱散/削甲/击退/增幅/持续”，腐蚀扩散整行全点。

修复前的错误输出由下方 RED 测试的逐字断言固化（旧逻辑输出「密集波 25% 溅射（半径 120）」）。诚实说明：原采集的 `r13-01-reactions-codex.png` 与修复后图 `r13-02-reactions-fixed.png` 的 sha256 完全相同（均 92f6497f…），字节一致、不构成独立的前后对比证据，该重复图已删除，此处不谎称存在独立的「修复前截图」；修复后状态以 DOM 探针与 HUD 因果链截图 `r13-03-corrosion-cue.png`（sha d529a535…）为证。

根因：
- `laiyipao/miniapp/src/game/feedback.ts` 旧逻辑把 `aoeRadius > 0` 一律解释成“密集波 25% 溅射”，而 `reaction_descr.test.ts:116-119` 明确腐蚀扩散的 `aoeRadius` 是“层数传播半径”，消费者是 `triggerCorrosionSpread` 而不是伤害溅射。
- `laiyipao/miniapp/src/pages/reactions/reactions.vue` 机制表漏“传播/溅射”列，且半径标签硬写“溅射半径”。

RED：三文件共 4 failed / 20 passed。逐字：
`expected '电+毒触发腐蚀扩散，密集波 25% 溅射（半径 120）；依据 react…' to contain '传播 1 层腐蚀'`。

GREEN：
- `feedback.ts` 新增 `reactionMechanism()`，按 `spreadStacks/amplifyPct/statusDurationMs/aoeRadius` 组合出与图鉴同源的机制短语；`spreadStacks>0` 只输出“传播 N 层腐蚀（半径 X，不触发新反应）”，绝不输出 25% 溅射。
- `reactions.vue` 机制表加“传播/溅射”两列；腐蚀扩散显示“传播半径 120”。

真浏览器复测 DOM：`腐蚀扩散 当前不可达 电 + 毒 毒 + 电 ... 层数传播 1`，机制表头 `反应 驱散 削甲 击退 增幅 持续 传播 溅射`。截图：`screenshots/r13-02-reactions-fixed.png`（非物理真机/本地回环）。

### D2 战报只列“反应 ×次数”，答不出“下一局换什么元素”

旧战报 `reactionText` 仅输出 `过热 ×43、蒸汽爆发 ×40、超导 ×22`，玩家没有可执行下一步。

RED：`battles.test.ts` 断言 `.reaction-learning` 不存在；逐字 `expected false to be true`。

GREEN：
- `feedback.ts` 新增 `reactionLearningAdvice({reactionsUsed,elementsUsed})`，只消费服务端权威 `reactions_used/elements_used`；缺数据返回“本局未上报反应使用记录，不给元素建议。”，绝不推算。
- `battles.vue` 新增 `.reaction-learning`，以最高次数反应为主，找一个“已有一种元素、只缺另一种”的可解锁反应给下一动作。

真浏览器复测 DOM 逐字：`AI · 反应搭配建议 反应复盘：焰+电触发过热 ×43；下一局补 毒（已有 焰），可解锁燃烧云（焰+毒）；依据：reactions_used/elements_used。`。截图：`screenshots/r13-05-report-learning.png`（非物理真机/本地回环）。

### D3 战斗中 HUD 因果链与图鉴术语一致性验证

用 `[7,16,0,0]` 电+腐蚀构筑进第 1 关，8× 后 HUD 逐字：
`电+毒 → 腐蚀扩散 -171 AI 电+毒触发腐蚀扩散，传播 1 层腐蚀（半径 120，不触发新反应）；依据 reaction=corrosion_spread/damage=171；点卡凑同元素`。
这与图鉴修复后的说明完全一致，说明 D1 修复已跨图鉴/HUD 双端生效。截图：`screenshots/r13-03-corrosion-cue.png`（非物理真机/本地回环）。

## 3. 跨端缺陷（写域外，已定位根因）

该局 127 次反应、39 杀、0 漏，结算被服务端拒绝：`settle rejected: 命中率越界：命中 165 > 发射 160`（截图 `screenshots/r13-04-result.png`）。

根因在 `laiyipao/miniapp/src/game/engine.ts`：
- `:1406` 每次开火 `this.shots++`；
- `:1515-1522` `checkProjectileHit()` 遍历敌人，每命中一个敌人就 `this.hits++` 并加入 `p.hitSet`。注释 `:1526-1532` 声称 hits 语义是“开火命中次数”、不应按多目标累加，但穿透弹丸会在同一次开火中命中多个敌人，导致 `hits > shots`。
- 服务端 `laiyipao/server/internal/domain/battle.go:512-513` 明确拒绝 `Hits > Shots`。

本任务按约束不改 `engine.ts`，交队长派发内核/后端双端修复：要么 hits 按弹丸而非按敌人计，要么服务端校验口径同步。客户端结算兜底已显示“战绩未入账”，未谎报入账。

## 4. 验证

- RED：`npx vitest run --config vitest.config.ts src/game/feedback.test.ts src/pages/reactions/reactions.test.ts src/pages/battles/battles.test.ts --reporter=verbose` → exit 1，4 failed / 20 passed。
- GREEN 定向：同命令 → exit 0，3 files passed / 24 tests passed。
- 全量：`npm test` → exit 0，Test Files 184 passed | 1 skipped (185)，Tests 1743 passed | 2 skipped (1745)。
- 类型：`npm run type-check`（vue-tsc）exit 0；`npm run type-check:test`（tsc）exit 0。
- 浏览器 errors：全程 `errors=[]`。
- 未执行 git；未写入真域名、真 IP、口令、令牌或私钥。

## 5. 反应系统可学习性 Top3 真实缺口（按优先级）

1. **穿透/溅射命中记账导致高反应局结算被拒**（最高优先级）：真实电+腐蚀局 127 反应却被 `命中 165 > 发射 160` 拒绝。代码证据：`engine.ts:1406` vs `:1515-1522`，`battle.go:512-513`。这直接打断“学会反应→实战验证→战报复盘”的完整闭环。
2. **图鉴的可执行建议仍不直达具体技能**：图鉴已有“去调整出战技能 →”，但不告诉玩家该换哪个槽位；战报已能算出“下一局补 毒”，但未给出可点的一键替换。代码证据：`reactions.vue` 的 CTA 仅跳 `/pages/skills/skills`，`feedback.ts:reactionLearningAdvice` 只返回文本。
3. **战报反应归因缺波次/敌人维度**：当前权威输入只有 `reactions_used/elements_used`，无法回答“第几波、哪种敌人触发最多反应”。代码证据：`feedback.ts:ReactionLearningInput` 字段集合；`engine.ts` 虽有 per-event reaction，但未持久化到战报输入。
