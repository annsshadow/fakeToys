# R24 设计终审报告：战斗实时元素层与反应触发预判 HUD

- 轮次：R24（round24-live-element-hud）
- 任务：t192「R24 六门禁终审」Attempt3
- 终审人：design-reviewer-wb（设计终审 / workbuddy 路由）
- 日期：2026-11-24
- **终审判定：verdict = pass（六门禁全 PASS）**

## 终审方式说明

本报告为**独立复核**：不采信实现/证据成员的自述结论，逐项亲读 SPEC、契约、源码、测试、审计产物，独立复跑测试与守卫、亲算截图 SHA-256、亲读截图内容。所有门禁均以可复现的决定性证据裁定。

诚实边界：真浏览器证据为 agent-browser 无头浏览器（390×844 视口、本地回环），**非物理真机**；本轮证据足以证明功能/视觉/契约正确，但不等价于真机触控与性能实测。

---

## 门禁① SPEC 落地 + 契约 RED→GREEN — PASS

- 亲读 `docs/laiyipao/specs/round24-live-element-hud/SPEC.md`（60 行）：§0 真实缺口（静态 `BattleConfig.enemies` vs 运行时 `this.enemies`，`Enemy.stacks: Map<Element,bigint>`）、§1 只读快照契约（`uid/defId/name/hp/maxHp/stacks/elementCap`，照抄 R15 `reactionTimeline()` 的 `slice()` 范例）、§2 派生算法（`REACTION_PAIRS` × `lookupReaction`）、§3 选择器与逐字文案、§4 AI 四判据、§5 诚实红线、§6 test-first 契约、§7 令牌锚点齐全。
- 亲读契约 `laiyipao/miniapp/src/pages/battle/battle_r24_element_hud.contract.test.ts`（101 行 / 8 用例）：G1 反应真相、G1 空集与同元素、G2 引擎只读快照、G2 五类选择器与逐字空态/依据、G3 诚实红线、G4 AI 四判据、G5 令牌锚点、G5 源码守卫。
- 独立复跑：`vitest run` 三文件 **17/17 passed**（契约 8 + 引擎 5 + H5 4），exit 0；RED→GREEN 由 test-first 契约与 `git show HEAD` 基线佐证。

## 门禁② 引擎旁路快照不污染 replay_hash / 权威字段 — PASS

- 亲读 `laiyipao/miniapp/src/game/engine.ts:791-809`：`elementLayerHud()` 仅 `this.enemies.filter(e=>!e.dead).map()`，`stacks[el]=Math.min(Number(n),cap)` 为深拷贝；未触碰 `step/replayHash/buildSnapshotSegment/score/phase/waveIndex/tick`。
- 亲读 `r24ElementLayerHud.test.ts:163-188`：调用前后 `replayHash()/buildSnapshotSegment()/score/phase/waveIndex/tick` 全不变，两次调用结果相等，`settleInput(1)` **不含** `element_layer_hud`；:140-151 深拷贝（外部篡改 `row.stacks/name` 不回写引擎），:153-161 clamp 到 elementCap 且空场返回 `[]`，:190-200 同 seed 确定性。

## 门禁③ H5 raf + fallback 双路径 tick 后刷新 — PASS

- 亲读 `battle.vue:448-450` `refreshElementHud()`（`typeof eng.elementLayerHud==='function'` 守卫 + 数组兜底）；:452-488 用 `REACTION_PAIRS` × `lookupReaction` 同源派生预判，去重、按优先级排序，诚实结论与空态结论齐备。
- 双路径刷新均落地：**raf 路径** `:1155-1180` 回调内 `refreshElementHud(eng)`；**fallback 路径** `:1010-1042` `fallbackTick()` 内 `refreshElementHud(eng)` + `setInterval`。
- H5 测试「tick 后刷新响应式（含 fallbackTick 路径）」通过。

## 门禁④ 真浏览器决定性证据 — PASS

- 四图 SHA-256 **亲算与 REPORT 完全一致、两两不同**：
  - r24-01-hud-live-a.png = `3f9560ac52da963997066c42b16dd4109428afa725368497302907ac9defbe40`（数据态：游荡者电1 + 4 条预判 + AI 注区块）
  - r24-02-hud-live-b.png = `56981216f3a07c127d40c7d5ceda65767ebe2cf30dfc2ab792df5936dae9feee`（空态）
  - r24-03-settle.png = `c6e9a0aea239f0ad12fbe1416d6428ca7b3215cd41e1630522b71939b2212de1`（过渡帧）
  - r24-04-settled.png = `b440258a683132d5a2aa676080d04ac5dd9d666b96890c5b492d7f706a0fd0a7`（通关结算页）
- **亲读** r24-01 / r24-02 / r24-04 均为战斗页非登录页；r24-04 结算页明示 `hash: 7486252a776befe / 632ab67f900b9861`、`回放哈希 632ab67f900b9861（可验真）`、反应 71 次。
- 40 帧 DOM 采样 `data/hud-samples.jsonl`（wc -l = 40）：uid 逐帧变化（首帧 [2,3]、末帧 [93,94,90,95,92]），元素层文案与预判数实时变化，`hasUndefined/hasNaN` 命中数 = 0；提示逐字「已附着 电 + 再补 焰 → 过热 反应」等。
- 探针：`peRaw 294` 全为 autoplay 噪声，过滤后 `peReal=0 / ce=0 / af=0 / fr=0`；结算 `{status:200, battle_id:477}`，回放哈希 `632ab67f900b9861`。

## 门禁⑤ 引擎反应真相 + 诚实红线 + AI 四判据 — PASS

- 反应真相与 `elements.ts:290-332` 一致：`fire+ice=steam_burst`、`fire+lightning=overheat`、`ice+lightning=superconduct`，同元素 `lookupReaction= null`。r24-01 截图实证「焰+冰→蒸汽爆发 / 焰+电→过热 / 冰+电→超导」，契约 G1 断言 fire 不含 superconduct、ice/lightning 各含正确反应。
- 诚实空态逐字「当前场上无可预判的元素反应」（`battle.vue:59/61` + r24-02 截图亲读，不编造）；诚实红线逐字「辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准」（`battle.vue:67` + 截图）。
- AI 四判据齐备：可见汉字「AI」（`r24-ai-title`「AI · 实时元素层预判」）、真实数据结论（`r24Conclusion` 基于真实 stacks）、逐字依据行（`battle.vue:65`）、可执行 CTA（`去释放对应元素技能 →`）；契约 G4 断言通过。

## 门禁⑥ 五类真值零泄露 + R1–R23 不回退 — PASS

- `auth-init.js` 已删，仅保留 `auth-init.tpl.js`（第 2 行 `'__LYP_TOKEN__'` 占位符，探针仅做 localStorage 注入与 error/API 收集，不篡改字段）；`.gitignore:318-320` 忽略 `round24-live-element-hud/data/` 与 `auth-init.js`。
- 审计目录真值正则扫描（JWT / 私钥 / root@IP / 带口令 DSN）**0 命中**；Go 真值守卫 `TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo` **exit 0**；QA 库 `pg_restore` 已还原战报数 103。
- 令牌三锚点 `tokens.css:14-16` 保留 `lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6`，无 `--r24-` 新增；R22/R23 回归契约独立复跑 **27/27 passed**（R22 12 + R23 15），令牌锚点契约与 R15–R23 守卫不回退。

---

## 独立复跑命令汇总

| 命令 | 结果 |
|------|------|
| `vitest run`（r24 引擎 5 + 契约 8 + H5 4） | 3 files / 17 tests passed, exit 0 |
| `vue-tsc --noEmit`（miniapp） | exit 0 |
| `vitest run`（R22 + R23 回归契约） | 2 files / 27 tests passed, exit 0 |
| `go test ./internal/domain/ -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo'` | ok, exit 0 |
| `sha256sum` 四图 | 与 REPORT 逐字一致、两两不同 |
| `wc -l hud-samples.jsonl` + 坏值 grep | 40 行 / 0 个 hasUndefined·hasNaN |

## 结论

六门禁全部 PASS，未发现任何真实缺陷或回退。R24 战斗实时元素层 HUD 为纯展示性只读预判，不进入 replay_hash、不影响结算权威字段，诚实红线与 AI 判据完整，真浏览器证据决定性有效。**verdict = pass**，建议放行本轮归档。

唯一非阻塞备注：真浏览器证据基于 agent-browser 无头 + 本地回环，非物理真机；如需覆盖真机触控/性能，另安排真机专项，不影响本轮门禁判定。
