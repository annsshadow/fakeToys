# R502 WCAG 3.3.4 Error Prevention 错误预防甄别（只读）

> 本轮为只读甄别，不改任何产品代码；真父基线 `9011d336e15c7d7816c8dcc5f5bd9528383d8230`（台账 §497）。
> 报告来源：Playwright Chromium headless（1440×900 DPR1；移动 H5 视图 430×932 DPR1），**非物理真机**。

## 1. 判据与反预判纪律

- WCAG 3.3.4 Error Prevention：对用户造成**不可逆/破坏性**后果的操作，须提供可逆、校验或确认机制。
- 三分判定：可达真缺陷（破坏性操作被真实触发且无二次确认/撤销，须实证）/不可达 L1（业务守卫或数据不存在使路径不可达）/正常规范预期（已有确认或等价防错）。
- 铁律：区分**业务守卫**（如道具不足、选中为空、数据不存在）与**确认机制**（Modal.confirm / uni.showModal / ElMessageBox.confirm / 撤销）；业务守卫挡住时不可据源码臆判缺陷；必须确认破坏性状态真实变化。
- 截图为 headless 真浏览器截图，SHA-256 与证据文件一致；域名与口令真值一律不入仓，只在 runner 运行时读取。

## 2. 静态候选与排除

| 编号 | 位置 | 操作 | 静态信号 | 工程内对照 |
|---|---|---|---|---|
| A1 | laiyipao/miniapp/src/pages/mastery/mastery.vue:24-36 `:28` 守卫，`:327-352` resetMastery | 重铸专精（清空全部已点亮节点，道具 mastery_reset 消耗） | 单击 `@click=resetMastery`，仅 `resetting` 守卫 + `showToast`，文案自述「点错方向后唯一的改向途径」 | 同仓 me.vue:1342-1373 删除防线有 `uni.showModal` |
| A2 | augmentor/web/src/pages/Augmentation.tsx:64-73 doDeleteCheckpoint；`:298-315` Tag closable onClose | 删除增强断点（从头重跑） | 单击直接 `deleteCheckpoint` + `message.success` 无 `Modal.confirm` | 同仓 DataManagement.tsx:281-295 删除数据有 `Modal.confirm` |
| A3 | laiyipao/miniapp/src/pages/bag/bag.vue:411-415 / `:1235-1247` doDeletePreset | 删除配装预设 | 单击 `@click=doDeletePreset`，仅 `presetWriteBusy` 守卫 + `showToast` | 同仓其他删除有 showModal |
| 排除 | me.vue:1342-1373（删除防线）；battle.vue:869；shop.vue:371；admin ContentView:449-452 | 各类破坏性操作 | 均有 ElMessageBox.confirm / uni.showModal / 服务端口令守卫 | — |

## 3. 真浏览器实测配置

- 主站点 A2：正式站 `/augmentor/`（React + Ant Design）。通过测试服 API 造数：`POST /api/data/upload` 上传最小 JSON 数组后 `POST /api/augment/start` 生成断点 task_id；正式站 `GET /augmentor-api/api/augment/checkpoints` 确认可见。浏览器带运行时 API key（localStorage `augmentor.api_key`）进入数据增强页。
- 移动 H5 A1/A3：正式站 `/laiyipao-h5/`（uni-app H5），viewport 430×932 DPR1；使用运行时注入的共享测试账号经真实 UI 登录，进入 `/#/pages/mastery/mastery` 与 `/#/pages/bag/bag`。凭据仅经 SSH runner 从进程环境读取，不回显、不入库。
- 每个候选点击前后多时点采样 DOM、MutationObserver 捕获新增 modal/toast/dialog 节点、CDP `Accessibility.getPartialAXTree` 检查 dialog AX 语义；截图在点击前后落盘。
- 采样频率 A2：120/400/1000/1800ms；A1/A3：120/400/1000ms。

## 4. 逐候选实测结果（programmatic）

### A1 重铸专精（laiyipao H5，light/dark 双主题）

- 登录成功（URL `/#/`）；进入专精页后 `.node.on` 已点亮节点数 3，重铸链接文本「重铸」且 `link-off=true`。服务端只读 wallet 显示 `tokens.mastery_reset=0`（重铸道具为 0）。
- 点击 `.reset-row .link`（force 点击到）后三个时点：`.node.on` 始终 3、无 modal、无 toast、无 native dialog、MutationObserver 记录 0、CDP dialog ax 节点 0、页面错误 0。
- 判定：**不可达 L1**。业务守卫 `resetTokens<=0` 使破坏性动作未真实发生，两主题一致；据实不得据源码无 showModal 判缺陷。

### A2 删除增强断点（augmentor，PC 1440×900）

- 点击前：断点列表 tag 1 个（`augment_792b1b36477a`），dialog 0，Mutation 0。
- 点击 `.ant-tag-close-icon`（force）后 120ms 起：tag 数变 0；四个时点 dialog / popconfirm / native dialog 全部 0；MutationObserver 捕获 2 条「断点已删除」成功消息节点；CDP dialog ax 节点 0；页面错误 0；测试服 `GET /api/augment/checkpoints` 变为 `{"checkpoints":[]}`。
- 截图目视：点击前显示断点 tag；点击后仅顶部绿色 toast「断点已删除」，断点列表变「暂无断点」，无确认弹窗。
- 判定：**可达真缺陷 P2**。删除断点为破坏性操作（删除后任务须从头重跑），真实触发且完全无二次确认/撤销；工程内 DataManagement.tsx 已有 Modal.confirm 对照模式，修复成本低。
- 修复建议：`Augmentation.tsx` 的 `doDeleteCheckpoint`（:64-73）与 Tag `onClose`（:298-315）删除前加 `Modal.confirm({title:"确认删除断点",content:"删除后该任务将从头重跑，确定继续？",...})`，onOk 后再调 `deleteCheckpoint`；UI 文案与 DataManagement.tsx:281-295 一致。

### A3 删除配装预设（laiyipao H5，light/dark 双主题）

- 登录后进入背包页：`.preset-delete` 可见元素数 0（当前共享账号无配装预设数据），点击未执行，三时点 DOM/obs/AX 均无变化，页面错误 0。
- 判定：**不可达 L1**。无真实数据可触发删除，据实不得判缺陷。

### 反证边界

- 静态候选 3 项均逐一真浏览器取证；排除项（me/battle/shop/admin ContentView）按已有确认机制不重复实测；未覆盖的其它删除入口不预判。

## 5. 三分判定汇总

| 判定 | 数量 | 项目 |
|---|---|---|
| 可达真缺陷 | 1（P2） | A2 augmentor 删除断点无二次确认 |
| 不可达 L1 | 2 类 | A1 重铸道具 0 被业务守卫挡住；A3 无预设数据 |
| 正常规范预期 | 0 项实测命中 | 排除项已有确认机制（静态对照，不重复实测） |

## 6. 截图与 SHA-256（命令行 sha256sum 核验）

| 文件 | 说明 | SHA-256 |
|---|---|---|
| pc-aug-delete-before-r502.png | A2 点击前：断点列表 1 个 tag | d90be4c7ee3578427e7ebdcf991f0d382cca7d76d0e96b727e2554e33f75d560 |
| pc-aug-delete-after-r502.png | A2 点击后：断点消失 + 成功 toast | 948eca130c5eaed4d3ef7fa0849a211672196cec3153797556fb9b3d2727d136 |
| mob-mastery-reset-light-r502.png | A1 light：点重铸后状态不变 | 3f93dad06add0263e8e1f56ba42f29f2c5412011ddb9e9e1a4fca866d1bc173f |
| mob-mastery-reset-dark-r502.png | A1 dark：同上 | 3f93dad06add0263e8e1f56ba42f29f2c5412011ddb9e9e1a4fca866d1bc173f |
| mob-bag-delete-light-r502.png | A3 light：背包页无预设 | 712b0d325fb0c0e388dbd3a0a0152ae655471e7764b1bb9e05a6bcc996de82d4 |
| mob-bag-delete-dark-r502.png | A3 dark：同上 | 712b0d325fb0c0e388dbd3a0a0152ae655471e7764b1bb9e05a6bcc996de82d4 |

> 双主题同 SHA 据实说明：`augmentor/web/src/App.tsx:119` 写死 `theme="dark"`，无主题切换入口，故 A2 只在单一暗态实测（PC 双主题不适用）；`laiyipao/miniapp/src` 无 theme/darkmode localStorage 机制（静态 grep 无命中），A1/A3 注入的 theme 参数无效，故 light/dark 截图同 SHA。均据实披露，不冒充不同主题视觉；判定依据为 DOM/状态采样，不依赖明暗。

## 7. 脱敏与边界

- 所有服务器地址、SSH 主机与登录凭据均以 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` / 环境变量占位；证据脚本与报告均位于 gitignored `oa4rust/test-results/`。
- 正式站 API key 与测试账号口令仅 runner 运行时从测试服进程环境读取，未写入任何入库文件、日志或提交信息。
- 本维度为只读甄别，不修改产品代码；修复项交由后续 RED→GREEN 轮次处理。