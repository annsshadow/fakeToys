# R44 真浏览器复查报告：R22 只读 CTA 真实点击无副作用

- 任务：t52（attempt 305ce381-d0f4-43ac-b9fa-71b2b4560a8c）
- 审计目录：docs/laiyipao/audits/round44-r22-cta-honesty/
- 依赖：t49 侦察、t50 规范与契约、t51 实施（仅改 laiyipao/miniapp/src/pages/battle/battle.vue:364 一行文案）
- 规范：docs/laiyipao/specs/round44-r22-cta-honesty/SPEC.md（9 节）

## 一、结论

**PASS（带诚实边界）**：L5 真实战斗进入选卡协同预判阶段后，R22 新 CTA 文案逐字为「↑ 点上方卡牌即可装备它 →」。真实底层鼠标事件点击该 CTA 后，href 仍为 battle 深链、无帮助层/时间线/暂停、手牌与预判内容完全一致；正对照真实点击手牌卡牌仍正常 takeCard 装备并把战斗推进到下一波。自动化门禁全绿。第三、六节如实列出无效证据与未验证项，均不作为 PASS 依据。

## 二、范围与约束（已遵守）

- 未修改任何业务源码；未执行任何 git 写操作（无 add/commit/checkout）。
- 仅新增审计目录产物；本轮无需再改 .gitignore（已有规则命中 docs/laiyipao/audits/round44-r22-cta-honesty/data/）。
- 浏览器交互由真实 Chromium 经 agent-browser CLI 完成；关键 CTA/卡牌点击使用底层鼠标事件 mouse move / mouse down left / mouse up left，非 DOM eval 派发点击。
- 脚本以 git rev-parse --show-toplevel 推导仓库根，不写死绝对路径。
- 令牌注入文件由运行时生成、置于 gitignore 数据目录，报告与脚本不含字面凭据。

## 三、环境与进入真实战斗态

- 复用已健康服务：API :8080 /api/v1/config HTTP 200，H5 :5174 HTTP 200。不重启、不 kill 服务 job；避免端口漂移导致 chrome-error。
- 会话未设置 QA_ADMIN_USER/QA_ADMIN_PASS；通过 QA 库未过期 guest_token_aliases 别名在回环 API /api/v1/auth/guest 换 user10（max_stage=60）的令牌，open 阶段注入 lyp_token / lyp_refresh / lyp_token_exp / lyp_profile / lyp_guest / lyp_battle_speed=8；令牌真值不落报告。
- 视口 390×844，先 open 再 set viewport；有效闭环探针 innerW=docW=390，href 始终为 http://127.0.0.1:5174/#/pages/battle/battle?level=5。
- R22 预判只在 engine.phase 为 card_select 且有已装备元素与带元素手牌时出现。初始波次态探针 hasR22cta=false 是状态未到，不是缺陷；driver 等待 .sheet/.hand-card/.r22-ai-cta 三者同时出现后才点击。
- 关键证据文件：data/closed-before.json、data/closed-cta-after.json、data/closed-card-after.json；点击日志 data/closed-cta-click.txt 与 data/closed-card-click.txt 均含 MOVE + 3 行 ✓ Done（move/down/up）。

## 四、闭环逐字段对账（最终有效证据）

| 字段 | CTA 点击前（closed-before） | CTA 点击后（closed-cta-after） | 正对照点卡后（closed-card-after） |
| --- | --- | --- | --- |
| href | http://127.0.0.1:5174/#/pages/battle/battle?level=5 | 同样，未跳页 | 同样，未跳页 |
| r22-ai-cta 文案 | ↑ 点上方卡牌即可装备它 → | 同左（逐字不变） | 同左 |
| r22 结论 | 当前手牌「燃烧弹」可强化已有【蒸汽爆发】 | 同左 | 同左（新卡集仍显示） |
| 手牌 | 燃烧弹(焰)/加固工事/连锁导引 | 燃烧弹(焰)/加固工事/连锁导引（完全一致） | 干冰弹/元素容器/连锁导引（takeCard 生效） |
| helpOverlay / hasTimeline | false / false | false / false | false / false |
| hasPause / hasResume / pauseLabels | 0 / 0 / [] | 0 / 0 / [] | 0 / 0 / [] |
| clock（距满分） | 131s | 131s（card_select 阶段引擎 tick 冻结，见第六节） | 116s（战斗推进到第 2 波） |
| 波次 / bodyHead | 波次 1/5、击杀 3、钳分中 | 同左 | 波次 2/5、击杀 8、钳分中 |
| 视口 innerW/docW | 390 / 390 | 390 / 390 | 390 / 390 |

判定：真实鼠标点击 CTA（坐标 197,783）后，页面没有出现帮助图例、时间线、暂停按钮，手牌、预判结论、CTA 文案逐字不变；正对照真实点击第一张卡（坐标 70,523）后手牌更换、波次推进、计时恢复，证明卡牌装备主路径未被破坏。R22 CTA 的点击确实无副作用。

### HUD 共存（同一个 before 探针）

hudCoexist：r22=true、r24=true、r26=true、r27=true、r28=true、r29=false。R29 只在结算态出现，当前为战斗态故为 false，非回归。bodyHead 还可见 R23 波次进度、R24 元素预判、反应日志、钳分等既有元素；R43 的 r27/r24 文案仍在场。

### 源码只读核对

- laiyipao/miniapp/src/pages/battle/battle.vue:364 仅一行改为新文案，无 @click；:2756 的 .r22-ai-cta 规则只有 color/font，无 pointer-events:auto。探针看到 r22CtaPE=auto 是 getComputedStyle 默认值（父层未设 pointer-events:none），不是源码声明违约；契约断言的是源码规则内不出现 pointer-events:auto，已满足。
- r27-ai-cta(:58)、r24-ai-cta(:95) 仍无 @click；r28-feed-cta 与 r26/r27 原样保留；未触碰 openHelp、takeCard 引擎逻辑。

## 五、门禁结果

- R44 契约（battle_r22_synergy.test.ts + battle_r22_synergy.contract.test.ts）：2 文件 / 20 测试通过，exit 0（R44_CONTRACT_RC=0）。
- battle.test.ts：61 测试通过，exit 0（BATTLE_RC=0）。
- pnpm run type-check：exit 0（TYPECHECK_RC=0）。
- pnpm run build:h5：exit 0，DONE Build complete（BUILD_H5_RC=0）。
- miniapp 全量 pnpm test：Test Files 212 passed / 1 skipped，Tests 1993 passed / 2 skipped，exit 0（FULL_TEST_RC=0）。
- Go 守卫：TestNoInfraIdentifiersInRepo PASS；TestNoRealSecretsInRepo **SKIP（非 PASS）**——缺席真值文件 laiyipao/deploy/.local.keys，守卫原话为「读不到 WORKTREE_ROOT/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是通过了」。该守卫未计入「全绿」。

## 六、假阳性、无效证据与诚实边界

1. 端口漂移错误页：r44-click3/final2 等尝试中，H5 因端口占用漂到 5175/5176/5178，浏览器落到 chrome-error://chromewebdata/（bodyLen≈233、clock=null）。data/final2-cta-after.json、final2-card-after.json、click3-before.json 等错误页探针与截图全部作废，未用于判定。队长明确要求任何 chrome-error 即本次无效重来；最终采用 closed 系列的同一有界会话闭环。
2. 空白/异常小图：r44-alias-*.png、r44-final-*.png（2740 字节或错误页）不计入有效截图；r44-closed-before.png 与 r44-closed-cta-after.png 字节相同是「点击无视觉副作用」的预期结果，不冒充互异图。
3. 选卡阶段计时冻结：base-1…base-6 每 2 秒探针均为 clock=131、onCardSelect=true、r22cta 可见，证明引擎在 card_select 阶段冻结 tick（engine.ts:1122-1141）。因此 CTA 点击前后 clock 不变不是暂停副作用；退出选卡后正对照 clock 131→116、波次 1/5→2/5，战斗继续推进。
4. r22CtaPE=auto 为 computed 默认值，不是源码 pointer-events:auto（见第四节源码核对）。
5. R29 在战斗态不可见属于阶段特性，未单独验证结算态 R29 CTA 点击；本轮结论限于战斗态只读 CTA 无副作用。
6. TestNoRealSecretsInRepo 为 SKIP，未计入自动化门禁通过数。

## 七、截图 sha256（有效互异、非登录页）

| 文件 | sha256 | 说明 |
| --- | --- | --- |
| screenshots/r44-root-card.png | c290ef2ac6b92611190e45a061822a2a10447056bde5dc14e54f718bbd2422f1 | 早期有效 R22 选卡态截图 |
| screenshots/r44-valid-before.png | d4aaf781838a02146d947a68f563743d21ad8736bf53c074a817481d7c5879c6 | 有效 L5 战斗态截图 |
| screenshots/r44-initial.png | 736af972188034274a7460ac0372286e20585f77bbde82fea3d730c33fbd805a | 有效非登录态截图 |
| screenshots/r44-closed-before.png | 6e5f7d477b7cfb25512e84f059ab11ccc2cf4401fa5e0fa140a6e38881ca3afc | 闭环 CTA 点击前 |
| screenshots/r44-closed-card-after.png | f41f0bd70de58b8cebbb1974dd4347f3459c1c09116af9dbf46adfd198177d6f | 正对照点卡后 |

≥4 张互异有效截图满足要求。r44-closed-cta-after.png 与 closed-before 同 sha（无视觉变化），不计入互异计数；alias/final/click3 等错误页/空白图见第六节剔除。

## 八、密钥与清理

- 令牌注入脚本 data/auth-init.js 运行时生成，置于 .gitignore 命中目录；报告与审计脚本不含字面凭据。
- 收尾删除 data/auth-init.js；QA 库 battle_records 复核为 count=104 / max(id)=477。
- git check-ignore -v 命中 .gitignore 对审计 data/ 的规则。本 QA 任务未新增业务源码改动；工作树中已有的 battle.vue / 契约测试 / round22 SPEC 改动来自依赖任务 t50/t51，不是本复查产生。
- 静态密钥扫描审计目录文本文件无字面 JWT / Bearer / 带口令连接串 / 本机绝对路径命中。
- 最终仅保留自身浏览器会话清理；服务 job 属共享健康服务，未 kill。
