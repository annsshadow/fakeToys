# R12 玩法实测：跨系统成长目标感

> 截图说明：全部为**非物理真机**——桌面 Chrome 通过 agent-browser 模拟移动视口 390×844，
> 本地回环 <YOUR_TEST_DOMAIN>（127.0.0.1:5174），无真实域名 / 口令 / 密钥入仓。
> before 为本次实测开始时的探针图，after 为修复后同一视口重采；五张均逐张目视确认。

## 一、真浏览器走通流程与玩家困惑点

走通路径：首页 → 技能图鉴 → 装备图鉴 → 宝石图鉴 → 背包（强化入口 doUpgrade）。
截图见 `screenshots/probe-*.png`（before）与 `screenshots/after-*.png`（after）。

**决定性困惑点：四页各自为政、结论互相矛盾。**
修复前，首页 AI 战术洞察（marginalGain 权威口径）说「装备契合仅 0%（权重 21%）：优先补它，收益 378」；
但技能页 KPI 说「重炮弹 效率榜首选」、装备页说「母核碎片 优先刷取部位」、
宝石页说「锋锐石还没拥有，优先补它」、背包说「当前 3 种元素已覆盖多类反应链，优先补齐 3/4 出战槽」。
玩家进入任一成长页，看不到首页认定的那个短板，四个页面给出四个不同「最该强化」——
首页的权威口径没有传导到任何成长页，无法一眼判断「我现在最该强化什么」。

R6/R7 在测试环境无战报时都是 NO_HISTORY 诚实空态（正确行为，非缺陷），
其 basis/CTA「去打一场战斗」合法（r6Recommendation.emptyRecommendation route=/pages/stage/stage），无断链。

## 二、真实缺陷与 test-first 修复（RED → GREEN）

**修复：新增跨系统成长优先级统一口径。**
契约 `laiyipao/miniapp/src/styles/r12GrowthPriority.contract.test.ts`（9 项）先 RED：9 failed / 0 passed。
实现：
- 新增 `src/pages/_shared/r12GrowthPriority.ts`：`computeGrowthPriority(buildRating, rating_weights)` 纯函数，
  逐字复用首页 dims/marginalGain 口径（五维 caps 5/7/8/18/8、权重份额 weightPct=round(w/Σw×100)、
  gain=weightPct×gap、降序取第一、DIM_HINT 五个 hint 路由）。不 import game/api，不写真值。
- `common.css` 追加 LYP-R12-GROWTH-PRIORITY-V1 全局层（复用 --ai-* 令牌，零新增 --r12-*；
  reduced-motion 可关；CTA min-height 88rpx）。
- skills/equipment/gems/bag 四页顶部各插入同一张 r12-priority 卡：AI 徽章 + 真实结论 + 依据 + CTA。
修复后 GREEN 9/9；契约中途纠正一处自身过度断言（R6 卡只在 skills/equipment，gems/bag 有各自 AI 卡）。

**after 实证（非空态假阳性）：** 五张截图均显示真实短板数据。
首页与四成长页口径完全一致：装备契合 0/18（0%，权重 21%）：补满可涨约 378 分，优先补它，
CTA「去配装备 →」。玩家从任一页都能一眼看出同一个「最该强化什么」。

## 三、成长目标感最值得优先补的 3 个真实缺口（代码级依据）

1. **【本轮已修】跨系统成长优先级缺失，四页各自为政。** 依据：修复前 pages/index/index.vue:566 marginalGain、
   :586 aiInsight 与 pages/{skills,equipment,gems,bag} 各自的 r3Ai*/r4AiBag 结论互不一致；
   本轮以 r12GrowthPriority.ts 统一口径，四页 before/after 截图实证一致。
2. **【需后端配合】无战报时推荐只给「去打一场战斗」，无冷启动可执行成长动作。** 依据：
   server/internal/service/recommendations.go NO_HISTORY 分支 insights=[]，
   pages/{skills,equipment}/ loadR6Recommendation() 走 emptyRecommendation（route=/pages/stage/stage）。
   玩家在测试环境（无战报）无法获得「先升哪一级」的个性化动作，只能先去打一场。
   根因在服务端数据不足分支不返回冷启动成长候选，需队长派发后端任务；前端已按 §1I 诚实空态，不伪造 id/replace。
3. **【需后端配合】升级操作后无「战力变化」即时数值反馈。** 依据：pages/bag/bag.vue doUpgrade(~:1038)
   调 api.upgradeSkill 后仅 toast「升级成功 Lv.X」+ refreshProfile，
   升级响应（api/client.ts:1017）只带 wallet 与 level，不带 before/after power/rating 快照；
   R10 仅覆盖宝石佩戴属性差分与养成前后战报差分，技能/装备升级本身无即时战力 delta。
   需服务端在 /me/skills/:id/upgrade 响应增加 power_before/after，前端再呈现（本任务禁碰后端，交队长）。

## 四、验证

- 契约 r12GrowthPriority.contract.test.ts：9/9 GREEN（RED 9 failed → GREEN）。
- 全量 `pnpm test`：184 files passed | 1 skipped，1736 tests passed | 2 skipped。
- `pnpm type-check`：exit 0；`pnpm build:h5`：DONE，产物含 r12-priority-text / LYP-R12-GROWTH-PRIORITY-V1。
- 未回退 R6/R7/R10、apply-done、去背包镶嵌双 CTA、幂等加载；未碰 game/后端/admin；无 git、零真值。

## 五、截图清单（非物理真机 / 390×844 / 本地回环）

| 截图 | sha256 |
| --- | --- |
| after-00-home.png | 30aa5cf67ef6fa7a49bce77a07498d8d6a53ef3528a635a5be184249d5b58606 |
| after-01-skills.png | 0e9cc285fb4503971ba1543e488d47cef4b8a0d23a2ee76b95ea0463338c648e |
| after-02-equipment.png | 96059607ac137cb32d9486f183974c60f8d1f61471fa49a0044664ba684908a2 |
| after-03-gems.png | 7d2c282c7a90c24d27729875e58478657cf2e563312278ca53b8b3a281e2b9f3 |
| after-04-bag.png | 01bafcd7ee255d2827aae4749e2a766a6eb34689a378e77a210bf930f0a0978b |
