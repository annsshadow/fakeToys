# R8 设计终审（t85）

- 日期：2026-11-22（Asia/Shanghai）
- 角色：design-reviewer（终审评审，只判定不改业务源码；写域仅 docs/laiyipao/reviews/**；未执行任何 git 操作）
- 审阅对象：t79（地形去重+威胁标签）/ t81（规范 §1G+速查 §7E）/ t82（钳分 HUD）/ t83（星级口径+机制拐点）产物与 t84 真浏览器复查报告
- 隐私声明：真域名/IP/口令/私钥/token 一律不入库、不入对话；本报告与所读截图均为本地回环 QA 证据。
- **所有截图来自真浏览器自动化页面，均非物理真机**（H5 390×844 移动视口 / 战斗 HUD 1258×588 桌面视口）。
- 独立复核原则：不采信任何自述结论；亲自逐张读决定性截图 + 亲自 grep/read 源码与契约断言。

## Verdict: pass

六条硬门禁全部达标，无阻塞 findings。全量测试绿灯为 t81/t83 依赖报告数据，本人未重跑，如实标注；本人复核的是截图肉眼可辨性、源码事实与契约断言未弱化。

## 1. 硬门禁逐条结论

| 门禁 | 结论 | 决定性证据（本人亲验） |
| --- | --- | --- |
| ① 星级口径诚实解释（约60%满分、禁高关更容易） | PASS | stage.vue:120-129 星级卡从 selected.max_score / star_targets 读取配置，starOnePct 由 star_targets[0]/max_score 计算（:438-439），无硬编码 11941；文案写「按本关满分百分比给星…不代表高关更容易」。亲读 r8-L98 图：满分 33646 / 1★20187（约60%）；r8-L100 图：52311 / 31386（约60%），数值来自配置且随关卡变化。 |
| ② 难度曲线机制拐点标注替代 +33‰ 假洞察 | PASS | stage.vue:157 渲染 diff-milestone、:134 图例「机制拐点」，r5AiStage（:522-526）改为「第 N 关是机制拐点…难度爬升来自机制变化，不能只看分数斜率」并写依据「is_boss / 敌种 id / 地形 id」；源码无「+33‰ 本章最大跳变」结论。t83 修正敌种标记只在当前关（去重），截图 AI 建议卡为真实机制结论。 |
| ③ 钳分 HUD 实时诚实 | PASS | battle.vue:32/35/38 渲染 time-budget HUD，:446-453 注释明确「服务端 scoreCapFor 是负向闸，锚点前一直钳分中，不能说成保分窗口」，scoreTimeBudget(level, elapsedMs) 纯函数来自配置。亲读 r8-battle-clamp-hud.png：顶部「钳分中·距满分 130s」与「距 1 星 11938」并排，战场 Canvas 完整可见不遮挡；t84 实测 131s→122s 随战斗时钟递减。 |
| ④ t79 两缺陷修复（地形索引去重 + 敌人组成威胁标签） | PASS | stage.vue:573-584 威胁标签按敌种输出（首领/飞行/远程等）；t84 报告敌人组成实测渲染 [远程][飞行][首领]，地形按唯一关卡去重，均不回退。 |
| ⑤ AI 四判据在关卡页可见（可见 AI 汉字+真实数据结论+依据+CTA） | PASS | 亲读 L98 图：AI 建议卡「第 98 关机制拐点·本关敌种 游荡者…难度爬升来自机制变化」+ 依据行（关卡曲线/地形/星数、is_boss/敌种id/地形id）+ CTA「先去装备页补强」四判据齐全；L100 图为「首领关·首领血量与技能机制改变打法」。战斗页本轮改动为钳分 HUD（非 AI 卡），文案诚实、依据为配置字段，无虚构 AI 结论，不违反 §1G.4。 |
| ⑥ R1-R7 不回退 + 真值零泄露 | PASS | t84 报告专项五项不回退（新手引导/攻击力诚实文案/AoE25%/成长页两步确认/admin 顶栏脱敏）全在；双端令牌锚点亲验 miniapp tokens.css 含 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6，无 lyp-tokens-v4、无 r8 新视觉变量；双端 r8LevelCurve 契约断言真实未弱化；t84 文本+截图二进制真值扫描 0 命中。 |

## 2. 本人独立复核方法

- 逐张读图（file_path + sha256 亲验，均非物理真机）：
  - r8-L98-star-rule-milestones.png（390×844，sha f4989511…）：星级口径卡 + AI 拐点建议肉眼可辨。
  - r8-battle-clamp-hud.png（1258×588，sha c255daba…）：钳分 HUD 与星级差额并排、无遮挡。
  - r8-L100-boss-fresh.png（390×844，sha 228566a3…）：满分 52311 / 1★31386、AI 首领拐点结论。
  - r8-growth-skills-logged.png（390×844，sha b519b148…）：诚实空态「本地规则引擎没有真实推荐，不调用收益预估，不展示虚构数值」，R7 成长页结构不回退。
- 源码事实（grep/read 亲验）：laiyipao/miniapp/src/pages/stage/stage.vue:119-129/438-439/522-526/573-584；laiyipao/miniapp/src/pages/battle/battle.vue:32-38/446-453/350。
- 契约断言亲验：laiyipao/miniapp/src/styles/r8LevelCurve.contract.test.ts 五例（§1G 五条款、非单调满分解释、机制拐点拒绝并列跳变、时间预算来自配置、令牌只保留既有锚点）；laiyipao/admin/tests/r8LevelCurve.contract.test.ts 六例（含 AI 四判据与双端令牌锚点），断言均为包含/正则匹配真实检查，未见空断言或弱化。
- 规范文档亲读：docs/laiyipao/design/AI_VISUAL_SYSTEM.md §1G.1-1G.5（docs/laiyipao/design/AI_VISUAL_SYSTEM.md:585-630）与速查 §7E 存在。

## 3. 诚实口径说明（非缺陷，不阻塞）

1. 本轮以 after 证据图 + round8-stage/README.md 记录的 before 基线（裸门槛 + +33‰ 假洞察）作前后对照；未在单一真浏览器会话内同采前后双图。after 图肉眼可辨、差异成立，判定门禁④⑤达标。
2. 规范 §1G.3 文本措辞为「剩余保分窗口 / score_full_at_sec」，而 t82 实现因服务端 scoreCapFor 为负向闸，采用更诚实的「钳分中·距满分 Xs / 即将免钳分 / 已免钳分」。实现方向比规范文本更诚实、不误导玩家，且契约同时钉住两者，不构成功能缺陷。

## 4. 低优先级观察（非阻塞，不列 findings）

- O-R8-1（low）：建议后续规范版本将 §1G.3 的「保分窗口」措辞与实现「钳分倒计时」对齐，避免新读者按旧语义误解；当前不影响玩法、诚实性与 R1-R7 不回退。

## 5. 结论

Round8 四项改造（星级口径诚实解释、机制拐点替代假洞察、钳分 HUD 实时诚实、t79 两缺陷不回退）在真浏览器与源码层均达标，AI 四判据在关卡页可见，R1-R7 不回退、真值零泄露。verdict=pass，放行下游提交，无需要修复的阻塞问题。
