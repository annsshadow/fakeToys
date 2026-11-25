# R51 · me 页 AI 诊断口径统一 R12 + 五维雷达可视化规范

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改 me.vue 业务代码）。
> 裁定：me 页 AI 诊断必须复用首页与权威 helper 同一口径（权重份额 × 缺口），并新增五维雷达可视化；不改评分算法本身。

## 0. 真实缺口与依据（带行号）

1. 口径漂移：laiyipao/miniapp/src/pages/me/me.vue:896-915 的 r4AiMe computed 在 :899-901 逐字用 Object.entries(R4_DIM_CAP).map(([key, cap]) => ({ key, pct: Number(rating[key] ?? 0) / cap * 100 })).sort((a, b) => a.pct - b.pct)[0]! 取裸完成百分比最低者为短板；而权威 helper laiyipao/miniapp/src/pages/_shared/r12GrowthPriority.ts:53-84 的 computeGrowthPriority 用 gain = weightPct × gap 降序取第一，首页 index.vue 的 aiInsight/marginalGain 同口径。
2. 服务端权重权威：laiyipao/server/internal/domain/power.go:253-269 的 RatingWeights/DefaultRatingWeights 逐字给出 element_coverage 12 / reaction_coverage 15 / mastery_done 10 / equipment_synergy 12 / mechanic_depth 8；me.vue 的裸百分比排序完全忽略权重，导致与首页建议相反。
3. 两组真实存档矛盾（队长实测）：案例1 rating={ element_coverage:5, reaction_coverage:3, mastery_done:8, equipment_synergy:8, mechanic_depth:8 } → R4 判「反应链 43%」最短板，R12 判「装备契合」（gain 210 > 反应链 104）；案例2 rating={ element_coverage:2, 其余同上 } → R4 判「元素覆盖 40%」最短板，R12 仍判「装备契合」（gain 210 > 元素覆盖 63 > 反应链 104）。两组结论均相反，非虚构。
4. hint 路由不一致：me.vue:902-906 逐字 mastery_done→'/pages/mastery/mastery'、equipment_synergy→'/pages/bag/bag'、其余→'/pages/shop/shop'；权威 DIM_HINT（r12GrowthPriority.ts:40-46）逐字元素覆盖→'/pages/bag/bag'、反应链→'/pages/reactions/reactions'、专精投入→'/pages/mastery/mastery'、装备契合→'/pages/equipment/equipment'、机制深度→'/pages/bag/bag'。me 把元素/机制短板错误导向商店。
5. 可视化缺失：me.vue:18-26 只有总分数字（:20 {{ store.buildRating.total ?? 0 }} 分）与关卡/战力两格，:37-41 数据网格也只有文字；laiyipao/miniapp/src/pages/enemies/enemies.vue:297-316 已有可复用 SVG 五维雷达几何（resistDirs / rr-ring / rr-axis / rr-shape / rr-label），me 页未用。
6. 测试盲区：现有 me 相关测试只校验总分与 AI 文案字样，未校验「me 的优先补强结论是否与首页/权威 helper 同口径」，也未校验雷达存在。

## 1. 范围与零改动面

- 本轮只交付本 SPEC 与 test-first 契约测试；不改 me.vue、不改任何业务源码。
- 禁改面：engine / replay_hash / score / settle / loot / tokens / 后端结算；三锚点 --lyp-token-version: lyp-tokens-v1、--lyp-r3-version: lyp-tokens-v3-r3、--lyp-r6-version: lyp-tokens-r6 保留；不新增 --r51- / --r52- 令牌。
- 不新造评分算法：computeGrowthPriority 与 resolveR12Cta 只读复用，不改 helper 内部逻辑。

## 2. 权威口径修复判据（逐字契约）

1. me.vue 必须从 '@/pages/_shared/r12GrowthPriority' 同时 import computeGrowthPriority 与 resolveR12Cta。
2. r4AiMe computed 必须调用 computeGrowthPriority(store.buildRating, store.config?.rating_weights) 得到 priority；权重必须来自服务端 config.rating_weights，不得本地硬编码。
3. r4AiMe 必须调用 resolveR12Cta(priority, '/pages/me/me') 得到 r4Cta，CTA 文案取 r4Cta.label、跳转取 priority.hint；自指（self=true）时不得 navigateTo 自身。
4. r4AiMe 区块不得再出现裸百分比排序 sort((a, b) => a.pct - b.pct) 或 Object.entries(R4_DIM_CAP) 取短板，也不得再出现本地 hint 三元（mastery_done/equipment_synergy/其余→商店）。
5. 空态诚实：priority 为 null（无评分/权重全 0/全部补满）时 r4AiMe 返回 null，模板沿用现有「AI 等待数据」空态，不编造短板。
6. basis 必须取 helper 的 priority.basis 逐字「依据：服务端五维评分（build_rating）/ 权重份额 × 缺口 · 当前存档 · 更新于本次进入」，不再用 me 本地 basis。

## 3. 五维雷达 SVG 判据

1. 在 me.vue 构筑评分区（:18-26）附近新增 class 前缀 r51-radar 的 SVG，viewBox 同 enemies.vue 风格（0 0 200 180），aria-label 用中文（如「构筑五维雷达图」）。
2. 五轴顺序严格同 R12 DIMENSIONS：元素覆盖 / 反应链 / 专精投入 / 装备契合 / 机制深度；cap 依次 5 / 7 / 8 / 18 / 8；半径 = min(100, value / cap * 100) 映射到固定像素半径。
3. 必须包含环线（r51-radar-ring，至少两圈）、轴线（r51-radar-axis，5 条）、数据多边形（r51-radar-shape）、中文标签（r51-radar-label）与权重份额文字（展示 priority.weightPct 或 config.rating_weights 同源份额）。
4. 空 rating（store.buildRating 为 null）时不渲染雷达，沿用 v-if="store.buildRating" 守卫；不得画全 0 的假雷达。
5. 雷达旁逐字标注「非权威战斗数据；权威以 replay_hash 为准」。

## 4. AI 四判据（me 页延续）

1. 可见汉字「AI」：保留「AI 养成诊断」标题与徽章。
2. 真实数据结论：结论行展示 priority.label 与 pct/weightPct，来自服务端 build_rating，不编造。
3. 依据行：priority.basis 逐字（见 §2.6）。
4. 可执行 CTA：r4Cta.label + priority.hint；自指时走页内指引，不 navigateTo 自己（复用 R48 防护）。

## 5. 诚实红线（不可协商）

- build_rating 与 rating_weights 为服务端下发的展示/养成派生值；雷达与诊断不参与 replay_hash、不重算 score / 星级 / loot、不改 settle。
- 不得把雷达半径或诊断短板写成权威战斗数据；权威以 replay_hash 为准。
- 零真值：不写域名/IP/口令/令牌/本机路径真值。

## 6. test-first 契约测试与 RED 门禁

- 契约文件：laiyipao/miniapp/src/styles/r51MeRadar.contract.test.ts（真实 node:fs 读 me.vue 与 tokens.css；真实 import 权威 helper）。
- 守卫：me.vue import 与调用、r4AiMe 无裸排序/本地 hint 三元、r51-radar 标记与五轴标签、诚实口径文字、tokens 三锚点与无 --r51-、真实 helper 在两组权重下短板均为装备契合。
- RED 门禁命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r51MeRadar.contract.test.ts（必须 exit≠0，失败来自上述真实缺口断言，不得是解析/模块缺失/超时）。
- RED 逐字摘要（2026-11-26 实测）：命令 `cd laiyipao/miniapp && pnpm exec vitest run src/styles/r51MeRadar.contract.test.ts --reporter=dot` → exit=1，RUN v4.1.12，Test Files 1 failed (1)，Tests 2 failed | 2 passed (4)，Duration 492ms（transform 53ms / import 58ms / tests 38ms / environment 0ms）。两条逐字失败行：
  - r51MeRadar.contract.test.ts:35:54 → AssertionError: me 未 import computeGrowthPriority: expected '<template>...' to contain 'computeGrowthPriority'
  - r51MeRadar.contract.test.ts:69:35 → AssertionError: 缺雷达标记 r51-radar: expected '<template>...' to contain 'r51-radar'
  - 无 PARSE_ERROR / 模块缺失 / 超时；两条失败全部来自真实缺口断言（me.vue 未 import 权威 helper、无 r51-radar 雷达标记）；真实 helper 行为断言与令牌锚点断言已通过（2 passed）；me.vue 保持原样，GREEN 留下一轮。

## 7. 三视口 QA（GREEN 后执行）

- 390×844 / 768×1024 / 1280×800：雷达与标签不溢出、不重叠；AI 诊断结论与首页 aiInsight 指向同一短板；空 rating 显示 AI 等待数据且不渲染雷达。

## 8. 实施分工与修订轨迹

- R51（本轮）：钉规范与 RED 契约，不实施 me.vue。
- 下一轮 GREEN：只改 me.vue 的 r4AiMe 与评分区模板（+ 必要样式），不改 helper/后端/权重值，不加令牌。

## 9. RED 门禁实测记录（2026-11-26）

- 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r51MeRadar.contract.test.ts --reporter=dot`
- 结果：exit=1，RUN v4.1.12，Test Files 1 failed (1)，Tests 2 failed | 2 passed (4)，Duration 492ms。
- 逐字失败：r51MeRadar.contract.test.ts:35:54「me 未 import computeGrowthPriority: expected '<template>...' to contain 'computeGrowthPriority'」；r51MeRadar.contract.test.ts:69:35「缺雷达标记 r51-radar: expected '<template>...' to contain 'r51-radar'」。
- 判定：失败全部来自真实缺口断言，无解析/模块缺失/超时；RED 门禁达标，me.vue 不实施。
