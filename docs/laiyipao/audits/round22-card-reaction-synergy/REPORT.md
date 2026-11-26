# R22 复查报告 · 选卡协同预判真浏览器实测

> 判定：**PASS**（无阻断缺陷）  
> 任务：t179 / attempt 2（qa-e2e-wb）  
> 日期：2026-11-24  
> 浏览器：agent-browser（**非物理真机**），视口 390×844，本地 Vite H5 服务。

## 一、复查口径与依据

- 规范：`docs/laiyipao/specs/round22-card-reaction-synergy/SPEC.md`（80 行）。核心契约：预判为纯客户端展示线索，**不落库、不进 replay_hash、不改结算、不参与战斗判定**（SPEC 第 4/15/24/59/60/79 行）。
- 真源：`laiyipao/miniapp/src/game/elements.ts` 的 `lookupReaction` / `REACTION_ORDER`；`laiyipao/miniapp/src/utils/reactionPairs.ts:15-26` 由引擎真源两两穷举派生 `REACTION_PAIRS`（纯静态数据，不落库不进哈希）。实测 fire+ice=steam_burst、ice+lightning=superconduct、fire+lightning=overheat。
- 算法：覆盖集 C(E) 与差集 A=C(E∪{e})−C(E)，按 REACTION_ORDER 排序，反应名只取 `REACTIONS[key].name`。

## 二、真浏览器实测（有数据态）

- 用户 10（技能槽 fire/ice/lightning 齐全），`http://127.0.0.1:5174/#/pages/battle/battle?level=5`，走到选卡阶段（handCount=3）。
- DOM 探针 `data/probe-hand.json`：
  - 卡 0「电弧弹」(cardElement=电)：`AI · 强化已有【蒸汽爆发】`，三标签「蒸汽爆发/过热/超导」——与已装备元素集 {焰,冰,电} 的差集推导一致。
  - 卡 1「散热涂层」、卡 2「连锁导引」：无元素卡逐字空态「该卡无元素，无法预判反应（属性/机制卡）」。
  - AI 卡 `note`：role=note，含可见「AI · 协同预判」、真实结论、逐字依据行「依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生」、CTA「去装备/选它」、诚实行逐字「辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准」。`hasAI/hasBasis/hasCTA/hasHonesty` 全 true。
  - `hasUndefined=false, hasNaN=false`，探针 `pageErrors/consoleErrors/apiFailures/failedRequests` 全空数组。
- 点选一张卡后 `data/probe-after-card.json`：handCount=0、cards=[]、note 消失（随手牌正确消失），探针仍全 0。

## 三、空态诚实与限制

- 手牌无元素卡空态：浏览器内已实测（见上「散热涂层/连锁导引」逐字文案），符合 SPEC「无元素卡不预判」。
- 「当前构筑还没有元素技能」空态：QA 库用户 127 的 `user_skill_slots` 无火/冰/电技能，真浏览器仍显示元素预判——根因 `equippedFromSnapshot(bt.build, skills)` 取服务端 `/battle/start` 下发的 build 快照，该快照仍带元素；故浏览器内未复现该空态。此分支由定向单测 `battle_r22_synergy.test.ts`（FakeEngine + mock `equippedFromSnapshot`）覆盖并 PASS。**如实标注此浏览器验证限制，不伪装已实测该分支。**

## 四、replay_hash 一致对照（静态 + 动态）

- 静态：`battle.vue:1222-1226` `takeCard(id)` 仅调 `engine.value.takeCard(id)`、刷新 `hand`、`cardChoiceVersion++`，完全不触 hash；`battle.vue:1152` settle 的 `payload.replay_hash` 来自 `eng.settleInput()`/`eng.replayHash()`；`settleResult.replay_hash = payload.replay_hash`。浏览器 window 内读不到引擎实例（keys 仅 `ongamepadconnected/ongamepaddisconnected`），故无法在页面内直接前后读 hash。
- 动态：推进一局至真实结算，`data/probe-settle.json` 记录 settle 响应 HTTP200 / win=true / battle_id=477；结算页 `.hash-note` 显示「回放哈希 1dbf94aebb62f81d（可验真）」，R10 对照卡显示两次结算 replay hash（历史 7468252a57f6befe → 本局 1dbf94aebb62f81d，逐局独立）。证明权威哈希由引擎/结算产生，选卡预判从未介入。

## 五、AI 四判据与诚实红线

- 四判据：①可见汉字「AI」（AI·协同预判 / AI 卡标题）；②真实数据结论（电弧弹强化蒸汽爆发，由本局 equipped×手牌真实派生）；③逐字依据行；④可执行 CTA「去装备/选它」。
- 诚实红线：逐字「辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准」。tokens.css 锚点 v1/v3-r3/r6 保留，无 `--r22-` 新令牌；R15–R21 契约未动；零后端/结算改动。

## 六、测试与 RED 基线

- 定向：`npx vitest run --config vitest.config.ts src/pages/battle/battle_r22_synergy.contract.test.ts src/pages/battle/battle_r22_synergy.test.ts` → 2 files / 15 tests ALL PASSED。
- 全量：`npx vitest run --config vitest.config.ts` → 200 passed / 1 skipped files，1825 passed / 2 skipped tests。`npx vue-tsc --noEmit` exit 0。
- RED 基线：`git show HEAD:laiyipao/miniapp/src/pages/battle/battle.vue | grep -c r22-synergy = 0`；两份 R22 测试 NOT_IN_HEAD（test-first 证据）。

## 七、截图证据（均亲读、非登录页、sha256 两两不同）

| 截图 | sha256 | 内容 |
| --- | --- | --- |
| r22-01-hand-synergy.png | 57fc52dbac8c5812d33131b491572cc5bd105413de0d446ae66aaa4fcb008f20 | 战斗选卡页：电弧弹 AI 强化蒸汽爆发+3 标签；2 张无元素卡空态；底部 AI 协同卡 |
| r22-02-synergy-card.png | 67d7eb6887372fedefd515b967563a981f6924b2327b69ac3d7b4abd01209ca8 | 协同卡面特写 |
| r22-03-ai-note.png | 19856b2f0c4b053cfb80d34616c4735f180badbac7316cc3d6523815659846c1 | AI 卡依据/CTA/诚实行 |
| r22-04-after-card.png | cff06a0cd438e8ef99b779ffe1a709508b2ae2a4a09702b6f6c1f82350922bf7 | 选卡后回到战斗态 |
| r22-05-empty-user.png | 059db5909611bca286480e6cf6b643799c881373037808c5cdf8efef76cb9dfe | 用户 127 选卡页（元素空态限制见 §3） |
| r22-06-empty-note.png | 7b733bd0f3fccb1e9132fde15955ddf749080b7212fc48372a65efa36455be1a | 空态卡面/AI 卡 |
| r22-07-settle-hash.png | 1fc611ef6b687f137a9f1f5214b38da1b3936d53195ea413457e77290e71d729 | 真实结算页：通关 + 回放哈希 1dbf94aebb62f81d（可验真） |

## 八、零真值与环境还原

- 含真 token 的 `auth-init.js` 已删除（`ops_stat` 确认 exists=false）；模板仅留 `__LYP_TOKEN__` 占位符。`data/` 与 `auth-init.js` 已入 `.gitignore`。
- 精确模式扫描（`C:\Users` / `Bearer eyJ` / 长 lyp_token 字面量 / password 字面量）审计目录 **0 命中**。
- QA 库快照 `data/qa-before.dump`（sha256=c0ce5ef16c68a641ca45cd9baf2125d31760e06a5dcd49a9ce81731879e1babc）已 `pg_restore --clean --if-exists` 还原，战报数回到 103。
- 未改业务源码、未执行 git 提交。
