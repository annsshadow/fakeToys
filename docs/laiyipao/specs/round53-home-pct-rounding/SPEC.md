# R53 规范：首页 dims.pct 取整，消除 AI 结论长浮点

本规范钉死一个用户可见的展示瑕疵：首页构筑五维 pct 用裸浮点，直出到 AI 结论。修复只涉及首页 dims computed 的五条 pct 表达式，不改口径、不改导航、不改任何权威数据。

## 0. 真实缺口（带行号与逐字证据）

- 首页 laiyipao/miniapp/src/pages/index/index.vue:470-528 dims computed：
  - :485 逐字「pct: ((r?.element_coverage ?? 0) / 5) * 100,」
  - :492 逐字「pct: ((r?.reaction_coverage ?? 0) / 7) * 100,」
  - :499 逐字「pct: ((r?.mastery_done ?? 0) / 8) * 100,」
  - :506 逐字「pct: Math.min(100, ((r?.equipment_synergy ?? 0) / 18) * 100),」
  - :513 逐字「pct: ((r?.mechanic_depth ?? 0) / 8) * 100,」
  - 五条均为裸浮点：8/18×100 = 44.44444444444444。
- 首页 :75 AI 结论模板逐字「{{ aiInsight.weak.pct }}%」直出该浮点；真浏览器 QA（R51）截图与部署后报告已如实登记此瑕疵。
- 权威对照 laiyipao/miniapp/src/pages/_shared/r12GrowthPriority.ts:14 注释逐字「当前完成百分比（整数，避免长浮点直出）」、:68 逐字「pct: Math.round(Math.min(100, (cur / d.cap) * 100)),」——R51 me 页结论已是整数 44%。
- 讽刺点：该 helper :5 注释逐字「口径完全复用首页 index.vue 的 dims + marginalGain」——首页是权威口径声明的源头，自身却违反整数口径。
- 排除误判：:522 的 weightPct 已是 Math.round；:547 的 ratingGrade.pct 已是 Math.round；唯独 dims 五维 pct 漏网。

## 1. 修复判据（GREEN 验收条件）

- dims computed 的五条 pct 全部被 Math.round(...) 包裹；装备契合须保留 Math.min(100, ...) 上界再取整（等价权威 helper :68 的 Math.round(Math.min(100, ...))）。
- 五条 pct 均为 0..100 的整数（Number.isInteger），不得引入 NaN/负值；原有 ?? 0 与 Math.min 语义保留。
- 示例：equipment_synergy=8、cap=18 → 44%（非 44.44444444444444%）；element_coverage=2、cap=5 → 40%；reaction_coverage=3、cap=7 → 43%。

## 2. 不变量（改动只允许触碰五条 pct 表达式）

- 权重份额 weightPct（:522）与服务端 rating_weights（:472）口径不变。
- marginalGain（:566-579）仍用 d.weightPct × gap 排序，不依赖 pct；排序结论与 R12 helper 一致。
- aiInsight（:586-591）仍从 dims.value.find 取 weak；AI 结论模板 :75 不变，仅数值由浮点变整数。
- 雷达几何 radarPolygonActual（:652-659）、进度条宽度 :179、空态判定 :186（d.pct === 0）继续消费 dims.pct，行为仅亚像素差（44.444→44），空态不变。
- 导航 go(aiInsight.hint)、DIM_HINT（:668）与 resolveR12Cta 口径完全不变；R12（四页统一短板）与 R51（me 页）不回退。

## 3. 同仓先例

- laiyipao/miniapp/src/pages/_shared/r12GrowthPriority.ts:68 Math.round(Math.min(100, (cur / d.cap) * 100))——本次首页修复须逐字对齐此式。
- laiyipao/miniapp/src/pages/index/index.vue:522 weightPct 与 :547 ratingGrade.pct 已是 Math.round，证明本仓既有整数口径惯例。
- laiyipao/miniapp/src/pages/me/me.vue:911 已复用 helper，其 pct 为整数；首页对齐后两页 AI 结论数值口径一致。

## 4. 三视口 QA 口径

- 390×844 / 768×1024 / 1280×800：-buildRating 为 8/18 的存档下，AI 结论显示「装备契合仅 44%」且不含任何长浮点；五维进度条与雷达可见、不溢出；空 rating 时显示「AI 暂时没有足够构筑数据」且 pct===0 的空维入口照常出现。
- 交叉验证：同一存档在 me 页 R51 诊断与首页 R53 结论均为装备契合 44%；四页 r12Priority 结论不回退（R12）。

## 5. 诚实红线

- 不改后端 / engine / replay / hash / score / settle / loot / tokens；不改 rating_weights 与服务端 BuildRating（cap 5/7/8/18/8）。
- 不新增 --r53- 令牌；tokens.css 三锚点 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6 保留。
- 本规范只写规范与契约，不改 index.vue 业务源码；GREEN 留下一轮实施。

## 6. test-first RED 门禁

- 契约 laiyipao/miniapp/src/styles/r53HomePct.contract.test.ts：真实读 index.vue 源码（node:fs + __dirname 相对 srcRoot），截取 dims computed 区块，断言五条 pct 均含 Math.round、无裸 pct: (( 模式；真实 import 权威 helper 用 8/18 案例断言 pct=44 整数并要求首页对应行含 Math.round；不变量断言 marginalGain/雷达/进度条/空态/tokens 仍在。
- 当前实现必 RED：五条裸浮点 pct 不含 Math.round，装备契合行不含 Math.round。
- RED 摘要（2026-11-26 真跑）：命令 cd laiyipao/miniapp && pnpm exec vitest run src/styles/r53HomePct.contract.test.ts --reporter=verbose → exit=1，RUN v4.1.12，Test Files 1 failed(1)，Tests 2 failed | 2 passed (4)，Duration 334ms。
- 逐字失败 1（契约 :38）：pct 裸浮点未取整: pct: ((r?.element_coverage ?? 0) / 5) * 100,: expected 'pct: ((r?.element_coverage ?? 0) / 5)…' to contain 'Math.round('；Received: pct: ((r?.element_coverage ?? 0) / 5) * 100,
- 逐字失败 2（契约 :67）：首页装备契合 pct 必须 Math.round 取整，与 helper 44% 一致: expected 'pct: Math.min(100, ((r?.equipment_syn…' to contain 'Math.round('；Received: pct: Math.min(100, ((r?.equipment_synergy ?? 0) / 18) * 100),
- 另 2 it 通过（AI 结论模板不变量、其他不变量）。两条失败全部来自真实缺口断言（裸浮点 pct 无 Math.round），无 Transform failed / 模块缺失 / 超时。GREEN 留下一轮实施，本轮不改 index.vue。
