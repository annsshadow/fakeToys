# R43 规范：战斗 HUD AI 解读区 CTA 文案—行为一致性契约

> 轮次 R43 · 规范负责人 same-route-lead · test-first RED→GREEN
> 本轮只交付规范与契约测试；实施（改 battle.vue）留给下一轮。

## 0. 缺口依据（只读侦察，均带文件:行号）

- `laiyipao/miniapp/src/pages/battle/battle.vue:53` 注释逐字：`<!-- R27 AI 波次推进解读：同源只读快照；只读展示不拦截点击 -->`。
- `battle.vue:58` 模板逐字：`<text class="r27-ai-cta" @click="openHelp">继续清理场上敌人 →</text>` —— 绑定 openHelp，与第 53 行自相矛盾。
- `battle.vue:95` 模板逐字：`<text class="r24-ai-cta" @click="openHelp">去释放对应元素技能 →</text>` —— 姊妹点，同根因。
- `battle.vue:80` 注释逐字：`<!-- R24 实时元素层与反应触发预判：只读快照派生，纯展示不拦截点击 -->`。
- `battle.vue:982-1000` `openHelp()`：`closeReactionTimeline()` → `helpVisible.value = true` → 未暂停且未结算时 `doPause()` 并置 `helpPausedByPage = true`。玩家点「继续清理场上敌人」实际打开图例遮罩并**暂停战斗**，与文案承诺完全相反。
- `battle.vue:2804` CSS 逐字：`.r27-ai-cta { color: var(--accent); font-size: 18rpx; font-weight: 700; pointer-events: auto; }`。
- `battle.vue:2778` CSS 逐字：`.r24-ai-cta { color: var(--accent); font-size: 19rpx; font-weight: 700; pointer-events: auto; }`。
- 盲区：`battle_r27_wave_progress.contract.test.ts:158` 只查 SPEC 含「CTA」字样；`battle_r24_element_hud.test.ts:167` 只测文案 `toContain('去释放对应元素技能')`；都不校验 @click 绑定。
- 范式：`battle_r28_live_reaction_attribution.contract.test.ts:92-97/:102-114` 已有「校验 CTA 绑定 + 读真实源码」的 RED 写法，本轮复用。

## 1. 范围与零改动面

- 本轮写域：本 SPEC + `battle_r27_wave_progress.contract.test.ts` + `battle_r24_element_hud.test.ts`（仅新增真实源码守卫 describe，不改既有断言）。
- 实施轮最小修复仅允许动 `battle.vue`：删除 `:58` 与 `:95` 的 `@click="openHelp"`，移除或对齐 `:2804`/`:2778` 的 `pointer-events: auto`。
- 硬禁改：`engine.ts`、`replay`、`replay_hash`、`score`、`settle`、`loot`、后端、`tokens.css`、`.gitignore`；禁 git 操作。
- 禁新增任何令牌（含 `--r43-`）；令牌三锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 必须原样保留（`src/styles/tokens.css:14-16`）。

## 2. 当前事实逐字表（修复前，RED 基线）

| 区块 | 模板行 | CTA 逐字文案 | CSS 选择器 | 当前绑定 | 当前 CSS |
| --- | --- | --- | --- | --- | --- |
| R27 | `battle.vue:58` | `继续清理场上敌人 →` | `.r27-ai-cta` | `@click="openHelp"` | `pointer-events: auto;` |
| R24 | `battle.vue:95` | `去释放对应元素技能 →` | `.r24-ai-cta` | `@click="openHelp"` | `pointer-events: auto;` |
| R26（对齐目标） | `battle.vue:310` | `结合本波威胁与手牌元素协同选卡` | `.r26-ai-cta` | **无 @click** | `color: var(--accent);`（无 pointer-events） |

## 3. 同仓先例（既定口径，非新发明）

- `battle.vue:310` R26：同类 CTA 纯只读提示，无 @click —— R27/R24 应对齐此形态。
- `battle.vue:77` R28：`<text class="r28-feed-cta" @click.stop="openReactionTimeline">查看完整反应时间线 →</text>` —— CTA 改指真实动作（时间线抽屉），不再指向图例。
- `battle.vue:2845` R29：`.r29-result-timeline-cta` 指向真实时间线入口；注释逐字「不新增 --r29-」。
- 归纳口径：**CTA 要么指向真实动作（R28/R29），要么是纯只读提示（R26）；绝不允许指向会暂停战斗的 openHelp，也绝不允许只读提示带 pointer-events:auto。**

## 4. 修复判据（实施轮 GREEN 条件）

1. `.r27-ai-cta` 所在模板行**不含** `openHelp`、**不含** `@click`。
2. `.r24-ai-cta` 所在模板行**不含** `openHelp`、**不含** `@click`。
3. `.r27-ai-cta` 与 `.r24-ai-cta` 的 CSS 规则体**不含** `pointer-events: auto`（也不含无空格写法 `pointer-events:auto`）；退化为与 `.r26-ai-cta` 一致的只读文本色。
4. 逐字文案 `继续清理场上敌人 →` 与 `去释放对应元素技能 →` 保留（只读提示仍可读）。若实施轮选择中性化文案，须同步更新两处契约断言与规范表，但本轮 RED 基线以逐字保留为准。
5. `.r26-ai-cta` 仍无 @click（锁定对齐目标，防止回归）。
6. 只读快照派生不进 replay_hash；`openHelp()` 函数体本身不改（图例按钮 `.help-chip` 仍可正常打开图例并暂停），只切断 AI 解读区到它的错误绑定。

## 5. AI 四判据与 CTA 诚实口径

- R27/R24 AI 解读区仍保留四要素：标题、真实数据结论、依据行、红线；CTA 是第四类元素但必须为**非交互提示**。
- 红线不改：R27 `辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准`；R24 `辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准`。
- 禁止把「继续清理」「去释放」实现成会改变战斗状态的动作；若产品要求真动作，须另立规范并指向真实动作函数（如 R28 的 openReactionTimeline），不得复用 openHelp。

## 6. 契约测试与 RED 门禁

- 新增位置：两测试文件各追加一个 `describe('R43 战斗 HUD AI 解读区 CTA 文案-行为一致性（真实 battle.vue）')`。
- 真实源码读取：r27 复用顶层 `battleSrc = readSrc('src/pages/battle/battle.vue')`；r24 用 `readFileSync(join(__dirname, './battle.vue'), 'utf-8')`。
- 定位方式：按 `class="<cls>"` 找模板行（R28 范式）；CSS 用 `indexOf(sel + ' {')` 截取规则体。**禁止内嵌参考实现**，断言对象是真实 battle.vue 文本。
- 断言集合：CTA 行 `toContain(逐字文案)` 且 `not.toContain('openHelp')` 且 `not.toContain('@click')`；CSS 规则体 `not.toContain('pointer-events: auto')` 与 `not.toContain('pointer-events:auto')`；R26 CTA 行 `not.toContain('@click')`。
- RED 门禁（必须真跑）：`cd laiyipao/miniapp && pnpm exec vitest run src/pages/battle/battle_r27_wave_progress.contract.test.ts src/pages/battle/battle_r24_element_hud.test.ts --config vitest.config.ts --reporter=dot`
- 期望：exit 1，失败来自新增 `not.toContain('openHelp')` / `not.toContain('@click')` / CSS `pointer-events` 断言；不得来自解析错误、模块缺失或超时。不要为了变绿改 battle.vue。

## 7. 实施分工（下一轮）

1. 实施者按 §4 做最小 diff：删两处 @click、去掉两处 pointer-events:auto。
2. 重跑 §6 同一命令应 exit 0；再跑 `pnpm run type-check` 与 miniapp 全量 `pnpm test` 不回退。
3. 真浏览器复查：点 R27/R24 AI 解读区任何位置不应暂停战斗、不应弹出图例；`.help-chip` 仍可打开图例。
4. 终审核对：三令牌锚点原样、无 `--r43-`、SPEC 与源码零真值。

## 8. 红线与零真值

- 不碰 engine / replay_hash / score / settle / loot / 后端 / tokens。
- 不新增令牌，不回显任何密钥、口令、真实域名、IP、个人绝对路径。
- 规范与测试只描述仓库相对路径与公开源码常量，不含环境真值。
