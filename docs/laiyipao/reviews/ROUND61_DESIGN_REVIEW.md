# R61 独立终审：aria-label 桥接修复可发布性

- 角色：same-route-reviewer（laiyipao-r36 专职独立终审）
- 终审对象：t112 实施（R61 表单输入 aria-label 运行时桥接修复）
- 链：t100 侦察 → t108/t111 规范+契约 → t112 实施 → t113 真浏览器重查 → 本轮独立终审
- **verdict = PASS（可发布）**

## 0. 独立性声明
本轮未参与 R61 规范/实施/QA 任一执行，全部结论基于独立亲读 t111/t112/t113 产物、独立复跑全部门禁命令、独立 sha256 亲算与 auth/QA 库零真值核实。只读取证，未改业务源码、未做 git 写操作；唯一新增文件为本报告。

## 1. 六维门禁总表

| 维度 | 结论 | 关键证据 |
|---|---|---|
| ①test-first 真实性 | PASS | SPEC §7 两轮 RED：首轮 2failed|2passed，t111 修复轮 RC=1 / 1failed|4passed（314ms），唯一失败为 it5「17 处 input 未挂 v-aria-native」缺口断言；无 Transform failed / 模块缺失 / 超时；本轮 GREEN 5/5 |
| ②契约与实现对齐 | PASS | 契约 155 行 5 it 真实 node:fs 读源码、无内嵌实现；17 处 v-aria-native 与 main.ts 指令桥接逐字对应；原属性零删除 |
| ③保护面零改 | PASS | tracked 14 文件均为展示层/配置；engine/replay/hash/score/settle/loot/tokens/后端零改；令牌三锚点原样、无 --r61-；R43/R44/R48/R55 锚点保持 |
| ④门禁独立复跑 | PASS | 契约 R61+R60 11/11、全量 2025 passed|2 skipped（218 files|1 skipped）、vue-tsc RC0、build:h5 RC0、git diff --check RC0、Go TestNoInfra PASS；TestNoRealSecrets SKIP 如实非 PASS |
| ⑤QA 浏览器铁证 | PASS | 37 组三视口真 Chromium：native input DOM 直带 aria-label、AX 树 textbox/spinbutton 逐字带名、前后对照坐实桥接；反馈播报 5/6 处真机坐实；4 图 sha 亲算互异并亲读非登录页 |
| ⑥诚实边界与零真值 | PASS | auth 四路径 ABSENT、gitignore:553 命中、QA 库 104|477；SKIP/未捕获项/假阳性更正/会话侧 mock 均充分披露 |

## 2. 维度一：test-first 真实性
SPEC（docs/laiyipao/specs/round61-form-a11y/SPEC.md，125 行）§0 记录真实缺口：H5 编译后每个 input.uni-input-input 的 aria-label/labels 全空、placeholder 属性为 null（占位由兄弟 DIV.uni-input-placeholder.ph 承载）、AX 树裸 textbox；反馈节点 errMsg/redeemMsg/passwordMsg/form-hint 无 role 不播报。§1A 根因分析有 @dcloudio/uni-h5.es.js:10856-10886 原生 input 白名单证据，静态 aria-label 被丢弃、setAttribute 后 AX 立即带名的对照实验支撑修复口径。
RED 两轮实测见 SPEC §7：首轮（t108）RC=1 / 1 failed file / 2failed|2passed / 337ms；修复轮（t111）RC=1 / 1failed|4passed / 314ms，唯一失败 it5 断言 17 处 input 未挂 v-aria-native，其余 it1-it4 全过，无解析/模块/超时类假失败。原始 RED 日志未随交付留存，依 SPEC 记录 + 契约结构 + 本轮 GREEN 采信（诚实边界见 §7）。
契约 laiyipao/miniapp/src/styles/r61FormA11y.contract.test.ts（155 行 / 5 it）以 node:fs readFileSync + node:url fileURLToPath + node:path dirname/join 读真实源码，INPUTS 17 组、NODES 6 组；inputBlock()/tagBefore() 按 v-model 开标签与 needle 前 text/view 块截取，无内嵌参考实现。it1（17 aria-label）、it2（6 播报）、it3（R60 tabindex/@keydown、R43/R44/index CTA 与 17 处 placeholder-class=ph 锚点）、it4（令牌三锚点无 --r61-）、it5（17 v-aria-native + main.ts app.directive/querySelector/setAttribute 桥接）。

## 3. 维度二：契约与实现对齐
实施 diff（git diff --stat）14 文件 / +68/-6：.gitignore(+9)、main.ts(+19/-1)、12 个页面（announcements/bag/enemies/equipment/gems/login×2/me×5/reactions/skills/signin/skins/verify）。main.ts 新增 aria-native 指令，applyAriaNative() 对原生 input.uni-input-input 直接、对外层 uni-input 经 querySelector 取 native，setAttribute(aria-label, name)；mounted/updated 双钩子保证值变更同步；createApp 内 app.directive(aria-native, ariaNative) 注册。
ops_grep v-aria-native 于 12 页面计数合计 17（announcements1/bag1/enemies1/equipment1/gems1/login2/me5/reactions1/signin1/skills1/skins1/verify1），与 SPEC §2 的 17 处逐字一一对应。四关键页亲读：bag.vue:404 预设名 input 加 v-aria-native+aria-label、presetSaving 加 role=status aria-live=polite；login.vue 用户名/密码各加 v-aria-native+aria-label、form-hint 加 role=status aria-live=polite、errMsg 加 role=alert；me.vue 5 处 input 各加桥接、passwordMsg 加 role=status aria-live=polite；verify.vue battleId 加桥接、errMsg 加 role=alert。所有既有 v-model/placeholder/placeholder-class=ph/class/:class/type 属性零删除。

## 4. 维度三：保护面零改与不回退
tracked 14 文件全部为展示层与配置；engine/replay/hash/score/settle/loot/tokens 与后端零改。tokens.css:12-14 三锚点（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）原样。全 src 扫 --r61- 仅契约 2 处否定断言（contract:128/:133），业务源码 0 命中。契约 it3 与 QA POSTFIX §5 双重确认 R60 键盘可达（48 处 tabindex/@keydown）、R43 r20 CTA（class=r20-ai-cta data-r20-cta role=button @click=goStage()）、R44 r29（class=r29-result-timeline-cta @click.stop=openReactionTimeline）、index CTA（@click=go(aiInsight.hint)/@click=goStage）、R55 placeholder-class=ph 均未回退。

## 5. 维度四：门禁独立复跑（本轮亲跑）
全部为本人本轮独立执行、非采信上游：
- 契约 R61+R60 合并：cd laiyipao/miniapp && npx vitest run src/styles/r61FormA11y.contract.test.ts src/styles/r60A11yKeyboard.contract.test.ts --reporter=dot → 2 files passed / 11 tests passed，RC=0（1.05s；job bash-432f6ae62ee44e45）。
- 全量 pnpm test → 218 files passed|1 skipped（合计 219）、2025 tests passed|2 skipped（合计 2027），RC=0（32.77s；job bash-dc7ea2cf2e3b456c；no_deadlock 占位日志混入属既有非失败）。
- type-check + build:h5 → vue-tsc --noEmit -p tsconfig.json RC=0；uni build Compiler 5.24 DONE Build complete RC=0（job bash-67c88d99a7a048b8）。
- git diff --check → RC=0（仅 .gitignore LF→CRLF warning，既有非本轮改动）。
- Go 守卫 → TestNoInfraIdentifiersInRepo PASS（8.22s）；TestNoRealSecretsInRepo SKIP，逐字「读不到…laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」」，整体 ok（job bash-9d389cd129664786）。**SKIP 非 PASS，不冒充通过。**

## 6. 维度五：QA 浏览器铁证与截图
POSTFIX_REPORT.md（162 行 / 16247 B）记录三视口（390×844/768×1024/1280×800）37 组真 Chromium 走查：全部 domNoName=0、AXBare=0、hOverflow=0、docW==innerW；native input DOM 直带 aria-label，AX 树 textbox（verify 为 spinbutton）逐字带 SPEC §2 的名，空/有值两态均带名。
桥接因果链有前后对照：t110 data/ancestry-login.json 中 native outerHTML 仅 type/maxlength/step/enterkeyhint/class/autocomplete/inputmode、无 aria-label；本轮 data/pf-empty-768-login.json 中 inputs[].nativeAria 为「用户名」「口令」、domAcc.src 为 aria-label。反馈播报 6 处真机坐实 5 处：login form-hint（status/polite）、login errMsg（alert）、verify errMsg（alert）、signin redeemMsg（status/polite）、bag presetSaving（status/polite，MutationObserver 捕获瞬态 data/pf-fb-bag-mo.json）。
截图 sha256 本人亲算 4 张，与 REPORT 逐行一致且互异：pf-768-login=b147dc6223b8f83b0df1f80966061f37d5cf80cc968d5ff9dbec92b14eebeffa；pf-768-announcements=84003005ba01e3f4c50b04d893f4c99f55f75f227909b670d5a9c03f610ebc3d；pf-768-me-expanded=fc7d7041da30a99a0f1ef0a1399caac48707e8710d1d4492b1104544d7c9fc4e；pf-1280-verify=0e91c7cf51e3a348441e725da9df5a160a5c876b05bec7d4b916addc1e36e3bf。REPORT §7 记 37 张全互异（TOTAL=37 UNIQUE=37，完整表 data/pf-sha256.txt）；本人亲读 pf-768-announcements / pf-768-login / pf-768-me-expanded（前序轮次已亲读 pf-1280-verify），均非登录页真实业务页。

## 7. 维度六：诚实边界与零真值
零真值核实（本轮亲跑）：auth 四路径 docs/laiyipao/audits/round61-form-a11y/data/auth-init.js、同目录 admin-auth-init.js、laiyipao/miniapp/auth-init.js、根 auth-init.js 全部 ABSENT；git check-ignore -v 命中 .gitignore:553 docs/laiyipao/audits/round61-form-a11y/data/（RC=0）；QA 库 docker exec lyp-qa-pg psql -U postgres -d laiyipao 查 battle_records=104|477（只读未写库）；ops_grep 扫 laiyipao/miniapp/src/pages 仅 login.vue:52 :password=!showPassword 一处，属 Vue prop 绑定非真值。
诚实边界（均非阻塞）：①TestNoRealSecretsInRepo 因缺 .local.keys SKIP，按守卫原话不是通过；②me passwordMsg 真浏览器未捕获——游客态 guest_token 环境 + 默认凭据 unauthorized，依 me.vue:601-610 源码 role=status aria-live=polite 与静态契约采信；③verify 假阳性更正——analyze-postfix.js getTB() 仅匹配 role=textbox，type=number 在 AX 为 spinbutton「battle_id」，改按 spinbutton 判 PASS；④原始 RED 日志未随交付留存，依 SPEC 两轮记录与契约结构、本轮 GREEN 采信；⑤me 基础页游客态 0 input 属条件渲染范围外，以 768-me-expanded 点击设置账号密码补证 2 处；⑥QA 使用会话侧 guest_token 注入，不改 DB/服务端；⑦全仓 git diff --check 的 .gitignore LF→CRLF warning 属既有；⑧R12 自指 CTA 等本次范围外项不在本轮判定内。

## 8. 终审结论
六维全绿，无硬门禁失败，满足可发布质量。裁定 verdict=PASS（可发布）。本报告为唯一新增文件；本人未改业务源码、未做 git 写操作。