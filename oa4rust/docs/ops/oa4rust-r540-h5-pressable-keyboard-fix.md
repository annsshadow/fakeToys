# R540 H5 工作台伪按钮键盘可达修复（WCAG 2.1 SC 2.1.1 / 4.1.2）

- 轮次：R540（repair，t45），真父 `b8457edeef5c522ffb1840b15854137dd4609d72`（台账 §530，已 push），契约 RED 基准 `a018481e0`。
- 缺陷：R539 判 1 项 P2——`index.vue` 2 统计卡 + 1 公告项 + 12 宫格 + 刷新/重试 + 公告详情关闭共 17 个 `<view @tap>` 伪按钮，AX `role=generic name=null`、无 tabindex，Tab30 次无一命中；键盘用户无法激活、读屏无法识别为按钮。
- 去伪：t43/t44 双重坐实——运行时注入 `role=button + tabindex=0 + aria-label` 后同节点 AX=button/focusable=true/name=注入值，稳定进 Tab 序列 15 次；pages.json tabBar 仅 5 项且 CDP 实测键盘层 `roleTabindexNodes=[]`、Tab18 次全 BODY，消息/审批/考勤鼠标可达但键盘无替代，不构成豁免。

## 一、方案取舍（Fail Loud）

队长要求方案A全局增强器或方案B逐页显式补，据实评估择优：
- **方案B（逐页显式补 role/tabindex/aria-label/@keydown）已采用**：17 处仅追加 a11y 属性，业务 handler（goTab/goPage/loadNotices/viewNotice）复用原函数、零改函数体；精确控制预期文案（待办审批/今日出勤/12 宫格名/公告动态标题/重试/刷新公告/关闭公告），只影响工作台页。
- **方案A（main.ts 全局增强器）未采用**：实证选择器 `.ai-pressable/.retry-btn/.o2-list-item` 被 browse/collect/doc/meeting/recycle/message/process/attendance/bbs/chat/contacts/search/start/topic/calendar 等十余个他页广泛使用，全局注入会越界修改本轮范围外页面（违反写域与业务零改）；且统计卡可见文本含动态数字（`{{ pendingBadgeText }}`），从文本派生 aria-label 不准确。方案B 与 t43/t44 注入对照口径一致。
- **@tap→click 链路已实证**：preflight 探针对 `.cell` 派发合成 `MouseEvent('click')`，`document.querySelectorAll('.cell')[2]` 点击后 hash 从 `#/pages/index/index` 变为 `#/pages/doc/doc`，证明 uni-app H5 把 `@tap` 编译为 DOM click，键盘 keydown→派发 click→触发 `@tap` 可行。E2E 后续 Enter/Space 实测导航成功进一步坐实。
- `main.ts` 本轮零改，R537 表单名增强器与 `calendar.vue:295` 正面对照完整保留。

## 二、写域（实际改动 3 文件）

契约 inScope 列 4 候选（main.ts + index.vue + 契约 + 文档），main.ts 本轮零改，实际改动 3 文件：

| 文件 | 变更 |
|---|---|
| `oa4rust/mobile/src/pages/index/index.vue` | 17 处 `<view>`/`<button>` 补 `role="button" tabindex="0" aria-label`，`@keydown.enter`/ `@keydown.space.prevent` 复用原 handler；公告项用 `:aria-label="n.title || '无标题'"` 动态名 |
| `oa4rust/mobile/src/pages/R540.h5PressableKeyboard.contract.test.ts` | 新增，10 用例源码级契约 |
| `oa4rust/docs/ops/oa4rust-r540-h5-pressable-keyboard-fix.md` | 本文档 |

业务零改：`goTab(uni.switchTab)` / `goPage(uni.navigateTo)` / `loadNotices` / `viewNotice` 函数体、`v-if/v-for`(含 `:key`)、原 `@tap` 鼠标行为、`v-model`、模板结构、样式段、他页、`calendar.vue:295` 正面对照逐字未改。git diff 实证 17 行仅追加 a11y 属性。

**公告详情关闭据实纳入**：R539 把 `nd-close`（原生 `<button>`）列为观察项，本轮据实判断——uni-app 编译后包裹层 AX role=generic 且 Tab 不达，补 `tabindex="0" aria-label="关闭公告"` 纳入修复（原文案「关闭」保留，handler 零改）。

## 三、契约 RED → GREEN

契约 10 用例：5 核心（2 统计卡 / 12 宫格 / 公告项 / 重试×2+刷新 / nd-close 的 role+tabindex+aria-label+keydown）+ 5 守卫（业务函数体 / loadNotices 逻辑 / 原 @tap 保留 / R537 增强器零改无伪按钮注入 / calendar:295 与样式段零改）。

- 真父 `a018481e0` 覆盖 index.vue → `pnpm test` 定点 exit=1，5 failed（核心全 fail：无 a11y 属性）| 5 passed（守卫全过）。
- 恢复修复态 → exit=0，10 passed。日志 `r540-red.log` / `r540-green.log`。

## 四、三 verify（全 exit 0，workdir=oa4rust/mobile）

| # | 命令 | 结果 | 日志 |
|---|---|---|---|
| V1 | `pnpm test src/pages/R540.h5PressableKeyboard.contract.test.ts` | exit 0，10 passed | r540-green.log |
| V2 | `pnpm test` | exit 0，81 files / 542 tests passed | r540-verify2.log |
| V3 | `pnpm typecheck` | exit 0，tsc --noEmit | r540-verify3.log |

## 五、决定性 CDP 取证（focused=true 非 root 节点 AX role=button / focusable=true / name 非空）

- runner `oa4rust/test-results/r540-e2e.cjs`，Playwright chromium headless，390×844 DPR2 isMobile/hasTouch，自起 uni H5（<TEST_HOST>），stub 认证/公告/待办/出勤（0 真值）。
- 判据口径：`getFullAXTree` 中 `prop(n,'focused')===true` 且非 `RootWebArea` 的节点，AX role=button / focusable=true / name === 预期；`document.activeElement` 为 `UNI-VIEW[role=button]`。**未探宿主 wrapper 假阳性**（R535 教训）；CDP focused 在 `n.properties[]` 非顶层字段。
- 结果 `R540_E2E_DONE`：`namesBad=0 fatals=0 pageErrors=0`。每组 15 项 probe（2 统计卡 + 1 公告项 + 12 宫格）全部 pass，AX role=button/focusable=true/name=预期（待办审批/今日出勤/R540-gonggao-title/消息/审批/文档/通讯录/考勤/发起流程/会议/日程/论坛/浏览/搜索/回收站）。
- Tab 可达：Tab 序列 40 次命中 37 个 `role=button[aria-label]` 节点。
- 注入对照：移除 `.cell[9]` 的 aria-label → focused name 失配（nameBad=true）；注入 `R540-injected-control` → focused AX role=button/focusable=true/name=R540-injected-control（nameBad=false），证明 AX 通道与采集方法有效。
- Enter/Space 等价导航：聚焦文档宫格按 Enter → hash=`#/pages/doc/doc`；聚焦通讯录宫格按 Space → hash=`#/pages/contacts/contacts`（preflight 已证 `@tap`→DOM click，E2E 坐实导航）。
- 正面对照：`calendar.vue:295` `[role=button][aria-label]` → role=button / aria-label=新建日历，零回归。

### Fail Loud：H5 无主题切换

uni-app H5 App.vue 无 `oa4rust_theme` / `data-theme`，light/dark 两组截图 sha 相同（`83d4954c…`），据实记录、不伪造暗色差异。

## 六、证据 sha256

| 产物 | sha256 |
|---|---|
| index.vue（修复后） | 892d75341e36d115bf61b21b53aaf62d5e53b7f94c06b57b382955cd8408cd40 |
| main.ts（本轮零改） | b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b |
| R540 契约 | 5c9942b0ad6654063275d9c4dee9d9b1e79d8f6dbaec1ce76a3f124ed763a401 |
| r540-e2e.cjs | d82b0635c6e2ee905e2c9bc88d7f78b67c07f7939f0b87693ca0ecd003bf87e8 |
| r540-ax-report.json | ae4988df856b783dae3bbe76f909111e1c33e7864f1be9c29cdba47030036388 |
| r540-red.log | 80c053e9de85b10b2ce12d1ebaabe7f03170a1ec9024a8d89b6d47605fffda28 |
| r540-green.log | 09fec55e5ef51dc8267dbe7118fbc5ba3de32cf381fe8b19c352328de65e8ac6 |
| r540-verify2.log | 3adee1be22ea4111e934546d4077a05d0bfccb95dfa8a25ef4ff282fe8da6b9d |
| r540-verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| r540-index-light.png = r540-index-dark.png | 83d4954c3151774a08376eb9a4d2ca41ca12e2b5b04a0d12c7d6d982e5e4682c |

截图 780×1687（390×844 @DPR2），2 图目视正常：工作台 2 统计卡（待办审批/今日出勤）、公告列表、12 宫格均完整渲染，a11y 属性不影响视觉布局。runner/日志/报告/截图位于 `.gitignore` 排除的 `oa4rust/test-results/`，不入库。

## 七、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 `<TEST_HOST>` 占位，认证与列表数据为 stub 占位，0 真值入库。
