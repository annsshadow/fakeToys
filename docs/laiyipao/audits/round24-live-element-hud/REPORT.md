# R24 真浏览器实时 HUD 复查报告（复查人 qa-e2e-wb）

## 判定

**PASS** —— 五项验收全部通过，未发现缺陷。

> 测试环境使用 agent-browser 无头浏览器（**非物理真机**），视口 390×844；所有"真浏览器实测"结论均指该无头 Chromium 环境，不等同于真机手势操作。

## 复查对象与依赖

- 规范：`docs/laiyipao/specs/round24-live-element-hud/SPEC.md`（t188，60 行）。
- 引擎只读快照 `elementLayerHud()`：t189，RED 5/5 → GREEN 5/5。
- H5 战斗页 `.r24-element-hud` 消费与派生：t190，目标 12/12、全量 1860 通过、type-check exit 0。
- 三者均为**只读辅助**展示：不进 replay_hash、不参与结算/score/伤害。

## 验收① 真浏览器打战斗：HUD 元素层数与反应预判实时变化

- 会话 `lyp-r24-e2e`，URL `http://127.0.0.1:5174/#/pages/battle/battle?level=5`，真实自动选卡打到结算。
- 40 次逐秒 DOM 采样落 `data/hud-samples.jsonl`（双层 JSON 解析统计）：
  - 捕获敌人 uid 共 21 个：`2/3/28..32/60..65/88..95` —— 随波次推进不断变化。
  - 元素层文案集合：`电1`、`电2`、`冰1·电2`、`冰2·电2`、`焰1`、`焰1·冰2`、`焰1·电2`、`焰1·冰2·电2`、`无元素层` —— 附着/清空实时变化。
  - 每帧行数序列：`2,2,2,2,1,…,0(空场),…,8,7,5,5`；预判提示数序列：`4,4,6,0,0,16,4,8,…,18,26,34,38,30` —— 随层数波动。
  - 有数据态首条提示逐字：`已附着 电 + 再补 焰 → 过热 反应`、`已附着 冰 + 再补 焰 → 蒸汽爆发 反应`、`已附着 焰 + 再补 毒 → 燃烧云 反应` 等。
  - 空态逐字：`当前场上无可预判的元素反应`，AI 结论 `当前场上无可预判的元素反应，不编造建议`。
  - **AI 四判据全程齐备**：可见标题 `AI · 实时元素层预判`；真实数据结论；依据行逐字含 `依据：敌人 stacks 元素层 × REACTION_PAIRS(源自引擎 lookupReaction) 派生；只读快照不进 replay_hash`；CTA `去释放对应元素技能 →`；诚实行逐字 `辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准`。
  - 全部 40 帧 `hasUndefined=false && hasNaN=false`。
- 结论：HUD 确实随战斗实时刷新，非静态假数据；空态诚实，无编造建议。

## 验收② hash 不污染（静态源码 + 动态实测）

- 静态源码：`laiyipao/miniapp/src/game/engine.ts:791-809` `elementLayerHud()` 仅 `this.enemies.filter(e=>!e.dead).map(...)` 读 stacks 并深拷贝，`stacks[el]=Math.min(Number(n),cap)`；未触碰 `step()` / `replayHash()` / `buildSnapshotSegment()` / `score` / 结算 / 伤害。
- 静态测试（`laiyipao/miniapp/src/game/r24ElementLayerHud.test.ts:163-188`）断言：两次调用前后 `replayHash()` / `buildSnapshotSegment()` / `score` / `phase` / `waveIndex` / `tick` 均不变，`settleInput(1)` 不含 `element_layer_hud` 字段；:140-151 深拷贝验证；:153-161 clamp 与空场；:190-200 确定性。
- 动态实测：整局结束后结算页回放哈希 `632ab67f900b9861`（可验真），settle 响应 `status=200, battle_id=477`；结算 AI 复盘卡显示权威 `replay_hash` 与hash一致，证明 HUD 刷新未改变回放哈希。
- 目标测试实跑：`npx vitest run --config vitest.config.ts src/game/r24ElementLayerHud.test.ts src/pages/battle/battle_r24_element_hud.test.ts src/pages/battle/battle_r24_element_hud.contract.test.ts --reporter=dot` → **3 files / 17 tests ALL PASSED**；契约测试 8 用例 5 组守卫覆盖选择器/逐字文案/AI 判据/诚实红线/三端不回退。

## 验收③ 探针 pe/ce/af/fr

- `window.__LYP_PROBE__`（document-start 注入，劫持 error/unhandledrejection/console.error/uni.request）最终读数：
  - `peRaw=294`：**全部为浏览器自动播放限制噪声**（`unhandled:play() failed because the user didn't interact with the document first`，无头环境无用户手势，属已知环境噪声，非业务错误）；按噪声模式过滤后 `peReal=0`。
  - `consoleErrors=0`、`apiFailures=0`、`failedRequests=0`。
- 结算响应捕获：`[{status:200, battle_id:477}]`，无失败请求。

## 验收④ 截图与亲读（均非登录页，sha256 两两不同）

目录：`docs/laiyipao/audits/round24-live-element-hud/screenshots/`

| 截图 | sha256 | 亲读结论 |
| --- | --- | --- |
| r24-01-hud-live-a.png | `3f9560ac52da963997066c42b16dd4109428afa725368497302907ac9defbe40` | 战斗中有数据态：游荡者 电1，列出"再补 焰→过热"等 4 条预判，AI 卡依据行/CTA/诚实行齐备，非登录页 |
| r24-02-hud-live-b.png | `56981216f3a07c127d40c7d5ceda65767ebe2cf30dfc2ab792df5936dae9feee` | 战斗中空态：`当前场上无可预判的元素反应`，不编造建议，非登录页 |
| r24-03-settle.png | `c6e9a0aea239f0ad12fbe1416d6428ca7b3215cd41e1630522b71939b2212de1` | 战斗中过渡帧，HUD+AI 卡可见，非登录页 |
| r24-04-settled.png | `b440258a683132d5a2aa676080d04ac5dd9d666b96890c5b492d7f706a0fd0a7` | 通关结算页：回放哈希、AI 复盘卡、`查看本局战报`，非登录页 |

## 验收⑤ 零真值与收尾

- 含真 token 的 `auth-init.js` 已删除（`finalize.sh`: `AUTH_INIT_DELETED`），仅保留 `__LYP_TOKEN__` 占位模板 `auth-init.tpl.js`。
- `.gitignore` 第 319–320 行忽略 `round24-live-element-hud/data/`（含 guest_token 的 dump 与样本）与 `auth-init.js`；`git check-ignore -v` 确认生效。
- 零真值正则扫描（JWT/guest_token 字面值/Authorization/Bearer/带口令 DSN/password 字面值）审计目录 **0 命中**。
- QA 数据库还原：`docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < data/qa-before.dump` → exit 0，战报数恢复为 103（与快照前一致）。
- 全程未改业务源码、未执行 git 提交。R24 相关文件仍为未跟踪状态（`??`）。

## 复现命令

- 启动：`bash docs/laiyipao/audits/round16-admin-reaction-timeline/start-api.sh`（后台）、`bash docs/laiyipao/audits/round15-reaction-timeline/start-h5.sh`（后台）。
- mint：`bash docs/laiyipao/audits/round24-live-element-hud/mint-token.sh 10`。
- E2E：`bash docs/laiyipao/audits/round24-live-element-hud/e2e.sh`（40 帧采样）、`bash docs/laiyipao/audits/round24-live-element-hud/e2e-finish.sh`（续跑到结算）。
- 收尾：`bash docs/laiyipao/audits/round24-live-element-hud/finalize.sh`。