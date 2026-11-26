# R60 独立终审：role=button 键盘可达性（WCAG 2.1.1 / 2.4.7）

- 终审任务：t104（attempt_id `25f8fac9-40ed-4ce7-b1c6-5c9527104fa4`），被审任务 t102（实施）/ t103（QA）。
- 独立链：t100 侦察 → t101 规范（SPEC + test-first 契约）→ t102 实施 → t103 真浏览器 QA → 本轮终审。
- 终审角色：same-route-reviewer；只读审阅 + 独立复跑 + 唯一落盘本报告；不改业务源码、不 git 写。

## 0. 总判定与独立性声明

**判定：PASS，可发布。** R60 修复把全站 48 处 role=button 的 text/view 从「键盘完全不可达」补齐为 tabindex=0 + @keydown.enter + cursor:pointer，真浏览器 6 页 × 3 视口共 18 组、每组 25 次真实 Tab 全部落到 role=button，Enter 激活与原鼠标 @click 行为一致，focus-visible 焦点环可见；六维门禁全部独立复跑通过，保护面零改，零真值入库。
独立性：本轮所有结论均由本人亲自读文档、读 diff、跑门禁、算 sha、亲读截图得出，不直接采信 QA/实施的自述；凡原始日志未留存者（RED 首轮）如实标注采信来源。

## 1. 维度一：test-first 真实性

- 规范 `docs/laiyipao/specs/round60-a11y-keyboard/SPEC.md`（54 行/8 节）：§0 依据 ROUND60_A11Y_RECON.md（t100 真 Chromium + axe-core），权威 WCAG 2.1.1 Keyboard + 2.4.7 Focus Visible；§1 缺口列 48 处与真浏览器 12 次 Tab 只在 BODY/INPUT 循环的铁证；§2 逐元素修复口径；§3 保护面；§4 诚实边界；§5 RED 门禁。
- 契约 `laiyipao/miniapp/src/styles/r60A11yKeyboard.contract.test.ts`（90 行/6 it）：真实 `node:fs`/`node:url`/`node:path` 递归读 pages 下全部 .vue 与 common.css/tokens.css，无内嵌实现；roleButtonLines 筛含 role=button 且 text/view 的行。
- RED 曾真：SPEC §5 记录终轮 exit=1 / 1 failed file / 3 failed | 3 passed / 263ms，三条失败均为真实缺口断言（48 处缺 tabindex、48 处缺 @keydown、common.css 缺 [role=button] 全局规则），三条通过为不变量；首轮 esbuild `Unexpected "*"` 解析错误（注释里 pages/**/*.vue 提前闭合块注释）属非正当 RED 已修正后重跑。**原始 RED 日志未留存，本项依 SPEC 记录 + 契约结构 + 本轮 GREEN 对照采信。**
- 本轮独立复跑契约 GREEN：6/6 passed（job `bash-3b5fce09b2a34f98`，RC=0，824ms）。

## 2. 维度二：契约与实现对齐

- 实施 diff（`git diff --unified=0 -- laiyipao/miniapp/src` 亲读）：11 个页面 vue + common.css 共 12 业务文件；每处 role=button 的 text/view 在保留原 @click 的前提下，同期追加 `tabindex="0"` 与 `@keydown.enter`，逐处核对 @keydown.enter 的处理器与原 @click 逐字等价（如 r5GoBattles()、r6Open = !r6Open、r14ConfirmApply()、r17GoAuthority 等），无语义改写。
- 精确计数（`grep` 全站 pages/*.vue）：role=button=48、tabindex=0=48、@keydown=48、@keydown.enter=48，四处一一对应，无遗漏、无多余。
- common.css 仅在 :907 新增一处 `[role="button"] { cursor: pointer; }` 全局规则；focus-visible 复用既有 :908 `:where(...,[tabindex]):focus-visible`（outline: 3rpx solid var(--accent)），契约 it4 断言该声明次数 ===1，本轮未重复造规则（:912 裸 :focus-visible 为既有通用规则，非 [tabindex] 变体，不计入）。
- 契约 6 it 覆盖：逐元素 tabindex / 逐元素 @keydown / 全局 cursor:pointer / focus-visible 不重复 / 已收口 CTA 锚点不变量 / 令牌三锚点不变量，与 SPEC §2 修复口径一致。

## 3. 维度三：保护面与红线零改

- tracked 改动仅 13 文件：`.gitignore`（审计 ignore 规则）+ 11 页面 vue + common.css，全部为 a11y 展示层；engine / replay / hash / score / settle / loot / tokens / 后端结算零改（相关路径无任何 diff）。
- 令牌三锚点 `tokens.css:14-16` 逐字原样（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）；全 src 扫 `--r60-` 仅契约 2 处否定断言（:83 / :88），业务源码 0 命中，未新增令牌。
- 已收口 CTA 锚点保持：契约 it5（本轮 GREEN）断言 battles r20 CTA 带 @click=goStage、battle r29 带 @click.stop=openReactionTimeline、index ai-cta 带 @click=go(aiInsight.hint) 与 @click=goStage，R43/R44/R46/R47/R48 言行一致性无回退。
- 未改任何原 @click、未改 focus-visible 既有规则内容、未改 CTA 文案与路由行为。

## 4. 维度四：门禁独立复跑

| 门禁 | 本轮独立复跑结果 | RC |
| --- | --- | --- |
| 契约 r60A11yKeyboard | 6/6 passed（job bash-3b5fce09b2a34f98，824ms） | 0 |
| miniapp 全量 pnpm test | 217 files passed / 1 skipped（218）；2020 tests passed / 2 skipped（2022），前台重跑 24.50s | 0 |
| type-check（vue-tsc） | passed（job bash-7a603362113149da） | 0 |
| build:h5 | DONE Build complete（job bash-f66a315aea0c4eb9） | 0 |
| git diff --check（全仓） | 干净（仅 .gitignore LF→CRLF 换行 warning，非 diff 错误） | 0 |
| Go TestNoInfraIdentifiersInRepo | PASS（8.22s，job bash-7606afa346ae4055） | 0 |
| Go TestNoRealSecretsInRepo | **SKIP 非 PASS**（读不到 .local.keys，真值扫描未执行） | 0（进程） |

说明：QA 报告（t103）阶段全仓 git diff --check 曾 RC=2（.gitignore:553 末尾空行，QA 无权改队长资产）；队长已删除该空行，本轮独立复跑全仓 RC=0，硬门禁已全绿。TestNoRealSecrets 逐字原文「真值扫描这条守卫没有执行，不是「通过了」」，如实标注为 SKIP，不计 PASS。全量测试 stderr 中 build.attacker 占位告警为既有测试桩提示，非失败。

## 5. 维度五：QA 浏览器证据复核

- Tab 聚焦（job bash-b250087946b94ab1，RC=0）：6 代表页 × 3 视口（390×844 / 768×1024 / 1280×800）共 18 组、每组 25 次真实 Tab，activeElement 全部落到 role=button（rbHits>0、firstHit 靠前）；全部 innerW==docW、hOverflow=0、hrefOk、visAllTabindex、visAllCursorPointer；焦点 outline 390/1280 为 solid 1px、768 为 solid 3px（=3rpx accent），全非 none。对照改前 12/12 次 Tab 只在 BODY/INPUT 循环、0 落 CTA。
- Enter 激活（job bash-fe9ccf25fc034df0，RC=0，768 六页）：announcements hash announcements→battles（bodyLen 692→1125）真导航；reactions/equipment/skills 展开可见副作用（bodyLen +158/+120/+113，scrollY 下移）；shop/mastery pageScrollTo 真滚动（scrollY 29→3803 / 0→1355）；均与原 @click 同效。鼠标正对照 click .r3-ai-cta 与 Enter 同样抵达战报页。
- 回归（job bash-20e6309f32d84536 / bash-b0f24be509d24142，RC=0）：R51 me 雷达 viewBox 0 0 200 180、R53 首页结论、R55 reactions CTA nowrap 均保持；battles r20 / index / equipment·bag·gems·skills r12-priority-cta 的 @click 言行完好。
- 截图 sha 亲算：25 张逐一重算与 QA REPORT §8 逐行一致；24 唯一，唯一重复 r60-enter-announcements.png = r60-mouse-announcements.png（sha d5482858…，同为 Enter/鼠标抵达的战报页，预期合理）。
- 亲读 4 张（r60-768-announcements / r60-768-reactions / r60-768-skills / r60-enter-announcements）均 768×1024、非登录页，页面内容与 QA 描述相符（公告摘要 CTA、反应图鉴对照表、技能构筑诊断、战报页 Enter 后落地）。

## 6. 维度六：诚实边界与零真值

- axe-core 5 项结构性 violation（label critical、landmark-one-main、meta-viewport、page-has-heading-one、region 9 节点）属 H5 框架层/全站结构，SPEC §4 明确不在本轮范围，仅披露；color-contrast 55 节点为 axe incomplete（伪元素背景无法确定），非确定 violation，不据此判定。
- CTA 键盘覆盖残留（范围外，非回退）：R12 自指 CTA「在本页强化装备契合 →」与 battles「看战报详情 →」仍为纯 view/text、无 role/tabindex/cursor:pointer；其 @click 鼠标言行完好，不属 R60 的 48 个 role=button 契约范围，不构成 R43/R44/R46/R47/R48 回退，整体 a11y 残留须后续评估。
- 零真值：审计/会话侧 auth-init.js、login.json 共五路径全部 ABSENT；git check-ignore 命中 .gitignore:549（round60-a11y-keyboard/data/）；QA 库 Docker lyp-qa-pg 查 battle_records=104|477，本轮只读未写库。guest_token 经会话侧注入、不改 DB/服务端。
- QA 自述四项（曾 RC=2、24/25 唯一、SKIP 非 PASS、CTA 残留）均如实披露，其中 git diff --check 已由队长修复并经本轮复跑 RC=0。

## 7. 结论与发布建议

R60 六维门禁全绿、修复真实有效、保护面零改、零真值入库、诚实边界充分披露，**判定 PASS，建议发布**。非阻塞保留项：axe 5 项框架层 violation 与 color-contrast incomplete（范围外）、CTA 键盘覆盖残留（范围外，后续轮次评估）、TestNoRealSecrets 因缺 .local.keys SKIP（非本轮可控，不得计为通过）。