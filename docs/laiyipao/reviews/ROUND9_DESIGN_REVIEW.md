# R9 设计终审（t93）

- 日期：2026-11-22（Asia/Shanghai）
- 角色：design-reviewer（终审评审，只判定不改业务源码；写域仅 docs/laiyipao/reviews/**；未执行任何 git）
- 审阅对象：t88（反应记录HUD+下一关入口）/ t89（Wiki路由修复）/ t90（规范§1H）/ t91（战斗因果链HUD+选卡反馈）/ t92（复查6项PASS）
- 声明：所有截图来自真浏览器自动化页面，均非物理真机（战斗HUD/因果链/选卡 1258×588 桌面视口；H5结算/战斗 390×844 移动视口；admin Wiki/命令面板 1258×588）。
- 隐私纪律：真域名/IP/口令/token/私钥不入库不入对话；本报告基于本地回环 QA 证据。
- 独立原则：不采信任何自述结论；亲自逐张读图 + 亲自 grep/read 源码、契约与测试。

## Verdict: needs_revision（终审；复审结论见文末 t96 章节，最终 verdict=pass）

六条硬门禁中五条达标（②③④⑤⑥），门禁①反应因果链本身达标（四判据齐、pointer-events:none、不遮挡战场），但独立复核发现一个高严重度真实缺陷：移动视口战斗页顶部 HUD 四个胶囊逐字竖排不可读（详见 R9-BUG-1）。该缺陷已由 t95 test-first 修复并经 t96 复审确认闭环（见文末复审章节）。

## 1. 六门禁逐条结论

| 门禁 | 结论 | 决定性证据（本人亲验） |
| --- | --- | --- |
| ① 反应因果链 HUD + pointer-events | 部分达标（见 R9-BUG-1，已复审闭环） | 桌面视口 r9c-02/r9c-03 亲读：每条含元素组合→反应名 / 伤害 / AI 因果（依据 reaction=xxx/damage=xxx + CTA 点卡凑同元素），四判据齐；battle.vue:1723 overlay-top pointer-events:none，reaction-feed 浮层不遮挡 Canvas。但 390×844 移动视口顶部 HUD 胶囊曾竖排不可读。 |
| ② 下一关入口 + 防失败谎报 | PASS | r9-07 / r9-h5-settle-nextlevel 亲读：真实通关结算出现「前往第 2 关 →」；源码 battle.vue:1084-1089 canGoNextLevel = win && !settleFailed && 下一关已解锁 && 关存在；:322 v-if 绑定，失败态不显示。 |
| ③ Wiki 路由修复 + AI 四判据 + 防伪装 | PASS | after-01-wiki-fixed 亲读：title 玩家玩法文档站 + kicker 玩法知识库/PLAYER GUIDE + AI 卡四判据（AI 徽章 + 真实结论「配置版本 6237e0b…反应条目 7 类」+ 依据「累计战报 168count/胜场 131count/版本来自 /config」+ CTA「查看内容 →」）；router/index.ts:34 /wiki 路由在、:35 兜底重定向在；LoginView.test.ts R9-WIKI 用例断言 /wiki.evil.example.com 前缀开放重定向被拒。 |
| ④ 选卡反馈来自真实事件 + 与钳分/星级口径一致 | PASS | r9c-03 亲读「已取卡 1 张」；battle.vue:515-539 选卡/结算反馈来自 card_picks 与 hit/reaction 事件，ratioPct 为 null 时诚实写「本局无完整伤害记录」，不编造；钳分文案 battle.vue:446-453 负向闸诚实语义，与 R7 星级差额同源配置，无各说各话。 |
| ⑤ R1-R8 不回退 | PASS | t92 §4 十一项清单 + 本人读图：新手引导/攻击力诚实文案/AoE25%/成长页两步确认/admin 顶栏脱敏/推荐空态诚实/星级口径来自配置/机制拐点/钳分倒计时/地形去重/威胁标签全在；双端令牌锚点 v1/v3-r3/r6 无 v4 无 r9 新变量；双端 r9BattleFeedback 契约断言真实未弱化。 |
| ⑥ 真值 0 泄露 + 截图证据有效 | PASS | t92 文本+13 PNG 二进制真值扫描 0 命中；本人亲读 7 张决定性图 sha 全部各异（无与登录页同 sha 的伪造），r9-admin-palette-fixed.png 命令面板真实打开（AI洞察分区 3 项、ariaModal=true）。 |

## 2. 阻塞 Finding（已修复，见文末复审）

### R9-BUG-1【high】移动视口战斗页顶部 HUD 胶囊逐字竖排、完全不可读

- 问题：390×844 移动视口战斗页顶部 HUD 第一行四胶囊「波次 1/5」「击杀 1」「距 1 星 11432」「钳分中·距满分 128s」逐字竖列，玩家无法读取波次、击杀、距下一星分数与钳分倒计时。证据：docs/laiyipao/audits/round9-after/screenshots/r9-h5-battle.png（sha 2cc968564d…）。对照桌面视口 r9c-02（1258×588，sha 713bfc1e…）同一 HUD 横向正常。
- 影响：R7 星级差额、R8 钳分倒计时两轮核心 HUD 交付在手机主游玩场景不可读。t92 复查战斗页全部使用 1258×588 桌面视口，系统性遗漏移动视口。
- 根因（源码）：battle.vue:1727 .hud-row flex-direction:row 且 gap:12rpx，:1739 .hud-pill inline-flex 但无 flex-shrink:0、文字无 white-space:nowrap；叠加后 390rpx 被压缩换行。
- 修复要求：test-first RED + 样式修复 + GREEN + 390×844 真浏览器截图 + 回 design-reviewer 复审。t95 已按此完成，t96 复审确认闭环。
---

## 复审章节（t96，R9-BUG-1 修复复审）

- 日期：2026-11-22（Asia/Shanghai）
- 任务：t96 复审 t95 对 R9-BUG-1（移动视口战斗 HUD 胶囊竖排不可读）的 test-first 修复。
- 依赖：t95 修复报告 docs/laiyipao/audits/round9-hudfix/README.md + 两张 390×844 真浏览器截图。
- 独立原则：不采信自述；本人亲读截图 + 亲自核实源码断言与令牌/语义事实。

### 复审 Verdict: pass

R9-BUG-1 已闭环：移动视口战斗 HUD 恢复单行可读，反应因果链仍可读、战场不遮挡，R1-R8 与 R9 其余五门禁不回退，真值 0 泄露。放行下游提交。

### 逐项判定

| 复审项 | 结论 | 决定性证据（本人亲验） |
| --- | --- | --- |
| ① 390×844 新截图四胶囊单行可读 | PASS | 亲读 r9hud-01-mobile-hud.png（sha 19cdae6286…）、r9hud-02-mobile-combat.png（sha 843322e69b…），均 390×844 非物理真机、非登录页、sha 与旧图 2cc96856… 均不同：第一行「波次 1/5 / 击杀 3 / 距 1 星 10423」横向单行，第二行「钳分中·距满分 125s」单行；不再逐字竖排。 |
| ② 因果链仍可读 + Canvas/技能槽不遮挡 | PASS | 两图亲读：三条反应因果链「焰+冰→蒸汽爆发 -222」等含 AI 依据（reaction=steam_burst/damage=222）仍可读；战场 Canvas、热量条、技能槽、选卡卡面完整可见，HUD 浮层只占顶部、不遮挡操作。 |
| ③ 源码修复真实 + RED 证据 | PASS | battle.vue:1730 .hud-row 加 flex-wrap:wrap、:1744 .hud-pill flex-shrink:0、:1748 white-space:nowrap、:1820 .time-budget-text nowrap、:1849 @media(max-width:430px) 收缩布局；test-first 用例 battle.test.ts:1357-1381 五条正则断言真实（flex-wrap/flex-shrink/nowrap/@media/隐藏spacer 逐项 toMatch），t95 README 记录 RED 逐字失败 expected ... to match /\.hud-row...flex-wrap/ 与 GREEN 通过，断言能失败非弱化。 |
| ④ R1-R8 与 R9 其余五门禁不回退 | PASS | 令牌锚点亲验双端仍 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6、无 v4 无 r9/r8 新变量；battle.vue:479-480 钳分负向闸诚实文案在、:29 liveStarGap 星级差额在、:42 reaction-feed 在、:322 + :1084-1087 canGoNextLevel(win&&!settleFailed)防谎报在；t95 仅改 battle.vue 与测试，未碰 admin/后端/stage。 |
| ⑤ 真值 0 泄露 | PASS | round9-hudfix 目录 grep JWT/私钥/真实域名模式 0 命中；两张新图二进制经 t95 声明零泄露，结合本人文本扫描无异常。 |

### 复审结论

R9-BUG-1（high）已按 finding 要求 test-first 修复并经 390×844 真浏览器复审确认闭环。R9 六门禁现全部达标。verdict=pass，放行下游提交。全量 1699 passed 与双类型检查绿为 t95 依赖报告数据，本人未重跑，如实标注；本人复核聚焦截图可读性、源码断言真实性与不回退事实。