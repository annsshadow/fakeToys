# R47 · R16 验真 CTA 言行一致性（兑现 SPEC 可执行语义）规范

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改 BattlesView.vue 业务代码）。
> 裁定：兑现可执行语义——CTA 必须可点击并定位到同抽屉验真区；不采用「把文案退化成只读指引」的保守派路线。

## 0. 真实缺口与依据（带行号）

1. R16 规范 `docs/laiyipao/specs/round16-admin-reaction-timeline/SPEC.md:39` 逐字：「4. **可执行语义**：给出 CTA 指向验真——例如「发起验真 →」定位到同抽屉已有验真区（`:558-589`），让运营看完逐次归因即可比对 replay_hash；不新造路由。」；`:46` 逐字「验收真区仍是唯一比对入口」。规范明确要求 CTA 是可执行的定位动作，而非装饰性文字。
2. 当前实现 `laiyipao/admin/src/views/BattlesView.vue:679` 逐字：`<div v-if="reactionTimeline.ok" class="r16-timeline-cta">发起验真 →</div>`——无 `@click`、非 `<a>` / `<router-link>`，是死文本 `div`。
3. 当前 CSS `laiyipao/admin/src/views/BattlesView.vue:741` 逐字：`.r16-timeline-cta { font-size: 12px; color: var(--el-color-primary, #409eff); margin-bottom: 4px; }`——无 `cursor: pointer`，无可点击视觉提示。
4. 真实验真区在 `laiyipao/admin/src/views/BattlesView.vue:617`（`<div class="section-title">发起验真</div>`）到 `:640`（提交比对按钮）之间；提交按钮 `:636` 逐字 `<el-button size="small" :loading="verifying" @click="submitVerify">`，`submitVerify` 定义于 `:268-279`，是既有唯一验真入口。
5. 测试盲区：`laiyipao/admin/tests/R16ReactionTimeline.render.test.ts:122` 与 `laiyipao/admin/tests/BattlesView.test.ts:450` 都只 `toContain('发起验真')` 校验字样，未校验 CTA 是否可点击或是否指向验真区，故言行不一致未被发现。

## 1. 范围与零改动面

- 本轮只交付本 SPEC 与 test-first 契约测试；不改 `BattlesView.vue`、不改任何业务源码。
- 禁改面：`engine` / `replay_hash` / `score` / `settle` / `loot` / 后端 / 令牌（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 三锚点保留）；不新增 `--r47-` / `--r48-` 令牌。
- 不新造路由、不裸 hash（admin 使用 `createWebHashHistory`，裸 `href="#xxx"` 会落 catch-all）；CTA 只在当前抽屉内定位。

## 2. 修复判据（逐字契约，供实施与 GREEN 验收）

1. `.r16-timeline-cta` 元素必须带可执行点击绑定：模板行含 `@click`，或 `v-on:click`；不得是纯 `div` 死文本。
2. 点击行为必须「定位到同抽屉验真区」：实现方法体中须出现 `scrollIntoView` 或 `focus` 之一，且引用验真区锚点（如 `verifySection` / `submitVerify` 按钮 ref / `.section-title` 验真标题）；仅 `ElMessage` 提示不算定位。
3. CSS 规则 `.r16-timeline-cta { ... }` 必须含可点击视觉提示：`cursor: pointer` 或 `pointer-events: auto`（沿用 admin/既有 CTA 惯例）。
4. CTA 文案保持逐字「发起验真 →」，不得改为只读指引措辞。
5. 不得出现裸 hash：`href="#` / `href='#` 均禁止；若用 `<a>` 必须绑定可点击处理并避免 hash 导航。
6. `submitVerify` 函数体（`BattlesView.vue:268-279`）、验真结论、replay_hash 比对逻辑不得改动；CTA 只做定位，绝不自动提交、不自动填入哈希。

## 3. 同仓先例（不重造）

- `laiyipao/admin/src/components/AiInsightCard.vue:43-44` 用 `router-link :to=insight.ctaPath` 表达可执行 CTA；`laiyipao/admin/src/views/DashboardView.vue:470` 用 `href=#/admin/battles` 合法 hash 路由。R47 的 CTA 是抽屉内定位而非跨页导航，故用 `@click` 定位而非路由。
- battle HUD 的 R43/R44 已建立「CTA 文案-行为一致性」契约范式：真实读源码定位 CTA 行断言 `@click` 绑定与 CSS 可点击性；本规范沿用该范式到 admin。

## 4. AI 四判据（延续 R16 §2）

1. 可见汉字「AI」：标题「AI · 反应时间线」保留。
2. 真实数据结论：摘要只陈述 timeline 真实事件（本局记录 N 次逐次反应，集中在第 M 波）。
3. 依据行：`依据：reaction_timeline.events（wave/tick/reaction/enemy_uid/damage），展示性归因、非权威结算`。
4. 可执行语义：本 R47 把「发起验真 →」兑现为可点击并定位到验真区的动作，让运营看完归因即可比对 replay_hash。

## 5. 诚实红线（不可协商）

- timeline 是展示性归因、非权威结算；不得用于重算 score / 星级 / loot，不参与 replay_hash。
- 验真区是唯一比对入口；CTA 只做定位，不改变验真结论，不伪造哈希。
- 缺数据走逐字空态 `该战报无逐次反应记录（旧版本结算）`，截断态逐字 `已截断，仅显示前 200 条`，均不因 CTA 增加而改变。
- 零真值：不写域名/IP/口令/令牌/本机路径真值。

## 6. test-first 契约测试与 RED 门禁

- 契约文件：`laiyipao/admin/tests/R16ReactionTimeline.render.test.ts` 追加 `describe('R47 R16 验真 CTA 言行一致性（真实 BattlesView）')`。
- 真实读源码：`node:fs` + `node:path` 读 `../src/views/BattlesView.vue` 与 `../../../docs/laiyipao/specs/round16-admin-reaction-timeline/SPEC.md`，禁止内嵌参考实现。
- 守卫：定位 `.r16-timeline-cta` 模板行断言含 `@click` / `v-on:click`、含 `scrollIntoView` 或 `focus`（可在该行或点击方法体）、CSS 规则含 `cursor: pointer` / `pointer-events: auto`、无裸 hash；SPEC:39 含「可执行」且 SPEC 与实现一致。
- RED 门禁命令：`cd laiyipao/admin && pnpm exec vitest run tests/R16ReactionTimeline.render.test.ts --reporter=dot`；期望 exit=1，失败来自新增 R47 断言（死 `div` 无 @click / CSS 无 cursor），不得是解析/模块缺失/超时。

## 7. 实施分工（下一轮 GREEN）

- 实施者只改 `BattlesView.vue` 一处 `.r16-timeline-cta` 元素（加 `@click` 与定位方法）与对应 CSS（加 `cursor: pointer`），并让新增契约转绿。
- 不改 `submitVerify` / 验真结论 / replay_hash / score / settle / loot / 后端 / tokens。

## 8. 修订轨迹

- R16（历史）：新增时间线模块与「发起验真 →」文案 + 可执行语义规范。
- R47（本轮）：发现 CTA 是死文本、与规范承诺不一致；裁定兑现可执行语义，先钉 test-first 契约（RED），下一轮实施 GREEN。
