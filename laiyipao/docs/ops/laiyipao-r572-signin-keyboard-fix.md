# R572 laiyipao/miniapp 签到页 signin.vue 5类裸@click伪按钮键盘可达修复

## 1. 背景与缺陷
- 缺陷：laiyipao/miniapp 签到页 signin.vue 有 5 类裸 @click 文本伪按钮无 role/tabindex/aria-label/@keydown，键盘用户无法 Tab 到达，AT 读出 generic/name=null，违反 WCAG 2.1 SC 2.1.1 A（关联 4.1.2 A）。
- 分类：每日签到按钮、离线领取按钮、周期切换 tab（每日/每周/成就）、任务领取按钮（v-for tasks）、兑换按钮。
- 来源：R571 甄别文档 laiyipao/docs/ops/laiyipao-r571-signin-keyboard-audit.md 判 P2 真 bug。
- 修复纪律：按真 bug 的 RED→GREEN 优先，先写契约在真父跑 RED，再修复跑 GREEN。

## 2. 方案取舍（Fail Loud）
- 方案A（全局增强器）：排除。理由：全局注入越界他页，且动态 aria-label 无法从 DOM 可靠派生易误伤；R537 增强器仅覆盖宿主表单控件，不覆盖小程序自定义元素。
- 方案B（逐页显式补）：采用。5 处开标签就地补 role=button + tabindex=0 + aria-label + @keydown.enter/.space.prevent 复用原 handler，@click 全保留，业务零改。先例 R561/R562/R564/R566/R568/R570 全绿。
- 周期 tab 形态：选 role=button + aria-pressed + tabindex=0 + aria-label=s.label，拒绝 role=tab + tablist。理由：验收要求 3 枚 tab 均 Tab 可达，WAI-ARIA Tabs 的 roving tabindex 只让选中项入 Tab 序列、其余需方向键导航，而 switchScope 无方向键逻辑，半吊子 tablist 反伤 AT；与 R564 rank.vue 先例一致。契约守卫钉死无 tablist/tab/aria-selected。

## 3. 写域与零改
- 写域精确 3 文件：
  - M laiyipao/miniapp/src/pages/signin/signin.vue（5 处开标签纯 a11y 属性追加）
  - A laiyipao/miniapp/src/pages/R572.signinKeyboardA11y.contract.test.ts
  - A laiyipao/docs/ops/laiyipao-r572-signin-keyboard-fix.md（本文件）
- 零改：doSignIn/claimOffline/switchScope/claim/doRedeem 函数体、api、样式、他页、main.ts、既有 a11y 契约、battles 正面对照、@click 全保留。

## 4. 源码修复表（5 处，git diff 22 insertions/1 deletion）
| # | 位置 | 元素 | 追加属性 | 复用 handler |
|---|---|---|---|---|
| 1 | :38-45 | 每日签到按钮 | role=button tabindex=0 aria-label=领取今日奖励 | doSignIn |
| 2 | :85-91 | 离线领取按钮 | role=button tabindex=0 aria-label=领取离线收益 | claimOffline |
| 3 | :115-125 | 周期 tab（3 枚） | role=button tabindex=0 aria-label=s.label aria-pressed=scope===s.key | switchScope(s.key) |
| 4 | :151-158 | 任务领取按钮（v-for tasks） | role=button tabindex=0 aria-label=领取任务+t.title | claim(t.id) |
| 5 | :173 | 兑换按钮（单行整行替换） | role=button tabindex=0 aria-label=兑换 | doRedeem |
- 踩坑：首轮 ops_edit_file 原子 5 处 EDIT_CONFLICT，因离线领取按钮缩进按 8 空格写、实际 10 空格；dryRun 逐条定位后修正。
- 编码：源文件无 BOM（hex 首字节 3c），未引入 BOM 变更。
- 修复版 sha=bbb48c8b2dca50ec5cc189e46b99fcad60407632c80aa2b38b76821162c6afdf，真父 sha=ae447d22dc6e7aa8c1a13638ffee23ac79b635753df2317ba5c83c91bbe5b794（见 §9）。

## 5. 契约 RED→GREEN
- 契约 9 用例 = 5 核心（blockFrom 起始锚点向下扫到 trimEnd().endsWith(>)，断言 role/tabindex/aria-label/@keydown 复用原 handler + @click 总数 5）+ 4 守卫（业务 handler + loadSeq 竞态、数据源/样式/main.ts、battles-rank-stage 正面对照 + @click 保留、aria-pressed 唯一）。
- 踩坑：用例 4 blockFrom(v-for=t in tasks) 定位到父 task 块非内层领取 view → 改 blockFrom(style=width: 140rpx; height: 64rpx; font-size: 24rpx)；用例 7 断言 .offline-claim{ 与敌人雷达 svg 不存在 → 改 .offline-msg{/.task-claimable{/<style scoped>。
- RED（真父 git show 覆盖）：EXIT=1，6 failed | 3 passed，HEAD 未变 9d239db6f。
- GREEN（修复版）：EXIT=0，9 passed。
- 恢复：cp 修复版备份回工作树，sha=bbb48c8b2dca50ec5cc189e46b99fcad60407632c80aa2b38b76821162c6afdf 坐实。

## 6. 三 verify（workdir laiyipao/miniapp，字面命令全 exit0）
- V1：pnpm vitest run --config vitest.config.ts src/pages/R572.signinKeyboardA11y.contract.test.ts → 9 passed，EXIT=0
- V2：pnpm test → 163 files passed | 1 skipped（164 files），1581 tests passed | 2 skipped（1583），EXIT=0
- V3：pnpm run type-check → vue-tsc 无错，EXIT=0

## 7. 真浏览器 CDP 决定性判据
- 环境：Playwright 390x844 DPR2 isMobile/hasTouch，自起 laiyipao H5，CDP getFullAXTree prop(n,focused)===true 非 RootWebArea。
- 判据口径：document.activeElement + getFullAXTree focused 非 root 节点的 AX role/name；严禁宿主 wrapper 假阳性（R535 教训）；UNI-VIEW/UNI-TEXT 身份独立确认。
- 8 组 = light/dark × withData/empty/noReward + func + mouse 探针。
- 决定性判据全绿：
  - withData/noReward 各 7 类 focused 节点 tag=UNI-VIEW/UNI-TEXT，role=button，name=领取今日奖励/领取离线收益/每日/每周/成就/领取任务R572日任务/兑换，isTarget=true isBody=false，axFocused=[button]。
  - 周期 tab aria-pressed 仅 scope0=true，其余 false（选中态程序化可读）。
  - DOM.getAttributes role=button/tabindex=0/aria-label 达标（empty 态 claim0=0 因无任务，据实披露）。
  - Tab 命中：withData/noReward 各 3 次；empty 态 claim0=0 据实披露。
  - 注入对照：stripA11y 后 7 类 attrs role=null；restore 后全 button；inject 给 7 类重设 aria=R572-injected-* 均生效。
  - 正面对照：注入原生 button aria-label=R572正面对照控件，AX role=button/name 达标（自包含验 AX 通路，规避跨页假阴性）。
  - 功能真命中：scopeEnter 色标 0→1 且 scopeCalls=[daily,weekly]；scopeSpace 色标→2 且 scopeCalls2 含 achievement；signEnter 文案 ✓今日已签到 + writeCalls=[signin]；offlineEnter 离线Msg=领取成功：500 金币 + writeCalls2 含 offlineClaim；claimEnter 经 reload 补造数后 writeCalls3 含 claim:21；redeemEnter=兑换成功：钻石 10 + writeCalls4 含 redeem。
  - 鼠标零回归：click scope1 色标转移 + click sign 已签到。
  - 0 pageerror / 0 fatal。

## 8. 造数与据实披露
- stub 端点：/signin/status、/me/offline-reward、/tasks、/signin/calendar、/redeem 等（同 R571）。
- 造数分层：周期 tab 常驻（纯前端状态切换 scope===s.key 改 color）；签到/离线/兑换需 status；任务领取需 /tasks；无结果态需对应 stub。
- runner 踩坑 3 处：① /tasks? 路径匹配漏 GET /tasks → 改 p.endsWith('/tasks') && m==='GET'；② func 探针 claim0 在切周期后不存在致 null focus fatal → 加 hasClaim 判断 + reload 重进造数；③ page.fill('.code-input') 对 uni-input 无效 → 改 locator('.code-input input, uni-input-input').first().fill。
- 端口披露：默认开发端口首启 EACCES（远端 IP TIME_WAIT 占用），uni-app 硬编码该端口忽略 --port 传参，最终该端口 HTTP=200 正常取证；真地址以 <TEST_HOST>:<PORT> 占位，不入库。
- 截图 6 张 780x1776 目视名实相符：withData=领取今日奖励+离线500+任务 R572日任务+兑换；empty=暂无任务（签到/离线/兑换仍在）；noReward=✓今日已签到+已领完 0 金币。H5 无主题机制，light/dark 视觉一致，sha 差异来自窗口状态，据实披露。
- 决定性判据均在截图/跳转前采集，不依赖截图。

## 9. sha256 证据表（逐项核对）
分两组：A 组为 runner 紧凑 JSON 汇总值，B 组为文件 sha256sum 实测值。

A. runner 汇总/汇总日志
| 项 | sha256 |
|---|---|
| R572_DONE 汇总 | 16cc664060e12aab448cbe6f9ea245b47617f1eb3ddab85904a6aee8abf52c51 |
| E2E 日志 | 745fab460bab1432ce3776af4210eb89ba9736d7bb5a60e48e58c8b626566427 |

B. 文件 sha256sum（逐一实测）
| 文件 | sha256 |
|---|---|
| signin.vue（修复版） | bbb48c8b2dca50ec5cc189e46b99fcad60407632c80aa2b38b76821162c6afdf |
| R572.signinKeyboardA11y.contract.test.ts | dcdca723b4f69cac89483666a49a0154c020cee7b364941a267dde338dda7579 |
| r572-e2e.cjs | a15149a41f954b9f68490ffecda069a5c3ef88a75ebc368601ea20f3f5ebac45 |
| r572-report.json | e43a190e6389e538aa4147bdb0abdf670d874f5055c7dea17f54a5a4d39f9102 |
| /tmp/r572-red.log | cdc5e9d262af52d0eec29ffc3572d0323a65199470828314c6db5df2d8e3272e |
| /tmp/r572-green.log | c9883e1ca3a0072c5d259c0051b0f5902694255244e4ac157e7b41cf644a66d4 |
| /tmp/r572-v1.log | 0dd3a0f3f5b7838633491ccedf86f077dd0a97dce37b36bf1e9e067bacc887b6 |
| /tmp/r572-v2.log | 89cab6a8f2e59c3c92283ee68bbd43ec5d4f2ce73e394216febd1f11c12df141 |
| /tmp/r572-v3.log | 37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f |
| r572-summary.cjs | 5619ba3949e7e5393862a5b94d00fb91124a935fdb23110220f8bcc5b146dde8 |
| r572-withData-light.png | 75616bc4da1596e8b6816f9a2728dc1ea30d7629d256b0780f0725fa84f31982 |
| r572-withData-dark.png | caa7fb885dfab0f40dfc0ab2362ccb9b78d9d2db29cc3fc8ae8c7cb017f15735 |
| r572-empty-light.png | c4593ab728a3309a0cd72142834744672d0586ff48435a50e3ca8e03ee24b5f9 |
| r572-empty-dark.png | 73038f8ad3262240601b59eb10467b901ed62839d913bc98f9eafa4887a1801f |
| r572-noReward-light.png | 66d0c5a09d65cdb9a5cf3f0d5af46670ca53b2651bdefb28b6496a3d3f6f010b |
| r572-noReward-dark.png | be7838be9b912e78851e9131442b470620e875de94c30220771d8352460a6aac |

真父 signin.vue sha=ae447d22dc6e7aa8c1a13638ffee23ac79b635753df2317ba5c83c91bbe5b794，HEAD 全程未变 9d239db6f41b7ced465669e77ad902d7da7f0946。

## 10. 脱敏检查
- LEAKSCAN：仅 <TEST_HOST>:<PORT> 占位，0 真值（无 IP/域名/token/口令/个人路径）。
- 证据文件全在 gitignored oa4rust/test-results/r572/ 与 /tmp，未 track。
