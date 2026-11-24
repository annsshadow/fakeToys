# R25 复查报告：单局战报反应伤害主力聚合卡（真浏览器实测）

- 任务：t196（attempt 2）｜复查人：qa-e2e-wb｜日期：2026-11-24
- 判定：**PASS**——真浏览器实测有数据态战报 378 的 `.r25-reaction-impact` 聚合行数值、占比、count 口径与 SQL 独立核验逐字段一致；旧战报 339 诚实空态正确；AI 四判据/依据行/诚实红线齐备；探针 pe/ce/af/fr 全 0；4 张截图 sha 各异且亲读非登录页；零真值。未改业务源码、未做 git。
- 浏览器：agent-browser（**非物理真机**，本地回环 H5 服务，视口 390×844 与 1258×588 截图）。

## 验收结论一览

| 验收项 | 结论 | 证据 |
| --- | --- | --- |
| 聚合行数值与逐条 damage 求和一致 | 通过 | battle378 DOM 三行伤害 5239.0/4472.0/3438.0 = SQL SUM(damage) 逐字段一致 |
| 占比正确（toFixed(1)、降序） | 通过 | 39.8%/34.0%/26.1%，手算 5239/13149=39.8%、4472/13149=34.0%、3438/13149=26.1% 吻合，按伤害降序 |
| count 取权威 reactions_used 不冒充 | 通过 | DOM『权威23次/27次/19次』= reactions_used{steam_burst23,superconduct27,overheat19}；源码 :723 取 detail.reactions_used，契约断言 count≠events 场景 |
| AI 四判据 + 依据行 + 诚实红线 | 通过 | probe-378.json：aiTitle/conclusion/basis/cta/honesty 全部存在且逐字比对一致 |
| 旧战报诚实空态 | 通过 | battle339 时间线 [] → rows=0、empty 逐字『本局无逐次反应伤害记录』、conclusion『不编造主力』，未用 reactions_used 反推主力 |
| 探针 pe/ce/af/fr 全 0 | 通过 | 两态探针 pe/ce/af/fr 均 0（已过滤 Autoplay 噪声） |
| 截图 sha 各异 + 亲读非登录页 | 通过 | 4 张截图 sha 两两不同，均亲读确认为战报页（非登录页） |
| 零真值 | 通过 | 扫描唯一真值命中为含 JWT 的 auth-init.js，已删除；脚本/模板仅占位符与回环地址 |

## 1. 口径与测试（不采信实施者自述，亲读源码 + 实跑）

- 规范 `docs/laiyipao/specs/round25-battle-reaction-impact/SPEC.md`（3277B）已读全文：纯前端派生 `total_damage=Σdamage`、`events` 条数、`count` 取权威 `reactions_used`（严禁 events 冒充）、占比 SUM=0 归零、旧战报空态逐字、AI 四判据、诚实红线逐字、三端不回退。
- 源码 `laiyipao/miniapp/src/pages/battles/battles.vue:708-745` 亲读：`reactionImpact` computed 消费已过滤的 `reactionTimeline`（:683-703 五字段校验+wave/tick 排序），按 reaction 分组累加 damage，`:723 count: typeof used[reaction]==='number'?used[reaction]:0`，`:724 toFixed(1)`，`:727` 伤害降序+key 字典序；`r25Conclusion` 取并列第一生成真实结论。纯展示，不碰引擎/step/replay_hash/score/结算/伤害/loot。
- 测试实跑：`cd laiyipao/miniapp && npx vitest run --config vitest.config.ts src/pages/battles/battles_r25_impact.contract.test.ts src/pages/battles/battles_r25_impact.test.ts --reporter=dot` → **2 files / 19 tests ALL PASSED**（vitest 4.1.12, 1.95s）。契约 16 断言含五类选择器、逐字文案、count 权威口径、SUM=0 不产 NaN、三端令牌锚点无 `--r25-`、零真值。

## 2. 有数据态：战报 378（与 SQL 独立核验逐字段一致）

- 真浏览器打开 `#/pages/battles/battles?battle_id=378` 并展开详情，DOM 探针 `data/probe-378.json`：
  - timelineRows=69；card=true、role=status。
  - 聚合三行：`steam_burst 蒸汽爆发 5239.0伤害 39.8% 权威23次 事件23条`、`superconduct 超导 4472.0伤害 34.0% 权威27次 事件27条`、`overheat 过热 3438.0伤害 26.1% 权威19次 事件19条`（按伤害降序）。
  - AI 四判据：aiTitle『AI · 反应伤害主力』；conclusion『AI：本局蒸汽爆发造成 39.8% 反应伤害，是伤害主力』；basis 逐字『依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准』；cta『下一局优先凑齐该元素组合 →』；honesty 逐字『辅助展示非伤害权威,权威以 replay_hash 为准』。
  - hasUndefined=false、hasNaN=false。
- SQL 独立核验（`docker exec lyp-qa-pg psql`）：
  - 按 reaction 聚合：`steam_burst|5239|23`、`superconduct|4472|27`、`overheat|3438|19` —— 与 DOM injury/events 完全一致。
  - 权威次数：`reactions_used={"overheat":19,"steam_burst":23,"superconduct":27}` —— 与 DOM『权威 N 次』逐字段一致。
  - 回放哈希 `battle_records.id=378 → 7468252a57f6befe`，验证为展示性派生、未改权威结算。
- 说明：本局每条反应的 events 条数（23/27/19）与 reactions_used（23/27/19）数值恰好相等，属数据巧合；卡片代码取的是 `reactions_used`，契约测试已用 count≠events 的反例钉死口径，不因本局巧合而混淆。

## 3. 旧战报诚实空态：战报 339

- 真浏览器打开 `#/pages/battles/battles?battle_id=339`，DB 中 `reaction_timeline=[]`。DOM 探针 `data/probe-339.json` + 补充探针：
  - rows=0、timelineRows=0；empty 逐字『本局无逐次反应伤害记录』；conclusion『本局无逐次反应伤害记录，不编造主力』；basis/cta/honesty 仍在。
  - 展开行回放哈希 `44faed4a07a3593b`（SQL 核验 id=339 哈希一致），确认空态卡属于 339 而非沿用上一局数据。
  - 该战报 `reactions_used={overheat21,steam_burst21,superconduct30}` 有值，但无 timeline 伤害行 → 卡片未反推波次/敌人/伤害、未编造主力行，符合 R15/R25 诚实红线。
- 探针记录响应顺序说明：`detailResponses` 数组记录的是页面加载过程中各详情请求的先后（含 loadGrowthCompare 拉取的同关战报），其数组末项可能不是当前展开行；已用展开行 hash 精确复核，DOM 数据与实际展开战报逐字段对应，不影响结论。

## 4. 探针与诚实红线

- 两态探针 `window.__LYP_PROBE__`（document-start 注入，`window.uni.request` 劫持捕获详情响应）：pageErrors / consoleErrors / apiFailures / failedRequests 全为 0；loginPage=false。
- 诚实红线：卡为只读派生，不改结算/replay_hash/score/loot；count 取权威 reactions_used；无 timeline 不编造；截断提示『已截断，仅统计前 200 条』在 truncated=true 时显示（本局未截断，note 为空，符合预期）。三端令牌锚点 v1/v3-r3/r6 保留且无 `--r25-`（契约守卫通过）。

## 5. 截图清单（sha256，均已亲读，非登录页）

| 截图 | sha256 | 内容 |
| --- | --- | --- |
| r25-01-impact-378.png | fa24f998555ddad04850b0367777942597d9e8fde85f6c8289fcad79de0e0c31 | 战报列表顶部（battle378 深链，非登录页） |
| r25-02-impact-detail.png | ed07f5749f3ee9ec57feb01ecd8bd0b77ff96441313cc959f9b55b622367bd08 | 决定性证据：聚合卡三行真实数据+AI四判据+诚实行，非登录页 |
| r25-03-old-empty.png | 6fc54cc3b1af2b56c88c91ef73280e6729483c52b3b6dfbb4cd65bc717fa38b5 | 旧战报339 列表顶部（非登录页） |
| r25-04-old-empty-scroll.png | 375274166220ffa64ea4eaecffda8b224256c3156b3caf7351824ea8aa5a3033 | 旧战报339 聚合卡逐字空态+AI卡+回放哈希44faed4a07a3593b，非登录页 |

## 6. 零真值与清理

- 精确模式扫描审计目录：唯一真值命中为 `auth-init.js` 中的 JWT（测试注入用），已删除（保留 `__LYP_TOKEN__` 占位模板 `auth-init.tpl.js`）；脚本中的 127.0.0.1 为回环地址，符合仓库约定，非真值。
- `.gitignore` 追加 `round25-battle-reaction-impact/data/` 与 `.../auth-init.js`，含 token 的注入脚本与 dump/probe 数据不入库。
- QA 库已 pg_restore 还原（`data/qa-before.dump`，dump 前 sha256=10300158601c897d4fbc27f8da5448962d66455d21046a2bbb54c36d6ff6f32e），还原后战报数与快照一致。
- 全程未改业务源码、未做 git；仅审计目录新增脚本/证据/报告。