# R526 role=button 嵌套原生按钮甄别（WCAG 2.1.1 / 4.1.2 / 1.3.1）

- 真父：`bce10f59f9aa6c2a4d1494531490b90d887211d7`（台账 §517）
- 任务性质：只读甄别，不改源码、不提交源码；本轮仅新增本文档（唯一入库文件），证据与 runner 脚本入 gitignored `oa4rust/test-results/r526-*`。
- 候选：`oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:51-69` 外层 `<li role="button" tabindex="0">` 内嵌 7 个原生 `<button class="bsm" @click.stop>`。
- 反预判结论：初始预判「内层 button 的 ARIA button 语义被父 role=button 压平（4.1.2）」经 CDP 实证**被证伪**；但补证发现**另一个真实缺陷**——键盘无法激活内层按钮（2.1.1），据实判 1 可达真缺陷，不虚报也不掩盖。

## 方法与取证环境

- Playwright Chromium headless，视口 1440×900，DPR 1，light/dark 双主题（`localStorage['oa4rust_theme']` 注入）。
- 本地 Vite dev server（加载真父工作区源码），`context.route` 拦截 `/oa4rust-api`：`/api/authentication/who` 与 `/login` 返回占位 stub user；`/meeting/list/apply` 返回 3 条占位会议（id `r526-m-1..3`，楼栋名占位 `<TEST_HOST>=楼`）。所有 mock 值均占位，无真值。
- CDP：`DOM.enable` + `Accessibility.enable` + `Accessibility.getPartialAXTree`（用 `DOM.querySelector` 取 nodeId）+ `Accessibility.getFullAXTree`；键盘 `page.focus` + `keyboard.press`；按 `request` 事件记录按键期间实际发出的 API。

## 静态锚点（源码核实）

- `MeetingApp.vue:51`：`<li v-for="m in meetings" class="mc modern-card" role="button" tabindex="0" @click="viewMeeting(m)" @keydown.enter="viewMeeting(m)" @keydown.space.prevent="viewMeeting(m)">`
- `MeetingApp.vue:60-66`：7 个 `<button class="bsm" @click.stop="...">`（加入/参会人/邀请/邀请管理/改时间/标记完成/签到码）——**只阻断 click，不阻断 keydown**。
- `MeetingApp.vue:656-668` `joinMeeting()`：加入动作 = `POST /api/meeting/assemble/control/meeting/{id}/accept`。
- `MeetingApp.vue:274-303` `viewMeeting()`：详情动作 = 5 个 GET（`meeting/{id}` + `control/list` + `attachment/list` + `next/5` + `prev/5`）。
- 正面对照：`MeetingApp.vue:79` `calendar-event role="button"` 无嵌套交互；`ProcessWork.vue:40/62/84` 原生 `<li @click>` 不加 role。

## 逐项实证

### 1. 4.1.2 预判——内层按钮 AX 语义被压平：证伪（正常）

- CDP `getPartialAXTree` + 独立 `getFullAXTree` 精确枚举（双主题一致）：7 个内层按钮全部作为**独立 AX node** 存在，`role=button`、`name` 恰为七个文本、`ignored=false`、`focusable=true`、各有 `childIds`（nodeId 276-282）；外层卡片按钮 nodeId 296，`descendantButtons=[]`，即内层按钮**不是**父按钮的 AX 后代，父按钮的可访问名只是静态文本拼接。`ul role=list`（nodeId 267）存在。
- 结论：内层按钮未被压平、未被吞掉，辅助技术可独立感知其角色与名称。预判的 4.1.2「语义被压平」不成立，不计缺陷。

### 2. 2.1.1 键盘操作——内层按钮无法用键盘激活：可达真缺陷（1 项）

- 双主题 × Enter/Space 四组合，直接 `focus` 内层「加入」按钮（`BUTTON.bsm`）：
  - `innerAccept=false`（**从未**发出 `POST .../accept`）；`parentDetail=true`（每次都发出 viewMeeting 的 5 个 GET）。
  - Space 额外打开会议详情 modal（`modalCount=1`），焦点跳到关闭按钮（`.bc`）；Enter 则焦点停在「加入」但同样已发详情请求。
  - `pageErrors=[]`。
- Tab 路径复核：从卡片 Tab 一次到首按钮「加入」，再按 Enter 或 Space，结果同上（5 个详情 GET，Space 开 modal）。
- 机制：父 `<li>` 的 `@keydown.enter=viewMeeting(m)` / `@keydown.space.prevent=viewMeeting(m)` 从内层 button **冒泡**触发；内层 7 个按钮只有 `@click.stop`，没有 `@keydown.stop`，因此键盘激活被父处理器劫持去打开详情，按钮自身动作（加入/参会人/邀请/…）对键盘用户全部失效。
- 对照排除假阳性：同 stub 下**鼠标点击**同一内层按钮 → 仅 `POST .../accept`（innerAccept=true、parentDetail=false、0 pageerror），证明探针与桩有效、缺陷只在键盘路径。
- 影响：键盘/读屏用户无法通过键盘使用卡片上 7 个操作按钮中的任何一个；这 7 个功能对键盘用户实质不可达，构成 WCAG 2.1.1（Keyboard）失败，并因按钮名正确却动作错误而连带 4.1.2 层面的行为失配。判为**可达真缺陷 1 项**（同一根因，按卡片组件计 1，不按 7 个按钮重复计数）。

### 3. 1.3.1 列表语义 / 可访问名——不可达 L1（1 项观察，不计真缺陷）

- `getFullAXTree` 中 `<li role="button">` 的 `role=button` 覆盖了原生 `listitem` 语义（list 下无 listitem node，runner 以 `listitem` role 筛得 `listNode=null`）；父按钮可访问名为会议标题+状态+7 个按钮文本的冗长拼接串。
- 这是结构/命名层面的潜在改进点（属 ARIA 反模式），但辅助技术仍能通过独立 AX node 访问 7 个按钮、鼠标操作有效、列表容器 role=list 存在，不构成可证的用户阻断，判为**不可达 L1 观察 1 项**（交后续优化，不计真缺陷、不阻断）。

### 4. 正常项

- 7 个内层按钮 AX 角色/名称完整、`ignored=false`、可聚焦；鼠标点击动作正确；正面对照 `calendar-event` 与 `ProcessWork.vue` 原生 li 无回归；双主题一致；全程 0 pageerror。

## 三分判定汇总

| 类别 | 数量 | 说明 |
| --- | --- | --- |
| 可达真缺陷 | 1 | 内层 7 按钮的 Enter/Space 被父 li 的 `@keydown.enter/.space.prevent=viewMeeting` 截获，键盘无法激活按钮自身动作（2.1.1，连带 4.1.2 行为失配）；双主题、键盘+Tab 双路径、鼠标对照三重实证 |
| 不可达 L1 观察 | 1 | 外层 `li role=button` 覆盖 listitem 语义 + 父按钮可访问名冗长拼接；潜在改进，非可证阻断 |
| 正常 | — | 内层按钮 AX 语义独立暴露、鼠标动作正确、正面对照无回归、双主题一致、0 pageerror |

- 建议修复方向（不在本任务范围，仅记录）：给内层按钮补 `@keydown.enter.stop` / `@keydown.space.stop`，或将父卡片的键盘激活收敛到独立的可聚焦标题元素，避免用 `role=button` 包裹交互后代；并可重新评估 `li` 是否需要保留 listitem 语义。

## 证据清单（全部 gitignored，位于 `oa4rust/test-results/`）

- runner 脚本：
  - `r526-nested-audit.cjs` sha256 `ac3a30d612d18b2eb51b22309dedec0309d0cf8fa6f8ab2dfd2a2ef9482b92a3`
  - `r526-full-tree-check.cjs` sha256 `7531b714afd6d1a7feb39819fea419aab12ab84a8d17e4ce4f10831ea29ea51d`
  - `r526-inner-keyboard-definitive.cjs` sha256 `7c735e426aebd1c8460d8f55828bd33aab6082a4a6c98e3b65d3c5d2c35cb646`
  - `r526-inner-mouse-control.cjs` sha256 `fbb7aea58dcc1e0ca44b0749235902e93a91471da0ae200e1416e6acf2a277b2`
  - `r526-login-probe.cjs`（登录按钮定位诊断，`button.login-btn`，文本「登 录」，需先填满两输入框才 enabled）
- 结构化报告：
  - `r526/r526-report.json` sha256 `30bcb122a4648bafd7a254e89505ecf81006704900da919ac2fcae67fa3d5cc5`
  - `r526/r526-full-tree-check.json` sha256 `1319c829af6186432ed21ec6b67303a72b31809d6b3fd1efba98ad972474c549`
  - `r526/r526-inner-keyboard-definitive.json` sha256 `aac00ad200687c75ec365d9478d6a6efa10c9d060fc750914bd738d0037d7527`
- 截图（均 1440×900）：
  - `r526/r526-meeting-light-list.png` sha256 `57e85a995a27243e118b2f831293e4560bd70467831a696e0c4962472598829c`
  - `r526/r526-meeting-light-keyboard.png` sha256 `946f155665acb62d9f7775634fef15511699d02a6017a9109f97a0f2df7efa69`
  - `r526/r526-meeting-light-inner-focus.png` sha256 `a2e4a7585b414a27f48773454338b4013dc3d36f836ce82ddfce81a43f026461`
  - `r526/r526-meeting-dark-list.png` sha256 `0f5ae247dd27fb68ebd95af1608db7100c9b2652309d80d916c470f141b0ea04`
  - `r526/r526-meeting-dark-keyboard.png` sha256 `7c6e0c5b099af897f564beec283e49b3575b12d2116bb6b40baf3e49283d52a4`
  - `r526/r526-meeting-dark-inner-focus.png` sha256 `1050c68521613e5dc6ba84663613588373391ae3356b51a56bbff299a36111f0`
- `git check-ignore` 确认上述 `r526-*` 全部被 `oa4rust/test-results/` 忽略，未入库。

## 脱敏声明

- 全文 0 真值：无密钥/口令/个人邮箱/手机号/真实域名或 IP；stub user 与会议 id、楼栋名 `<TEST_HOST>=楼` 均为占位符，本地回环地址仅用于可复现验证，不含任何个人信息或环境真值。
