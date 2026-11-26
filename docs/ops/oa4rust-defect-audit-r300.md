# R300 全栈真实缺陷审计清单（第一轮）

- 审计日期：2026-11-21（Asia/Shanghai）
- 分支：`opt1000/oa4rust-pc-ai-refactor`（HEAD `dc9566715c57c91da491f15a6b4b5ce6e33aa50d`；审计期间未改任何实现代码）
- 关联基线：`docs/ops/oa4rust-page-audit-r300.md`（R300 A2 真浏览器逐页批判，commit `5bf8fbed7`）；本文档把其中的 P0/P1 现象升级为可验证缺陷清单。
- 审计范围：desktop / H5 / 后端接口交叉定位；按队长指示只审计不修实现，真 bug 留待后续 RED→GREEN 轮次。
- 来源声明：全部现象来自 tabbit 真浏览器（Tabbit Browser/Chromium）访问测试服 `https://<YOUR_TEST_DOMAIN>/oa4rust`（PC）与 `https://<YOUR_TEST_DOMAIN>/oa4rust-h5`（H5），非本地 dev server、非静态假数据、非真机。测试账号经登录页 UI 登录，口令来自 `laiyipao/deploy/.local.keys`（不入仓、不回显）。
- 证据目录：`Temp/qa-e2e-r300/`（运行产物，被 .gitignore 忽略，不入库）；本文档引用其文件名作为证据索引。

## 审计方法

1. 以 R300 A2 真浏览器逐页截图与结果 JSON 为现象基线；
2. 对每条现象在源码中做静态交叉定位（前端页面 → SDK 传输层 → 后端路由/处理器），给出精确文件与行号；
3. 如实区分「源码可直接判定的确定根因」与「受取证限制只能给出待验证方向」两类，不猜测；
4. 每条缺陷给出：现象、真浏览器证据、源码定位、可复现路径、修复方向、验证方式。

## 一、P0 缺陷（阻断业务正确性，优先 RED→GREEN）

### P0-1 BAM 统计加载失败：`["bam","list"] data is undefined`

- 现象：PC `/app/bam` 页面出现错误条 `BAM 统计加载失败：["bam","list"] data is undefined`（R300 截图出现 2 次），统计卡无法呈现真实值。
- 真浏览器证据：`Temp/qa-e2e-r300/live-bam.png`；`pc-live-results.json`。
- 源码定位：
  - 查询：`oa4rust/frontend/apps/desktop/src/views/BamApp.vue:600-623`，queryKey `['bam','list']`，queryFn 调 `GET /api/processplatform/assemble/bam/state/summary`，`const d = ((r as any)?.data ?? {})`，各统计字段再 `?? 0`。
  - 错误呈现：`oa4rust/frontend/apps/desktop/src/views/listQuery.ts:7-11`（`describeListError` 取 `error.message` 拼前缀）、`:65-68`（`useListError` 由 `isError` 生成 `loadError`）；BAM 侧 label 见 `BamApp.vue:628-631`。
  - 传输层：`oa4rust/frontend/packages/sdk/src/api.ts:82-89`（`parseJsonEnvelope` 仅做 JSON 解析，不校验 `data` 是否存在）；返回点 `:296/:365/:508`。
  - 后端：`oa4rust/backend/crates/processplatform_assemble_bam/src/lib.rs:306` 注册 `GET /api/processplatform/assemble/bam/state/summary`；`state_summary` 返回真实 `x_work`/`x_task` 统计（约 `:2410-2450`）。
- 取证限制与判定：`data is undefined` 文案在 oa4rust 全仓源码（*.ts/vue/js）grep 无命中；当前源码 queryFn 已有 `?? {}` 与 `?? 0` 兜底，理论上不会因 data 缺失而抛出该错。错误文案含 queryKey 序列化前缀 `["bam","list"]`，判断来自 TanStack Query 对 queryFn 异常的包装。因此本项**不能**直接判定为某一行源码 bug，需核实「测试服部署产物与当前源码版本是否一致」。
- 可复现路径：真浏览器登录 PC → 导航 `/app/bam` → 观察错误条；刷新后仍复现。
- 修复方向（不在本任务实施）：
  1. RED：新增契约测试，mock summary 接口返回异常信封/字段缺失，断言页面不把底层 `error.message` 直出为 `data is undefined`；
  2. GREEN：SDK 对 2xx 响应做 `data` 存在性校验并给出业务可读文案；`BamApp` queryFn 显式归一化 summary 字段；`useListError` 清洗 queryKey 序列化前缀；
  3. 先重新部署确保测试服产物与源码一致，再判断是否为部署滞后。
- 验证方式：修复并重新部署后，真浏览器打开 `/app/bam`，断言统计卡显示真实数值且无 `data is undefined`，契约测试由 RED 转 GREEN。

### P0-2 考勤统计卡为硬编码假数据，与明细不一致

- 现象：PC `/app/attendance` 统计卡显示 45 应出勤 / 42 实际出勤 / 3 迟到 / 2 请假，但明细列表显示「暂无考勤记录」。
- 真浏览器证据：`Temp/qa-e2e-r300/live-attendance.png`。
- 源码定位：`oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue:329-334`，`stats` 为 `computed` 直接返回常量数组 `45/42/3/2`（UI 假数据）；明细走真实接口 `GET /api/attendance/assemble/control/attendancedetail/filter/list?month=&page=&size=20`，取 `r.data?.list`（`:338-353`）。
- 根因判定：确定（源码直读）。统计卡与明细数据源天然分离——统计是写死的常量，明细是接口真实数据，二者不可能一致。
- 可复现路径：真浏览器登录 PC → `/app/attendance` → 对比统计卡与列表；切换月份后列表仍空而统计数字不变。
- 修复方向（不在本任务实施）：RED 契约测试断言「统计值由同月真实数据派生」；GREEN 移除硬编码，统计改由明细接口聚合或后端统计端点提供，无数据时统计归零而非固定值。
- 验证方式：真浏览器切换月份/增删记录后统计与明细同步变化；契约测试 GREEN。

### P0-3 H5 论坛 / 搜索 / 我的三页「连接服务器超时，点击屏幕重试」

- 现象：H5 `#/pages/bbs/bbs`、`#/pages/search/search`、`#/pages/mine/mine` 显示「连接服务器超时，点击屏幕重试」，bodyTextLen=17，页面实质不可用。
- 真浏览器证据：`Temp/qa-e2e-r300/h5-bbs.png`、`h5-search.png`、`h5-mine.png`；`h5-results.json`、`h5-chain-results.json`。
- 源码定位：`oa4rust/mobile/src/services/http.ts` 为 H5/移动端统一传输层（`uni.request`，30s 超时，H5 同源 Cookie），`transportFailureMessage` 对 `errMsg` 中 `/timeout/i` 归一为「连接服务器超时」文案。
- 取证限制与判定：tabbit 执行器在 message/attendance/meeting/browse/recycle/contacts/search/mine 多页出现 exit=70（`Evaluation exceeded hard deadline of 12000 ms`），属工具链超时，不能据此判定业务缺陷；仅 bbs/search/mine 三页有落盘截图确认真浏览器渲染出超时文案。三页是「业务接口真超时」还是「测试服反向代理超时」本轮未抓到实际请求 URL 与耗时，需后续取证。
- 可复现路径：真浏览器 H5 登录 → 依次点击论坛/搜索/我的 → 观察超时态。
- 修复方向（不在本任务实施）：先用真浏览器抓三页实际请求 URL 与耗时区分根因；若接口 30s 未返回则排查后端对应端点，若为 `/oa4rust-api` 反向代理超时则调整网关阈值；前端补明确重试引导与离线态。
- 验证方式：修复后真浏览器复测三页，断言不再出现超时文案且渲染真实数据。

## 二、P1 缺陷（数据/体验明显退化，需排期修复）

### P1-1 PC 工作台待审批 0 与 H5 的 9 不一致

- 现象：PC `/app/dashboard` 统计卡待审批为 0，H5 工作台显示「9 待办审批」。
- 真浏览器证据：`Temp/qa-e2e-r300/live-dashboard.png`、`h5-home.png`。
- 源码定位：`oa4rust/frontend/apps/desktop/src/views/Dashboard.vue:219-225`，`loadPending` 走 `GET /api/processplatform/assemble/surface/task/list/my/paging/1/size/20`；H5 走 mobile 侧自有待办接口。两端数据源不同。
- 取证限制：本轮 PC 标签因工具链原因丢失，两端待办接口的实际响应未取证，故仅标记「数据源不一致待核对」，不判定具体数值错误归属。
- 修复方向：真浏览器抓两端待办接口响应对比，统一数据源或明确口径差异；PC 统计卡应与待办列表同源。
- 验证方式：同一账号两端登录，待办数一致。

### P1-2 PC 多页首屏数据缺失，落后于 H5

- 现象：PC `/app/org` 主体长期 `Loading...` 且顶部 `端点调试 98`；`/app/directory` 显示「共 0 人，请先加载名册」；`/app/file`、`/app/bbs`、`/app/meeting`、`/app/note`、`/app/recycle` 均为空态。对比 H5 通讯录可见 `E2E001…E2E014` 真实人员数据、待办为 9。
- 真浏览器证据：`Temp/qa-e2e-r300/live-org.png`、`live-directory.png`、`live-file.png`、`live-bbs.png`、`live-meeting.png`、`live-note.png`、`live-recycle.png`；`h5-contacts.png`。
- 源码定位：各页首屏查询与加载策略分散（如 `directory` 需手动加载名册、`Dashboard` 待办接口见 P1-1），本轮未逐页定位到统一根因，列为「按 H5 真实数据源补齐 PC 首屏」的改进项。
- 修复方向：PC 首屏改为自动加载真实数据；Loading 增加超时兜底与明确空态引导；端点调试入口从业务页移出。
- 验证方式：真浏览器逐页断言首屏呈现真实数据或有引导的空态，无长期 Loading。

### P1-3 端点调试侵入业务页

- 现象：`/app/org` 显示「端点调试 98 展开查看全部端点」，`/app/note` 显示「端点调试 1」，调试入口抢占业务版面并暴露内部接口数量。
- 真浏览器证据：`Temp/qa-e2e-r300/live-org.png`、`live-note.png`。
- 源码定位：`oa4rust/frontend/apps/desktop/src/views/BamApp.vue` 等页以 `EndpointFold` 组件收纳调试按钮（BAM 约 25 个）；org/note 同类接入。
- 修复方向：调试入口统一收进开发者面板，生产/测试服业务页默认不展示。
- 验证方式：真浏览器逐页确认业务页无调试痕迹。

## 三、P2 体验缺陷

1. 可见「已切换到：XXX」无障碍 live 提示泄漏到视觉文本（截图 `live-settings.png`）：应在视觉层隐藏、仅保留给读屏软件。
2. 流程/待办直接展示原始 UUID 与内部 ID（如 `start:S3 运行闭环 3848b8d6-cd2d-4179-85a6-b53b84b8601c`，截图 `live-process.png`）：应展示可读业务标题，ID 折叠到详情。
3. 多处空态（消息、日历、文件、搜索）缺少引导文案与操作入口。

## 四、取证限制与不确定性（如实标注）

- 本轮 PC 标签因 tabbit `finish --discard` 会关闭标签而丢失，BAM/考勤/待办三处接口的实际 HTTP 响应未抓到；相关根因判定仅依据源码静态交叉定位 + R300 真浏览器现象，已逐条标注确定性等级。
- BAM 错误文案全仓源码无命中且现有 queryFn 有兜底，存在「测试服部署产物滞后于源码」的可能，未做猜测性定论。
- H5 message/meeting/browse 多次采集因 tabbit 执行器超时未稳定落盘，不作为业务缺陷证据；仅以稳定性问题记录。
- 所有截图来自 tabbit 无头浏览器（非真机）；H5 以桌面宽视口渲染，仅作页面可达性与内容批判，不等同真机表现。

## 五、结论与后续

本轮把 R300 A2 逐页批判升级为可验证缺陷清单：3 项 P0（BAM 加载失败、考勤假数据、H5 三页超时）、3 项 P1（工作台待办口径、PC 首屏数据、端点调试侵入）、3 类 P2 体验问题，并逐条给出证据、源码定位、复现路径、修复方向与验证方式。所有真 bug 须在后续轮次以 test-first RED→GREEN 实施，本轮只审计不修实现。本轮仅为 1000 轮持续迭代中的第一轮审计记录，非项目完成态。