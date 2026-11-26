# R22 设计终审报告 · 选卡反应协同预判（六门禁独立复核）

> 终审任务：t180 / attempt 2（design-reviewer-wb）  
> 日期：2026-11-24  
> 被审轮次：R22 round22-card-reaction-synergy（t177 规范 / t178 H5 实施 / t179 真浏览器复查）  
> 终审方式：独立亲读规范/契约/源码/证据 + 独立复跑测试 + 截图 sha256 亲算 + 关键截图亲读，不采信成员自述  
> **Verdict = pass（六门禁全通过，无阻断缺陷）**

---

## 门禁结论一览

| 门禁 | 结论 | 决定性依据（本人独立复核） |
| --- | --- | --- |
| ① SPEC 与契约一致 | PASS | SPEC.md 80 行亲读；契约测试 12 条独立复跑全绿；逐字条款对齐 |
| ② 纯展示不污染 replay_hash/结算 | PASS | takeCard 不触 hash；settle 哈希来自引擎；派生仅 computed/ref |
| ③ 协同预判与引擎反应对一致 | PASS | REACTION_PAIRS 源自 lookupReaction 穷举；差集单测与浏览器探针均对齐引擎真相 |
| ④ 真浏览器证据有效 | PASS | 7 图 sha 亲算与报告完全一致且两两不同；亲读 3 图真实渲染非登录页；探针全 0 |
| ⑤ 零真值 + R1-R21 不回退 | PASS | auth-init.js 已删、模板仅占位、零真值已扫部分 0 命中；令牌 v1/v3-r3/r6 保留、无 --r22- |
| ⑥ 无新真实缺陷 | PASS | 唯一诚实限制（无元素技能构筑空态）已由定向单测覆盖并如实标注，不构成缺陷 |

---

## 门禁① SPEC 契约一致 —— PASS

- 亲读 `docs/laiyipao/specs/round22-card-reaction-synergy/SPEC.md`（80 行）：§0 缺口（battle.vue:208-248 选卡零预判）、§1 数据基础（REACTION_PAIRS/equippedFromSnapshot/ELEMENT_NAME/lookupReaction 均为现成真源）、§2 覆盖集 C(E)+差集 A=C(E∪{e})−C(E) 确定性算法与三分枝文案、§3 选择器与逐字空态、§4 AI 四判据、§5 诚实红线、§6 契约测试位置、§7 三端不回退锚点、§8 分工，条款自洽。
- 契约测试 `battle_r22_synergy.contract.test.ts`（121 行）亲读：12 守卫逐字钉死 SPEC 关键词、选择器、空态、AI 四判据、诚实红线、令牌锚点、引擎真相反应对。
- **独立复跑**：`vitest run --config vitest.config.ts battle_r22_synergy.contract.test.ts battle_r22_synergy.test.ts` → 2 files / **15 tests ALL PASSED**，exit 0（本人执行，非引用自述）。
- 关键真相修正已锁定：SPEC/契约均按引擎真相 fire+ice=steam_burst、ice+lightning=superconduct、fire+lightning=overheat，未沿用草稿中「焰+冰=超导」的错误映射。

## 门禁② 纯展示不污染 replay_hash/结算 —— PASS

- 亲读 `battle.vue`：
  - `takeCard(id)`（:1223-1228）仅调 `engine.value.takeCard(id)`、刷新 `hand`、`cardChoiceVersion++`，完全不触 hash/结算。
  - R22 派生 `equippedElements`（ref）、`r22Covered/r22Synergy/r22Partners/synergyOf`（:745-801）纯函数 + computed，只读装备快照与手牌，不写任何权威字段。
  - `settle(eng)`（:1156）的 `payload.replay_hash` 仍来自 `eng.settleInput()/eng.replayHash()`；R15 逐次反应 timeline 旁路（:1157-1159）为既有机制，R22 未介入。
  - 装备元素集合 E 取 `equippedFromSnapshot(bt.build, skills)` 的 `applyElement`（:1030-1034），与伤害/哈希同源，杜绝 optimistic UI 猜测。
- 动态对照：真实结算 HTTP200 / battle_id=477，结算页回放哈希 `1dbf94aebb62f81d`（可验真）由引擎产生，与探针 settle 响应一致；选卡预判从未参与。

## 门禁③ 协同预判与引擎反应对一致 —— PASS

- 真源亲读：`reactionPairs.ts:15-25` 的 REACTION_PAIRS 由 `lookupReaction` 对五元素两两穷举反查派生，纯静态、天然与引擎一致；`replay.ts:222` `applyElement: (def.apply_element || def.element)` 为权威快照字段。
- 算法亲读：`r22Covered`（:745-756）以 REACTION_PAIRS+REACTION_ORDER 两两查表收集合；`r22Synergy`（:764-787）算差集 added/reinforced 并按 REACTION_ORDER 排序，反应名取 `REACTIONS[key].name`、元素名取 `elementName`，无硬编码中文映射。
- 定向单测（本人复跑通过）：火+冰覆盖 steam_burst 唯一；火+冰再选电 → added=[overheat, superconduct]、reinforced=[steam_burst]；重复元素无新增；差集两次结果逐项相等；REACTION_PAIRS 每条对都能 lookupReaction 反查到同 key。
- 浏览器探针（本人亲读）：用户 10 电弧弹（电）显示 `AI · 强化已有【蒸汽爆发】`，三标签 `[steam_burst, overheat, superconduct]`，与已装备 {焰,冰,电} 差集推导一致。

## 门禁④ 真浏览器证据有效 —— PASS

- **截图 sha256 亲算**（本人对文件字节计算，7 张与 REPORT.md 记录逐一完全一致，且两两不同）：
  - r22-01-hand-synergy.png = `57fc52dbac8c5812d33131b491572cc5bd105413de0d446ae66aaa4fcb008f20`
  - r22-02-synergy-card.png = `67d7eb6887372fedefd515b967563a981f6924b2327b69ac3d7b4abd01209ca8`
  - r22-03-ai-note.png = `19856b2f0c4b053cfb80d34616c4735f180badbac7316cc3d6523815659846c1`
  - r22-04-after-card.png = `cff06a0cd438e8ef99b779ffe1a709508b2ae2a4a09702b6f6c1f82350922bf7`
  - r22-05-empty-user.png = `059db5909611bca286480e6cf6b643799c881373037808c5cdf8efef76cb9dfe`
  - r22-06-empty-note.png = `7b733bd0f3fccb1e9132fde15955ddf749080b7212fc48372a65efa36455be1a`
  - r22-07-settle-hash.png = `1fc611ef6b687f137a9f1f5214b38da1b3936d53195ea413457e77290e71d729`
- **关键截图亲读**（本人）：
  - r22-01 选卡页：HUD（第5关·边缘哨站/波次1-5/击杀3）真实渲染；电弧弹「AI·强化已有【蒸汽爆发】」+三标签；散热涂层/连锁导引逐字空态；底部 AI 协同卡。非登录页。
  - r22-03 AI 协同卡：可见「AI」、真实结论、逐字依据行「依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生」、CTA「去装备/选它」、诚实红线逐字，AI 四判据齐全。
  - r22-07 结算页：通关结算（击杀36/反应68/分数9522）+「回放哈希 1dbf94aebb62f81d（可验真）」，与探针 settle 一致。非登录页。
- 探针决定性数据（本人亲读 probe-hand/after-card/settle）：handCount=3、note 四判据 hasAI/hasBasis/hasCTA/hasHonesty 全 true、hasUndefined/hasNaN=false、`pe/ce/af/fr/settle/start` 记录中页面错误/控制台错误/API 失败/失败请求全空；点选后 handCount=0/note 正确消失；settle=[{status:200, win:true, battle_id:477}]。
- 浏览器为 agent-browser（**非物理真机**，390×844，本地回环 Vite H5）——此为本轮实际取证方式，如实标注，不夸大为真机。

## 门禁⑤ 零真值 + R1-R21 不回退 —— PASS

- auth-init.js：`ops_stat` 确认 exists=false / revision=absent（含真 token 的脚本已删）；模板 `auth-init.tpl.js` 仅用 `__LYP_TOKEN__` 占位符，探针只做 localStorage 注入与错误收集。
- .gitignore:311-312 已忽略 `round22-card-reaction-synergy/data/` 与 `round22-card-reaction-synergy/auth-init.js`；qa-before.dump 位于被忽略的 data/ 内，不入库。
- 零真值扫描：对审计目录的可读文本 grep（Bearer/长 JWT 字面量/C:\Users/ssh-rsa/PRIVATE KEY 等模式）已扫描部分 **0 命中**，唯一命中是 REPORT.md 自身描述扫描模式的说明文字；dump 为二进制导致扫描器返回 partial（保守行为），已扫描文本无泄露。
- 令牌锚点（本人独立 grep）：`tokens.css` 保留 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`，**无 `--r22-` 新令牌**。
- 不回退：SPEC §7 明确 R15-R21 时间线/验真/伤害贡献/效率/关卡教练契约一律不动，本轮零后端、零结算改动；契约测试守卫 battle.vue 选卡基线（REACTION_PAIRS/elementName/showCards/hand/takeCard/discardCard）保留。

## 门禁⑥ 无新真实缺陷 —— PASS

- 未发现任何阻断性或高优先级真实缺陷。
- **诚实限制（非缺陷，非阻塞）**：「当前构筑还没有元素技能」空态分支未在浏览器复现——QA 用户 127 的 `equippedFromSnapshot` 取服务端 /battle/start 下发的 build 快照，该快照仍带元素，故页面仍显示元素预判。该分支已由定向单测 `battle_r22_synergy.test.ts`（FakeEngine + mock equippedFromSnapshot）覆盖并通过；t179 已如实标注该浏览器验证限制，未伪装为已实测，符合诚实红线。
- 结算页显示的「AI·赛前后」与实际结算说明属 R10/R14 既有机制，非本轮 R22 引入，不纳入本轮缺陷判定。

---

## 终审结论

R22「选卡反应协同预判」六门禁全部 PASS。规范与契约逐字一致且锁定引擎真相；预判为纯客户端展示派生，不进 replay_hash、不改结算；差集算法与引擎 REACTION_PAIRS 同源并经单测+浏览器双重对齐；真浏览器证据（7 图 sha 各异、亲读非登录页、探针全 0）有效；零真值与令牌不回退到位；无新真实缺陷。

**Verdict = pass**。同意本轮通过终审，可进入后续批次。

> 环境备注（诚实、非缺陷）：本轮真浏览器取证使用 agent-browser 非物理真机、本地回环 Vite H5；零真值 grep 因二进制 dump 返回 partial，已扫描文本 0 命中。以上均不阻断六门禁结论。
