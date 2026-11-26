# R309 深度功能缺陷审计（8 业务闭环 + 跨页一致性）

## 来源与取证声明
- 执行者：qa-e2e（只读审计，未改任何源码 / 契约测试 / 后端 / 生产 / 清单）。
- 运行环境：部署测试服（域名用 `<YOUR_TEST_DOMAIN>` 占位），PC 入口产物 `index-DlH8Vnfr.js`、H5 入口 `index-4TKb15fX.js`，服务 active。
- 浏览器证据：Playwright Chromium headless，PC 1440x900、H5 390x844（isMobile/hasTouch），真 UI 登录（口令运行时经部署宿主读取，不入库、不回显）。截图为**本地无头浏览器渲染，非物理真机**。
- 证据目录：`oa4rust/test-results/r309-deep-audit/`（gitignored），含 `flow-evidence.json`、`flow-supplement.json`、`flow-confirm.json`、`flow-final.json`、`flow-last.json`、`role-member-confirm.json`、`dom-probe.json`、`modal-probe.json` 及逐页截图。
- 敏感信息：本文域名用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位；0 真值（无口令、私钥、会话、真实姓名/手机/邮箱）。
- 判定原则：仅真实业务失败算缺陷；工具选择器 / 脚本异常 / 前置数据缺失一律标注为**取证限制**，不臆断为 bug。

## 一、总览
8 个业务闭环真 UI 实测：6 项通过（通讯录建部门、工作流发起、笔记新建删除、文件新建回收、论坛建版块发帖、角色新建与成员弹窗），1 项真实缺陷（**会议创建 HTTP 500**），1 项取证受限（日程新建无可用日历）。
跨页抽检 2 项：工作台待审批 vs 待办为截断展示口径（非矛盾）、H5 消息未读与 PC 一致（count=0）。
全链路 pageErrors=0；唯一非 2xx 为会议创建 500（4 份证据合计稳定复现 4 次）。

## 二、真实缺陷

### D1【P1】会议创建稳定 HTTP 500（新建会议完全不可用）
- 现象：PC `/app/meeting` 填写标题 + 起止时间后点「预订会议室」，`POST /api/meeting/assemble/control/meeting/create` 返回 **HTTP 500**、响应体为空；「我的会议」列表无回显。稳定复现 4 次（flow-evidence / flow-supplement / flow-confirm / flow-final）。
- 抓包 payload（flow-final.json，脱敏后仅结构）：`{"title":"<TS>","content":"","startTime":"2026-11-21T10:30","endTime":"2026-11-21T11:30","roomId":"","creator":""}`。前端时间格式合法、字段齐全（creator 由后端会话覆盖，空串非根因）。
- 源码定位（写操作，本任务只读未改）：
  - 前端提交：`oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:235-242` createMeeting mutation；表单定义 `:158`、`:21-22` datetime-local。
  - 后端路由：`oa4rust/backend/crates/meeting_assemble_control/src/routes.rs:54` 绑定 `create_meeting`。
  - 后端 handler：`oa4rust/backend/crates/meeting_assemble_control/src/lib.rs:2467` `create_meeting`，INSERT 在 `:2502`：`INSERT INTO x_meeting (... start_time, end_time ...) VALUES ($1..$7, NOW())` —— **时间参数 `$5/$6` 无类型转换**。
  - 对照实现：`oa4rust/backend/crates/meeting/src/lib.rs:293` 同表 INSERT 用 `$5::text::timestamp, $6::text::timestamp`（有显式 cast）。
- 根因（基于源码对比的高置信推断，后端错误日志未取到）：`meeting_assemble_control` 版本的 INSERT 缺 `::text::timestamp` cast，前端 datetime-local 的 ISO 字符串（`2026-11-21T10:30`）无法直接绑定到 `timestamp` 列，触发数据库错误 → 500。两 crate INSERT 唯一实质差异即该 cast。
- 复现路径：登录 → `/app/meeting` → 填标题 + 起止时间 → 「预订会议室」 → 观察 500。
- 修复方向（交实施者，test-first）：给 `meeting_assemble_control/src/lib.rs:2502` 的时间占位符补 `$5::text::timestamp, $6::text::timestamp`，与 `meeting` crate 对齐；并补一条「datetime-local 格式字符串可成功创建会议」的后端集成测试（RED→GREEN）。
- 证据：`oa4rust/test-results/r309-deep-audit/flow-final.json`（REQ/RES 抓包）、`flow-confirm.json`（全字段仍 500）、`C3c-meeting-form.png` / `C3c-meeting-after.png`。

## 三、待判 / 低优先级（需产品确认，不直接判 bug）

### D2【低】笔记删除成功但回收站不可见
- 现象：PC `/app/note` 新建笔记回显 PASS；点「确认」后 `POST /api/cms/core/entity/note/delete/{id}` 返回 HTTP200 `deleted:true`、列表移除该条（残留 0）；但 `/app/recycle` 的 `GET /api/recycle/list` 仍 `data:[]`，回收站看不到该笔记。
- 源码：`oa4rust/backend/crates/cms_core_entity/src/lib.rs:483` 软删除 `UPDATE {table} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL`；note 列表 `:410` 按 `deleted_at IS NULL` 过滤。回收站端点是否聚合 `x_cms_note` 未在本次只读范围确认。
- 判定：删除功能本身正常；「是否应进入统一回收站」属产品口径，可能设计如此。建议实施轮次核实 `recycle/list` 的数据来源并明确「笔记删除」预期；若预期应进回收站，则按 RED→GREEN 修复。
- 证据：`flow-confirm.json` C5c steps、`C5c-note-recycle.png`。

### D3【低】通讯录无「新建人员」入口
- 现象：PC `/app/directory` 仅提供 `org-create-unit-btn`（新建部门，回显 PASS）；`person-create-entry` 计数 0，无新建人员 UI；人员来自 `POST .../person/list/filter`（21 条真实数据）。
- 判定：属功能边界/设计口径（人员经组织域或后端导入），非本次重构引入的回归；若产品要求界面直接建人，需另立需求。
- 证据：`dom-probe.json`、`flow-evidence.json` C1。

## 四、取证限制（非缺陷，前置数据 / 脚本因素）
- C2 日程新建：`GET /api/calendar/calendar/list/my` 返回 `my_calendars:[]`，无可用日历；`CalendarApp.vue:253` 的建日历为 prompt 调试入口，创建日程需 activeCal。无前置日历数据 → 无法走通，非前端 bug。
- 会议楼栋/会议室：`GET .../meeting/assemble/control/building/list` 返回 `data:[]`；后端 create_meeting 仅校验 title/startTime/endTime，roomId 可为 None，楼栋空与 D1 的 500 无直接因果。
- 角色成员授权：UI 为只读可视化（`RoleManager.vue:64` 空态文案「该角色暂无分配成员，可在组织架构中完成分配。」）。成员弹窗本身工作正常（`.table-row` 选择器下 8 行、成员按钮存在、弹窗打开、count=0）。授权在组织架构完成，属设计边界。
- 脚本因素：多轮删除/成员校验失败系选择器 or confirmMsg（自定义 DOM 弹窗 `#oa4-confirm-overlay`，非原生 confirm，Playwright dialog 事件拦不到）定位问题，修正后对应闭环均 PASS，不计入缺陷。

## 五、跨页一致性抽检
- X1 工作台待审批 vs 待办：工作台概览「5 待审批」、Dashboard 待办列表 5 条、工作流待办全量 11 条。非矛盾——`Dashboard.vue:334` `pendingItems.value = ((resp as any)?.data ?? []).slice(0, 5)` 明确截断前 5，DR-02 注释（`:320`）声明统计卡与列表同源。属截断展示设计。
- X2 H5 消息未读：H5 用正确路由 `#/pages/login/login` 登录后到达工作台，body 含未读区；`/api/message/unread/count/im` count=0，与 PC 工作台「0 未读消息」一致。首轮用根路径导致停在登录页属脚本路由错误，非缺陷。
- 回收站 PC/H5：PC `/app/recycle` 与 H5 回收站入口均可见；本轮无跨端数据不一致证据。

## 六、8 闭环最终判定表
| # | 闭环 | 判定 | 关键证据 |
|---|---|---|---|
| 1 | 通讯录新建部门 | PASS | org-create-unit 建部门回显；新建人员入口缺失见 D3 |
| 2 | 日程新建 | 取证限制 | my_calendars=[]，无前置日历 |
| 3 | 会议新建 | **FAIL（D1 真实缺陷）** | POST meeting/create 500，抓包确认 |
| 4 | 工作流发起 | PASS | processing/work HTTP200，我发起的列表回显 |
| 5 | 笔记新建删除 | PASS（回收站可见性见 D2） | note/delete HTTP200，列表残留 0 |
| 6 | 文件新建回收 | PASS | files-recycle-list 回收站=1 |
| 7 | 论坛建版块发帖 | PASS | bbs/subject/create HTTP200，刷新 my 页持久 |
| 8 | 角色新建/成员弹窗 | PASS | role 创建回显；成员弹窗打开（授权只读属边界） |

## 七、结论与建议
本轮发现 1 个 P1 真实缺陷（会议创建 500，新建会议功能完全不可用），建议下轮按 RED→GREEN 优先修复（补时间 cast + 集成测试）。另有 2 个低优先级口径问题（笔记回收站、通讯录建人入口）需产品确认。其余失败均为取证限制或脚本因素，不构成缺陷。项目持续迭代，非完成态。