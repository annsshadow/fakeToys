# R422 H5 日历写链 200+type:error 误报成功甄别审计

> 本审计基于 390x844 headless Chromium（isMobile/hasTouch/DPR2）真浏览器实测，**非物理真机**。所有截图为无头浏览器渲染，非真机截图。域名/主机一律以 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位，真值仅留 gitignored 证据目录，不入库、不回显。

## 一、审计范围

- 目标：H5 端 `oa4rust/mobile/src/pages/calendar/calendar.vue`（532 行）三条写链，甄别后端 HTTP 200 + `type:error` 业务失败下是否误弹成功 toast。
- 三写链：`submitCalendar`（创建日历，模板 302 行 @tap）、`submitEvent`（保存事件，模板 317 行 @tap）、`removeEvent`（删除事件，模板 355 行 @tap）。
- 只读剔除：`load`（GET calendar/list/my + PUT event/list/filter）、`openEvent`（eventDetail）、`switchRange`/`fmtTime`/`toggleCompose`/`toggleCalendarForm`、loadError 展示、`onShow` 鉴权。均不弹成功写 toast，不判缺陷。
- 无 EndpointFold/devToolsEnabled/import.meta.env.DEV/defineExpose 门控，生产真渲染；E2E DOM 审计 createCalBtn=1/composeBtn=1 真实可达。

## 二、源码甄别

- `submitCalendar`（71-93）：第 80 行 `calendarApi.create(name)` → 81 `const created=resp?.data` → 82-83 `if(created)` 校验 → 86 `showToast(日历已创建, success)`。不读响应 `type`。
- `submitEvent`（100-132）：103-114 表单校验 → 117-123 `calendarApi.coreEventCreate(payload)` → 124 `showToast(事件已创建, success)`。不读 `type`，catch（128）仅非 2xx。
- `removeEvent`（134-158）：140-147 `uni.showModal` 确认守卫 → 149 `coreEventRemove(id)` → 150 `showToast(已删除, success)` → 151 清详情 → 152 `load()`；catch（153-154）`showToast(删除失败, none)`。
- services/index.ts：441 create=POST `/api/calendar_assemble_control/calendar`；458-459 coreEventCreate=POST `/api/calendar/event/create`；461 coreEventRemove=POST `/api/calendar/event/remove`。
- SDK `mobile/src/services/http.ts`：performRequest（159-220）仅非 2xx 抛错（401 refresh / 403 PermissionError / 其它 httpStatusError）；2xx → parseEnvelope（67-77）原样返回，不查 `type`/`success`。buildUrl（106-108）同源代理，apiBase 默认空。

## 三、后端失败信封（真源码 + 真探针）

- 真实 H5 端点 handler 在 `backend/crates/calendar/src/lib.rs`（非 calendar_core_entity，后者路径 `/api/calendar/core/entity/*` 不同）。
- `event_create`（437-490）：缺字段全 `AppError::BadRequest` 非 2xx；INSERT 成功恒 `ActionResult::success`（489）；无 200+error 失败分支。
- `event_remove`（564-615）：id 缺 BadRequest；查不到 → `AppError::NotFound`（582）即 HTTP 404 非 2xx；成功恒 success（614）。`grep ActionResult::error calendar/src/lib.rs` 命中数=0。
- `calendar_create`（`backend/crates/calendar_assemble_control/src/u2.rs` 335-397）：INSERT 恒 `ActionResult::success`（390），无 error 分支；路由 routes.rs 29-30 POST `/api/calendar_assemble_control/calendar`。
- 路由 `calendar/src/routes.rs`：27 POST `/api/calendar/event/create`；29 POST `/api/calendar/event/remove`。
- 真后端假数据探针（gitignored `test-results/r422-h5-calendar-write/probe.json`）：
  - eventCreateMissing → 200 `type:success`（后端无字段语义校验，直接 INSERT，真写入一条测试事件）；
  - eventCreateFakeCal → 200 `type:success`（假 calendarId 无外键门禁）；
  - eventRemoveFake → **HTTP 404 `type:error`**（message=not found，prompt=ExceptionEntityNotExist）；
  - calendarCreateEmpty → 200 `type:success`（空名照建）；calendarCreateNormal → 200 `type:success`。
- 结论：三写端点均不产生「HTTP 200 + type:error 业务失败」信封。创建类失败要么非 2xx，要么根本不校验直接成功；删除类失败是非 2xx 404 → SDK 抛错 → catch → `删除失败` 正确提示。**误报前提不成立。**

## 四、真浏览器实测（390x844 headless 非物理真机）

- 决定性对照（写响应全程走真后端不拦截，`page.on(response)` 抓新写请求响应体）：
  - 真点「创建日历」填名 → 真 POST `/api/calendar_assemble_control/calendar`=200 success，writesDelta=1，toast=`日历已创建`（与后端一致，无误报）。
  - 真点「保存事件」填合法表单 → 真 POST `/api/calendar/event/create`=200 success，writesDelta=1，toast=`事件已创建`（一致，无误报）。
  - 真点删除事件 + 真 uni-modal → 假 ID 删除走真后端 = HTTP 404 error，SDK 抛错，不会弹 `已删除`。
- 取消零写：注入列表 1 行假事件 → 真点删除 → 真点 uni-modal「取消」按钮（`.uni-modal__btn_default`）→ `/api/calendar/event/remove` 写请求 writesDelta=0，详情仍在，零写通过。
- 非 2xx：PUT `event/list/filter` 注入 500 → 显示 `部分日历日程加载失败：HTTP 500` + 重试按钮 1；空 data → `今日暂无日程` 空态，可区分。
- 注入对照变体（人造 200+type:error，仅佐证前端不读 type；真后端不产生此形状，不单独判缺陷）：创建/保存仍弹成功。
- `pageErrors=[]`，totalWrites=4（两条真实写 + 探针 + 取消守卫）。
- 关键截图 sha256（1440 非本任务；实际 390x844 headless 非物理真机，已目视确认）：
  - r422-calendar-create.png = d6f49f91c35dee414f3012e3a9c8d7ffe62583f2515775e4cb5e6b24bb3a3a64（绿✓ 日历已创建）；
  - r422-event-create.png = c61b76d187c1536a7b466a7cf6cc91a1a57ca4c9a767d298eaa8a4fb3c8b9549（绿✓ 事件已创建）；
  - r422b-cancel.png = dd41d1183f4a885094526e38bc7c76afcff9012372ecc4966dec04ee46d021d8（uni-modal 删除确认弹窗，取消后零写）；
  - r422b-error-state.png = c90d4c3ccf024587dbf42ae715dfed1a427029ba868b2d3f4df31030648a4771（部分日历日程加载失败：HTTP 500 + 重试）；
  - r422-dom.png = 893c4a7941154cf55ec1aba1aedc6349145e866742c1c6a206f4b2d6a19f7f81（入口 DOM 审计）。

## 五、三分判定

- **真缺陷（P1/P2）：0 个用户可达。** 三写链虽不读响应 `type`，但后端不产生 200+type:error 失败信封，误报成功的用户可达路径不存在。
- **L1 隐患（据实记录，非本轮缺陷）：** 三写函数均不读 `type`，若后端口径变更为 200+type:error（如其它模块 o2server 兼容风格），则创建/保存会在业务失败时误弹成功；人造 stub 已证实此风险。当前真后端无此形状，不改码。
- **正确模式：** removeEvent 失败非 2xx → SDK 抛错 → catch → `删除失败`；showModal 取消零写坐实；创建/保存后端恒 success，成功 toast 与响应一致。
- 产品语义差异：创建类 handler 无字段/外键校验（缺字段、假日历、空名均 200 success 真写入），属后端语义，不是误报缺陷。

## 六、取证限制

- 破坏性真写（真删用户数据）未执行：删除链仅在 uni-modal 取消路径坐实零写；假 ID 删除走真后端返回 404（非真删）。
- 创建类探针（eventCreateMissing/FakeCal、calendarCreateEmpty/Normal）产生 4 条真实测试数据写入测试服，为验证后端信封所需，据实披露。
- 注入变体（人造 200+type:error）仅佐证前端不读 type，真后端不产生该形状，不据此判缺陷。
- 全部写按钮按代表性覆盖，未逐一枚举页面所有 tap 入口（本任务聚焦三条写链）。
- H5 历史 E2E 均为 390x844 视口（本轮同），与队长派发文本所写 1440x900 不同，据实以 390x844 标注。

## 七、结论与最小修复建议

- 本轮判定 **0 个用户可达真缺陷**，不改码。R422 关单依据：后端三写端点失败信封非 200+type:error，误报前提不成立。
- 预防性建议（非本轮范围）：若统一防误报，可为三写链新增 `calendarWriteSucceeded(resp)`（type 为主不强求 bool）守卫，失败 toast.error 透传 message、成功才弹成功；并推动后端对创建类补字段/外键校验。
- 证据目录（gitignored）：`test-results/r422-h5-calendar-write/`（evidence.json / probe.json / extra-cancel.json / extra-final.json / 各截图）。
