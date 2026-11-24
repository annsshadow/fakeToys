# R25 设计终审报告：单局战报反应伤害主力聚合卡

- 任务：t197「R25终审：六门禁独立复核」｜终审人：design-reviewer-wb｜日期：2026-11-24
- attempt_id：91ba3961-e5fa-4c64-bb6c-a401f85a07b7
- 依赖：t194（SPEC+契约）、t195（H5 实施）、t196（真浏览器复查 PASS）
- **verdict = pass**：六门禁独立复核全部通过，未发现真实缺陷。

## 终审方式与独立性声明
不采信实施/复查成员自述，全部结论来自本人独立亲读源码与规范、独立复跑测试、独立亲算截图 SHA、独立亲读截图、独立只读 SQL 复核 QA 库。真浏览器证据来自 agent-browser（**非物理真机**，本地回环 H5 服务，视口 390×844 与 1258×588）；此为覆盖边界，非 R25 缺陷。

## 门禁① SPEC 落地 + 契约 RED→GREEN
- 亲读 `docs/laiyipao/specs/round25-battle-reaction-impact/SPEC.md`（55 行/3277B）全文：§0 缺口、§1 纯前端派生口径（count 取权威 reactions_used 严禁 events 冒充、SUM≤0 归零防 NaN、降序+key 字典序）、§2 诚实空态逐字、§3 五类选择器、§4 AI 四判据、§5 诚实红线逐字、§6 契约 test-first、§7 三端不回退，条款自洽。
- 契约 `battles_r25_impact.contract.test.ts`（176 行）亲读：16 断言含分组聚合、count≠events 反例（事件 2 条 vs 权威 7 次）、占比 toFixed(1)、SUM=0 全 0/isFinite、旧战报空数组、reactionName 映射兜底、同伤害字典序、五类选择器、逐字文案、令牌三锚点无 `--r25-`、零真值。
- 行为测试 `battles_r25_impact.test.ts`（126 行）亲读：jsdom 挂载真实组件，三组场景（有数据降序/权威 count/空态三变体/truncated+AI 四判据+逐字文案）。
- 独立复跑：`vitest run` 两文件 → **2 files / 19 tests ALL PASSED**（vitest 4.1.12, exit 0）。RED 基线由 t195 以 test-first 交付，GREEN 实测成立。

## 门禁② 纯前端派生不碰结算/replay_hash/score/loot
- 亲读 `battles.vue` `reactionImpact` computed（:708-745）：只消费 `reactionTimeline.value`（已过滤校验的干净事件）与 `detail.value?.reactions_used`；无网络请求、不写引擎、不碰 `step()`/`replayHash()`/score/loot/结算/伤害。
- 独立 SQL 复核 QA 库：战报 378 `replay_hash=7468252a57f6befe` 与 t196 复查前一致，结算权威字段未被派生卡改写。派生结果确为展示性只读。

## 门禁③ count 取权威 reactions_used 不冒充（队长关注点①）
- 源码 :723 `count: typeof used[reaction] === 'number' ? used[reaction] : 0`，明确取 `reactions_used`，缺值取 0；events 仅为时间线事件条数，二者分列展示（模板 `权威{{row.count}}次` / `事件{{row.events}}条`）。
- 契约反例钉死口径（steam_burst 事件 2 条、权威 7 次，断言 count=7 且 count≠events）。
- 独立 SQL 复核 378：`reactions_used={overheat:19, steam_burst:23, superconduct:27}`；DOM 三行「权威 23/27/19 次」逐字段一致。本局 events 条数（23/27/19）与权威次数数值巧合相等，属数据巧合，不构成口径混淆。

## 门禁④ 占比 SUM=0 归零防 NaN（队长关注点③）
- 源码 `damage_share_pct: Number((sum > 0 ? (g.total / sum) * 100 : 0).toFixed(1))`：sum≤0 一律归零，不产 NaN/Infinity/-0。
- 契约专门断言全 0 事件下每行占比 0、`Number.isNaN=false`、`Number.isFinite=true`。DOM 探针 `hasNaN=false, hasUndefined=false`。
- 378 占比手算复核：总和 5239+4472+3438=13149；5239/13149=39.8%、4472/13149=34.0%、3438/13149=26.1%，与 DOM 三行逐项吻合且按伤害降序。

## 门禁⑤ 旧战报诚实空态逐字（队长关注点④）
- 源码模板空态 `.r25-impact-empty`：逐字「本局无逐次反应伤害记录」；无 timeline 时 `r25Conclusion` 返回「本局无逐次反应伤害记录，不编造主力」。
- 行为测试三变体（无字段/空数组/非数组结构）均走逐字空态、0 行、不编造。
- DOM 探针 probe-339：`rows=[]`、`empty="本局无逐次反应伤害记录"`、`conclusion="本局无逐次反应伤害记录，不编造主力"`；独立 SQL 复核 339 `reaction_timeline=[]` 但 `reactions_used={overheat21,steam_burst21,superconduct30}` 有值——卡片未反推波次/敌人/伤害、未编造主力行，符合诚实红线。

## 门禁⑥ 真浏览器决定性证据 + SQL 逐字段一致（队长关注点⑤）
- 4 图 SHA256 本人亲算，与审计报告完全一致且两两不同：
  - r25-01-impact-378.png = `fa24f998555ddad04850b0367777942597d9e8fde85f6c8289fcad79de0e0c31`（390×844）
  - r25-02-impact-detail.png = `ed07f5749f3ee9ec57feb01ecd8bd0b77ff96441313cc959f9b55b622367bd08`（390×844）
  - r25-03-old-empty.png = `6fc54cc3b1af2b56c88c91ef73280e6729483c52b3b6dfbb4cd65bc717fa38b5`（1258×588）
  - r25-04-old-empty-scroll.png = `375274166220ffa64ea4eaecffda8b224256c3156b3caf7351824ea8aa5a3033`（1258×588）
- 4 图全部亲读：均为「我的战报」页非登录页；r25-02 决定性显示聚合卡三行「蒸汽爆发 5239.0 伤害 39.8% 权威23次 事件23条 / 超导 4472.0 伤害 34.0% 权威27次 事件27条 / 过热 3438.0 伤害 26.1% 权威19次 事件19条」+ AI 四判据+诚实行+回放哈希 7468252a57f6befe；r25-04 显示 339 逐字空态+回放哈希 44faed4a07a3593b。
- 独立只读 SQL 复核 QA 库（不经 t196 转述）：378 按 reaction 聚合 `steam_burst|5239|23`、`superconduct|4472|27`、`overheat|3438|19`，与 DOM 三行伤害、占比、权威次数、事件条数逐字段一致。
- 探针两态 `pe/ce/af/fr` 全 0、`loginPage=false`。说明：probe 的 `detail` 数组是页面加载过程的历史响应序列（末项 battle_id 与当前展开行不一定相同），决定性证据为 DOM 实际渲染行，已用展开行 replay_hash 精确复核，不影响结论。

## 门禁⑦ AI 四判据 + 诚实红线逐字（队长关注点⑥）
- 源码模板 `.r25-ai-note` 四行齐备：结论文案 + 依据行逐字「依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准」+ CTA「下一局优先凑齐该元素组合 →」+ 诚实行逐字「辅助展示非伤害权威,权威以 replay_hash 为准」；容器 `.r25-impact-label` 含可见汉字「AI」。
- DOM 探针 aiTitle/conclusion/basis/cta/honesty 五项齐备且逐字一致；行为测试断言同款逐字文案与截断提示「已截断，仅统计前 200 条」。

## 门禁⑧ 零真值 + R1-R24 不回退（队长关注点⑦）
- 真值守卫：独立复跑 `go test ./internal/domain/ -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo|TestCurrentStateDocUsesDomainPlaceholder'` → **exit 0**（8.369s）；守卫从 `.local.keys` 读真值做全工作树子串比对并禁私钥路径/root@IP。
- 精确正则扫描 R25 审计目录（JWT/带口令 DSN/IP/个人路径）→ `ZERO_MATCH`；唯一曾含真值的 `auth-init.js` 已删，仅留 `auth-init.tpl.js` 的 `__LYP_TOKEN__` 占位模板；`auth-init.js` 与 `data/` 已在 `.gitignore:323-324` 拦截。SQL 复核使用的库名/回环连接为本地测试环境，非生产真值，未写入报告。
- 令牌三锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 保留，`tokens.css` 无 `--r25-`。
- 不回退：正确文件名独立复跑 R15/R20/R21/R22/R23/R24/tokens 七组契约 → **7 files / 54 tests ALL PASSED**（exit 0）；R15 时间线块在源码模板中仍完整存在，R25 仅在其后新增聚合卡，未改旧逻辑。

## 独立复跑命令汇总
| 命令 | 结果 |
| --- | --- |
| miniapp `vitest run` battles_r25_impact.contract + .test 两文件 | 2 files / 19 tests PASS, exit 0 |
| miniapp `vue-tsc --noEmit -p tsconfig.json` | exit 0 |
| miniapp `vitest run` R15/R20-R24/tokens 七组契约 | 7 files / 54 tests PASS, exit 0 |
| server `go test ./internal/domain/ -run 三真值守卫` | exit 0（8.369s） |
| 审计目录真值正则扫描 | ZERO_MATCH |
| 4 图 sha256sum | 与 REPORT 一致、两两不同 |
| QA 库只读 SQL（378/339 权威字段 + 378 聚合） | 与 DOM 逐字段一致 |

## 非阻塞备注（不影响 verdict）
1. 证据来自 agent-browser 无头浏览器 + 本地回环，非物理真机；覆盖边界已如实标注。
2. probe `detail` 数组为加载过程历史响应序列，末项 battle_id 与当前展开行可能不同；已用展开行 replay_hash 精确复核 DOM 与 SQL 一致，不影响结论。

## 最终结论
六门禁全部 PASS，队长 7 关注点逐项核验通过，未发现需修复的真实缺陷。**verdict = pass**。
