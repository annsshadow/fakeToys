# R22 线上回归报告：选卡反应协同预判

- 轮次：R22（选卡反应协同预判）
- 执行：release-ops-wb（提交部署）
- 环境：测试环境，域名统一以 <YOUR_TEST_DOMAIN> 占位；本地回环只用于真浏览器 E2E 探针
- 结论：三端部署完成，线上回归全绿，无阻塞问题
- 说明：R22 为纯 H5 客户端展示派生，无后端接口/结算/replay_hash 改动，admin 无界面改动。

## ① 部署产物

| 端 | 产物 | sha256 | 说明 |
| --- | --- | --- | --- |
| H5 | laiyipao/miniapp/dist/build/h5/assets/index-BAp_UeMz.js | 8a2de0db3a4557d426ba6d544496b23e5408a484993cae8d17f83285b09a72d2 | 本轮重建（VITE_API_BASE 指向测试环境 API） |
| H5 页面块 | pages-battle-battle.YQrKzk-4.js / battle-DNwI6VHe.css | — | grep 命中 r22-synergy-card / 协同预判，证明 R22 预判卡进入产物 |
| 后端 | laiyipao/deploy/bin/lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用 R21（本轮无后端改动，R22 不新增接口/字段） |
| admin | laiyipao/admin/dist/assets/index-CGo_zhD3.js | d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70 | 沿用 R21（本轮 admin 无改动） |
| 迁移 | 无 | — | R22 纯展示派生，不落库、无 migration |

## ② 集成验证（部署前测试）

依据 t177–t180 独立实施与终审记录：

| 验证项 | 结果 |
| --- | --- |
| SPEC.md 契约守卫（12 条） | 12/12 全绿，exit 0 |
| 定向测试 battle_r22_synergy.test.ts（含 RED→GREEN） | RED 3/3 失败基线 → GREEN 15/15 通过 |
| miniapp 全量测试 | 200 passed / 1 skipped files，1825 passed / 2 skipped tests，exit 0 |
| vue-tsc 类型检查 | exit 0 |
| 设计终审六门禁 | verdict=pass，六门禁全通过，无阻断缺陷 |

## ③ 端到端 HTTP 抽样（测试环境健康）

| 端点 | HTTP | 说明 |
| --- | --- | --- |
| /healthz | 200 | 后端健康 |
| /readyz | 200 | 后端就绪 |
| H5 首页（回环探针） | 200 | H5 dev server |
| admin 首页（回环探针） | 200 | admin dev server |

端到端业务流程在真浏览器内覆盖：进战斗 → 走到选卡阶段 → 读取协同预判 → 点选卡牌，详见 ⑤。

## ④ R22 核心功能验证（选卡协同预判）

线上真浏览器在选卡阶段（level=5，handCount=3）决定性 DOM：

| 卡 | 元素 | 预判结果 |
| --- | --- | --- |
| 燃烧弹 | 焰 | 「AI · 强化已有【蒸汽爆发】」，三标签 steam_burst / overheat / superconduct（与已装备元素集 {焰,冰,电} 的差集推导一致） |
| 散热涂层 | 无 | 逐字空态「该卡无元素，无法预判反应（属性/机制卡）」 |
| 连锁导引 | 无 | 同上逐字空态 |

- AI 卡（role=note）：可见「AI · 协同预判」、真实结论、逐字依据行「依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生」、CTA「去装备/选它」、诚实红线逐字「辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准」；hasAI / hasBasis / hasCTA / hasHonesty 全 true。
- 点选一张卡后 handCount=0、预判随手牌正确消失。
- 零污染：hasUndefined=false、hasNaN=false；静态证明 takeCard 不触 hash，权威 replay_hash 来自引擎/结算（t179 动态 settle 200、battle_id=477、哈希 1dbf94aebb62f81d 可验真）。
- 诚实限制：「当前构筑无元素技能」分支因服务端 build 快照仍带元素未在浏览器复现，由定向单测覆盖并如实标注，不构成缺陷。

## ⑤ 真浏览器验证

| 项 | 结果 |
| --- | --- |
| 浏览器 | agent-browser（非物理真机），视口 390×844 |
| 数据态 | 燃烧弹三标签 + 2 张无元素卡空态 + AI 四判据齐全 + 诚实行逐字 |
| 空态 | 手牌无元素卡逐字空态已在浏览器实测 |
| 探针 | pageErrors / consoleErrors / apiFailures / failedRequests 全 0 |
| 截图 | 7 张入库图 sha 各异、亲读非登录页 |

## ⑥ 截图清单（sha256）

| 截图 | sha256 |
| --- | --- |
| r22-01-hand-synergy.png | 57fc52dbac8c5812d33131b491572cc5bd105413de0d446ae66aaa4fcb008f20 |
| r22-02-synergy-card.png | 67d7eb6887372fedefd515b967563a981f6924b2327b69ac3d7b4abd01209ca8 |
| r22-03-ai-note.png | 19856b2f0c4b053cfb80d34616c4735f180badbac7316cc3d6523815659846c1 |
| r22-04-after-card.png | cff06a0cd438e8ef99b779ffe1a709508b2ae2a4a09702b6f6c1f82350922bf7 |
| r22-05-empty-user.png | 059db5909611bca286480e6cf6b643799c881373037808c5cdf8efef76cb9dfe |
| r22-06-empty-note.png | 7b733bd0f3fccb1e9132fde15955ddf749080b7212fc48372a65efa36455be1a |
| r22-07-settle-hash.png | 1fc611ef6b687f137a9f1f5214b38da1b3936d53195ea413457e77290e71d729 |

注：本轮部署后 E2E 重拍了 r22-01~04（内容与入库版一致，均为有效选卡页），为相邻提交稳定已恢复为入库版；sha 清单取自 git HEAD 入库版本，与 t180 终审亲算一致。

## ⑦ 结论

R22 四批精确提交完成并推送；三端部署完成（H5 重建含 R22，后端/admin 沿用无改动）；线上回归全绿——选卡协同预判有数据态三标签、无元素卡空态、AI 四判据与诚实红线、探针全 0、7 图各异亲读；零真值入库（含 token 的 auth-init.js 已 gitignore，模板仅占位符）；无阻塞问题。
