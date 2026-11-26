# R30 复查报告：390×844 反应行无溢出闭环

- 轮次：R30
- 任务：t235（执行成员 qa-e2e-wb）
- 审计目录：docs/laiyipao/audits/round30-reaction-feed-mobile-fit/
- 复查时间：2026-11-25
- 总体结论：PASS —— 七项门禁全部满足；R28 基线既有 6px 溢出已消除；长文本压力、CTA 抽屉、服务端对账与清理均闭环

## 一、结论摘要

本轮针对 t234 的移动端盒模型修复做独立复查。修复内容为：移除移动端 `.r28-attribution` 重复 `white-space` 声明，桌面/全局 `nowrap` 规则前移到 `@media (max-width: 430px)` 之前并保持逐字不变，移动端块内 `white-space: normal` 成为唯一声明；`.reaction-line` 增加 `box-sizing:border-box / max-width:100% / min-width:0`，长文本子项用 `overflow-wrap:anywhere`。

在真 Chromium 390×844 视口下实测：正常三行与长英文/无空格压力串均无横向溢出，每行 `right=380 <= 390`、`width=369 <= feed.clientWidth+1`、`feed.scrollWidth=clientWidth=369`，`docOverflow=false`；R28 基线的 `right=396 / feedOverflow=true` 已消除。CTA 真 CLI 点击可打开 R29 时间线抽屉并关闭，结算页入口同样可用，服务端 timeline 69 条逐字段对账 `MISMATCHES=0`。未发现新增缺陷。

## 二、测试环境与方法

- 浏览器：agent-browser 真 Chromium，viewport 390×844。**如实标注：这是浏览器视口模拟，不是物理真机**。
- 前端：H5 dev server；后端 Go API 指向本地回环 QA Postgres 容器（lyp-qa-pg）。
- 认证：用 `__LYP_TOKEN__` 模板 + 一次性 mint 生成 157 字符 token 注入 `window.uni.request` 劫持脚本；token 仅存在于被 .gitignore 忽略的 `auth-init.js`，复查结束已删除。
- 采集：主 E2E 会话 lyp-r30-full 逐帧采样 probe-eval；压力会话 lyp-r30-pressure 单独重做长文本注入；证据 JSON 落 data/（被忽略）。

## 三、七项门禁验收表

| # | 门禁 | 关键证据 | 结论 |
|---|------|----------|------|
| 1 | 390×844 无横向溢出，对比 R28 基线已消除 | measure.json：3 行 right=380、feed sw=cw=369、docOverflow=false；基线 right=396/feedOverflow=true | PASS |
| 2 | 真实 DOM 长文本压力可恢复、不隐藏 | stress2-applied/measure/restored：长串换行、overflow=visible、原文恢复 | PASS |
| 3 | CLI click CTA 开合 R29 抽屉、R24/R26/R27/R28/R29 共存 | cta-wait-* 与 result-open/closed：timelineOpen true→false、rowCount 4/69 | PASS |
| 4 | settle200/hash、探针零错误 | final.json：status200/battle_id478、hash 回放哈希 1c1bb0865c376e81、peReal=ce=af=fr=0 | PASS |
| 5 | ≥4 张新截图 sha 各异、亲读非登录页 | 7 张 390×844，sha 两两不同 | PASS |
| 6 | 零真值、data 忽略、QA 还原 | auth-init.js 删除、check-ignore 命中、records=104/maxid=477、零真值扫描 0 命中 | PASS |
| 7 | 报告与原始证据齐备 | 本 REPORT.md + data/ 原始 JSON | PASS |

## 四、几何量测：R28 基线 → R30 修复后

| 指标 | R28 基线（既有缺陷） | R30 修复后 | 判据 |
|------|---------------------|------------|------|
| 视口宽度 | 390 | 390 | — |
| `.reaction-feed` scrollWidth/clientWidth | 386 / 369 | 369 / 369 | sw <= cw+1 |
| 每行 `right` | 396（超视口 6px） | 380 | <= 390 |
| 每行 `width` | 386 | 369 | <= feed.cw+1 |
| 每行 `scrollWidth/clientWidth` | 384/384 | 367/367 | — |
| `.r28-attribution` white-space | nowrap（移动端重复声明） | normal（唯一声明） | — |
| docOverflow | false | false | false |
| feedOverflow | true | false | false |

133 帧主 E2E 采样二次解析统计：`states={WAIT:126, CARD_PHASE:6, SETTLED:1}`，`lineCounts` 全为 3，`feedOverflow=0`、`docOverflow=0`、`hasR24=133`、`hasR26=6`、`hasR27=133`、`hasR28=133`、`hasUndefined=0`、`hasNaN=0`。即整局对战全程无溢出，且 R24/R26/R27/R28 功能共存。

## 五、长文本压力探针（真实页面 DOM，可恢复）

在独立会话 lyp-r30-pressure 的 WAIT 态，把第一条 `.reaction-name` 与 `.r28-attribution` 临时替换为 `SUPERLONGUNBREAKABLETOKENXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789NEXT` 长串，量测后恢复原文，全程不修改产品源码。

- stress2-applied.json：`applied=true`、`feedSW=369/feedCW=369`、`docOverflow=false`、`anyOverflow=false`。
- stress2-measure.json：3 行均 `right=380/width=369/sw=367/cw=367`；长串所在归因 `attrRight=371/attrSW=353/attrCW=353`、`white-space=normal`、`overflow-wrap=anywhere`、`overflow=visible`。
- 未使用 `overflow:hidden` / `text-overflow:ellipsis` / `display:none` 等隐藏手段，长串在页面内真实换行可见（见截图 r30-05）。
- stress2-restored.json：`restored=true`，名称与归因恢复为 `焰+冰 → 蒸汽爆发` / `第1波 · #1`（见截图 r30-06）。

## 六、CTA 抽屉与结算对账

- WAIT 态：`.r28-feed-cta` 真实 CLI click → `cta-wait-post.json` `timelineOpen=true`、`rowCount=4`、`helpVisible=false`，结论 `本局显示 4 条逐次反应记录`，依据/诚实行逐字正确；抽屉 z=40、4 行 right=371/w=sw=cw=351；关闭后 `timelineOpen=false`。
- 结算态：`.r29-result-timeline-cta` 真实 CLI click → `result-open-probe.json` `state=SETTLED`、`timelineOpen=true`、`rowCount=69`、`helpVisible=false`、`result=通关`；关闭后仍停留结算态。
- 对账：`reconcile.sh` 得 `BATTLE_ID=478`、`TIMELINE_TOTAL=69`、`TRUNCATED=false`；`reconcile-full.sh` 得 `ROW_COUNT=69`、`DETAIL_TOTAL=69`、`MISMATCHES=0`、`FULL_RECONCILE_OK=true`。DOM 末三条 `第5波 · 1617 · 蒸汽爆发 · #136 · 320.0 伤害`、`第5波 · 1695 · 蒸汽爆发 · #137 · 320.0 伤害`、`第5波 · 1724 · 超导 · #137 · 337.0 伤害` 与 BattleDetail.reaction_timeline 逐字段吻合。产品 DOM 无 `data-r29-index/wave/enemy` 属性（探针读 null），逐字段比对按文本正则解析 wave/enemy_uid/damage 完成，**如实登记此口径**。
- 结算：`settle=[{status:200,battle_id:478}]`、`hash=回放哈希 1c1bb0865c376e81（可验真）`；`pe=52` 全为 Autoplay 噪声（`The play() request was interrupted by a call to pause()`），过滤后 `peReal=0`，`ce=0/af=0/fr=0`。

## 七、截图证据（7 张，sha 两两不同，均亲读非登录页）

| 文件 | sha256 | 内容 |
|------|--------|------|
| r30-01-normal-three-lines.png | 4c933dcbc9f7bab71a26e42607010ebe86e7b335af809f38fe41092f04c2eec1 | WAIT 态正常三行 |
| r30-02-stress-longtext.png | 8838fb0e169e39efe937727daeed356aee2d7e90d10f3f4c3d53d613f02bd742 | 结算页压力那次 |
| r30-03-result-drawer.png | 2c50cc923a091b5bf214c665d93406a28cbfa98fde8cd74f211d80117717a611 | 结算 69 行抽屉 |
| r30-04-settle.png | 69d6ca261432082c1832ae8877f782c7888da32e3fc6ed15d556988c8a36ea45 | 结算页 |
| r30-05-stress-visible.png | 8cf20c1c7130e942852629c3cc7414aab385c22ac66cb165ed376a3137a0416d | WAIT 态长串换行可见 |
| r30-06-stress-restored.png | a39add54bf2da94f6396108cc40692c1a00c8a966e10a5cb9f1dbcfe67c58d56 | 原文恢复 |
| r30-07-cta-drawer.png | f073b13162159320af610056e47b5feaba1f08cb147db8ae36c9ffb5b138d805 | WAIT 态 CTA 4 行抽屉 |

7 张均为 390×844 PNG，逐张亲读确认为游戏战斗 HUD/结算页/抽屉（含中文界面），非登录页；sha256 由 sha256sum 实测，两两不同。

## 八、测试与清理

- 契约与单测：`pnpm exec vitest run --config vitest.config.ts src/pages/battle/battle_r30_reaction_feed_mobile_fit.contract.test.ts src/pages/battle/battle.test.ts` → 2 files / 76 tests ALL PASSED（2026-11-25 00:30:35）。
- 契约覆盖：SPEC 钉死根因与几何判据、battle.vue 移动端盒模型修复属性、`.r28-attribution` 移动端唯一 `white-space:normal`、长文本子项约束、禁止隐藏/截断、桌面基线未变、三端选择器与 CTA 共存、tokens 无 `--r30-`、零真值。
- 清理：一次性 `auth-init.js` 已删除（`AUTH_DELETED=YES`），模板 `auth-init.tpl.js` 仅含 `__LYP_TOKEN__` 占位符；`.gitignore` 已将本轮 `data/` 与 `auth-init.js` 列入，`git check-ignore` 命中。
- QA 库还原：`pg_restore --clean --if-exists -U postgres -d laiyipao < data/qa-before.dump`（注意不能加 `--single-transaction`，否则报 input file is too short），还原后 `battle_records=104`、`max_id=477`，与基线一致。
- 零真值扫描：JWT / Bearer / guest_token 正则在本轮审计目录 0 命中。未修改任何业务源码，未做 git 提交。

## 九、如实标注与局限

- 本轮使用 agent-browser 真 Chromium 的 390×844 视口，不是物理真机；几何结论限于该视口与渲染环境。
- 主 E2E 的结算态 CLI 点击曾受结果 sheet 遮挡，已改用结果页新增入口 `.r29-result-timeline-cta` 真 CLI click 完成；DOM eval 仅用于量测与辅助探针，不作最终证据。
- 产品 DOM 未提供 `data-r29-index/wave/enemy`，逐字段对账按文本解析完成，已在第六节说明口径。
- 未引入新的 backlog；R28 基线既有 6px 溢出经本轮修复已消除。

## 十、最终判定

七项门禁全部 PASS。本轮 t234 修复在真浏览器视口下确认消除 390×844 反应行横向溢出，长文本压力不隐藏、CTA 抽屉与跨轮功能共存、服务端对账与清理闭环。上报 captain：t235 PASS。