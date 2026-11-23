# R426 H5 推送注册 / 假期申请写链误报成功甄别（只读据实）

## 0. 结论与判据

- 本轮仅审计两个 H5 showToast(icon:success) 写链候选，**未改任何代码**（kind=work 只读甄别）。
- **三分判定：0 个用户可达真缺陷**（误报前提不成立）。两写链后端失败均为非 2xx（401/400）或恒 HTTP200+type:success，不存在真 UI 可触发的 HTTP200+type:error 失败信封；前端不读 type 的风险登记为 L1 隐患，真后端不产生该形状，不判缺陷。
- 三全口径：真 UI 可复现（真实 @tap 入口+DOM 可达）+ 真请求响应体（page.on(response) 捕获真后端响应体铁证）+ 源码定位（前端函数行号+后端 handler 行号）。
- 真父=HEAD 5573e48395ab92e2b50ab150e3d545af665c043d，分支 opt1000/oa4rust-pc-ai-refactor。
- 测试环境：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，真值仅存在于 gitignored 证据，不入库、不回显。

## 1. 源码甄别（逐函数真实可达 / 剔除项）

- 候选1 mine.vue `registerPush`（:40-56）：:41 session.user.unique 守卫，:44 await pushApi.deviceCreate({userId, platform: 'h5', token: clientDeviceToken()})，:49 pushRegistered=true，:50 `uni.showToast('推送注册成功', icon:'success')` **不读响应 type**；catch(:51) 仅捕获非 2xx 抛错走红 toast。模板 :126 `<button @tap="registerPush" :disabled="pushBusy">` 真实可达，门控 grep（EndpointFold/devToolsEnabled/import.meta.env.DEV/defineExpose）在两页均为 0，E2E DOM pushBtns=1/fold=0 确证。
- 候选2 attendance.vue `applyLeave`（:96-112）：:97 空日期守卫（红 toast 请选择假期日期并 return），:103 await attendanceApi.selfHolidayApply({holidayDate, reason})，:104 `uni.showToast('假期申请已提交', icon:'success')` **不读响应 type**，:105-106 清空表单；catch(:107) 仅非 2xx 走红 toast。模板 :233 `<button @tap="applyLeave" :disabled="leaveBusy">` 真实可达，E2E DOM leaveBtns=1/fold=0 确证。
- 严格剔除：mine.vue `changePassword`（:62 起，已有 resp.type 守卫，队长备料注明正确模式）、calendar/bbs-new/doc/collect/process 三写链（R422/R423/R425 已判 0）、recycle（R424 已修）、meeting act/start submitStart/topic send 及两页全部 load*/open* 只读展示类。
- SDK：mobile/src/services/index.ts :577-578 `deviceCreate`→POST /api/jpush/device/create；:291-292 `selfHolidayApply`→POST /api/attendance/assemble/control/selfholidaysimple；两调用均**不带** discardResponse。http.ts `parseEnvelope`(:67-77) 仅 JSON.parse 不查 type/success；`performRequest`(:186-191) 2xx 直接 resolve(parsed)，仅 401 refresh / 403 PermissionError / 其它非 2xx 抛错（:194/:204/:208）。故 200+type:error 不抛错会走成功分支——但前提是后端产生该形状，本轮证伪。

## 2. 后端精确行与失败信封（真后端假数据探针据实核实）

- jpush `device_create`：backend/crates/jpush/src/lib.rs :120-160。:125 `require_session(session)?` 无会话→AppError::Unauthorized（401 非 2xx）；:130 userId 取会话 person_unique（客户端 userId 被覆盖，防推送劫持）；:131-140 platform/token `unwrap_or("")` 空值不报错；:144-150 INSERT...map_err 仅 DB 异常 500；:152 `ActionResult::success{id,userId,platform,token}` 恒成功。**全函数无 ActionResult::error 分支**。路由 jpush/src/routes.rs :14 POST /api/jpush/device/create。
- attendance `selfholidaysimple_create`：backend/crates/attendance_assemble_control/src/lib.rs :4939-4980。:4957-4959 holidayDate 空→AppError::BadRequest（400 非 2xx）；:4946-4952 personId 归属/管理员校验非本写链失败信封；:4961-4969 INSERT...map_err 仅 DB 异常 500；:4971 `ActionResult::success{id,docId,personId,holidayDate,created}` 恒成功。**全函数无 ActionResult::error 分支**。路由 attendance_assemble_control/src/routes.rs :178 POST /api/attendance/assemble/control/selfholidaysimple。
- 队长备料行号校正：jpush device_create 真实 handler 为 :120（队长给 :120 与 :156 两处，:156 实为 template_list 边界，本链仅 :120）；selfholidaysimple_create 确为 attendance_assemble_control crate :4939（非未知 crate）。均经精确文件 grep 坐实，未依赖 glob。
- ActionResult::error 为 o2server 兼容的 HTTP200 失败信封（shared/response.rs），但两 handler 均不产生它。

## 3. 真后端假数据探针（job exit 0，证据 gitignored probe.json）

- jpushNoSession（无会话）= HTTP401 type:error unauthorized；jpushFakeData（带假 userId/token）= HTTP200 **type:success**（userId 被会话覆盖为 admin，真写入 1 台设备）。
- leaveNoSession（无会话）= HTTP401 type:error unauthorized；leaveEmptyDate（空日期）= HTTP400 type:error 'holidayDate is required'；leaveFakeDate（假日期）= HTTP200 **type:success**（created:true，真写入 1 条假期申请）。
- 结论：两写端点均**无 HTTP200+type:error 失败形状**。破坏性真写未以避免，探针真写入共 2 条测试数据据实披露（假设备 1 台 + 假假期 1 条）。

## 4. 真浏览器 E2E 决定性对照（390x844 headless isMobile/hasTouch DPR2，非物理真机）

- 手段：沿用 H5 audit 历史验证路径（r422/r423/r425），Playwright headless Chromium，390x844 isMobile/hasTouch deviceScaleFactor=2，page.on(response) 捕获写请求响应体；写响应默认 **r.continue() 全程走真后端不拦截**，仅注入变体用 r.fulfill 作 L1 佐证。
- S0 DOM：mine pushBtns=1、按钮文本[开启推送/保存新密码/退出登录]、fold=0；attendance leaveBtns=1、hasLeave=true、fold=0。两入口真实可达无门控。
- S1 推送真对照：真点开启推送→真 POST /api/jpush/device/create **不拦截走真后端**=HTTP200 type:success id=…，writesDelta=1，绿 toast「推送注册成功」，按钮变「重新注册」=成功语义与响应一致，**无误报**。
- S2 推送注入佐证：route 注入 HTTP200 type:error(device quota exceeded)→仍弹绿「推送注册成功」=前端不读 type 的 L1 隐患（真后端无此形状，不判缺陷）。
- S3 推送非 2xx：注入 HTTP500→红 toast「HTTP 500」，正确。
- S5 表单校验零写：空日期点提交→writes=[]，红 toast「请选择假期日期」，零写通过。
- S6 假期真对照：填日期+事由→真 POST /api/attendance/assemble/control/selfholidaysimple **不拦截走真后端**=HTTP200 type:success created:true，writesDelta=1，绿 toast「假期申请已提交」，表单清空=成功语义一致，**无误报**。
- S7 假期注入佐证：注入 HTTP200 type:error(already applied)→仍弹绿「假期申请已提交」=L1 隐患（真后端无此形状，不判缺陷）。
- S8 假期非 2xx：注入 HTTP403→红 toast「没有权限执行该操作 / 没有权限」，正确。
- S9 失败态：preCheck 注入 HTTP500→「考勤信息加载失败：HTTP 500」+重试按钮1，与空态（今日暂无打卡记录）可区分。
- 0 pageerror（全程 ev.errors=[]）；totalWrites 见 gitignored evidence.json（含真域名，不入库不回显）。

## 5. 截图证据（sha256 为命令行 sha256sum 原始值；read_image 的 attachmentId 为重编码值不采用）

| 截图（390x844 headless 非物理真机 fullPage） | sha256sum 原始值 |
| --- | --- |
| r426-push-real.png | 9ac5a136cc7f705f72430451eee0e98b90596a7e2d89d7d2d1f60af484415486 |
| r426-leave-typeerror.png | ad1e384b650e4cd868deff84f311e78ffed029ea0a3b01f90ae898d8872c149c |
| r426-push-typeerror.png | c2d9dea053c6143ee3f5e5f6827e00d88feb0cc71ede4dd5c03b3f61e73374bf |
| r426-push-500.png | ea938e63766ac53e62708af6479e89dbd353ee6ff3377f268f8a2578097f62fe |
| r426-leave-403.png | af1da1ae43800f2e201afd74fe969c7c2dec2e03566c6002a07ba4c9bb84ac6f |
| r426-fail-state.png | 93656c23168097632d54dcb837c1fbcca3f763c9ef8269c8df4d665b9633551c |
| r426-mine-dom.png | 69f61c36e6dbb557fd733d1a71d072dc13ae7ab1387afe09e608997224ccc5eb |
| r426-attendance-dom.png | 91c98777d25471179ed04c220ab22ff0228e149417d18c2ea7f66007ef747e21 |
| r426-leave-real.png | 91c98777d25471179ed04c220ab22ff0228e149417d18c2ea7f66007ef747e21 |

- 目视确认：r426-push-real 绿「推送注册成功」+重新注册按钮（真后端 200 success 一致）；r426-leave-typeerror 绿「假期申请已提交」（注入 200+error 佐证 L1）。r426-leave-real 与 attendance-dom 字节相同系真提交后表单清空+toast 消散，响应体铁证在 evidence.writes。

## 6. 三分判定与最小修复建议

- 0 个用户可达真缺陷：两写链后端失败均为非 2xx（401/400，SDK 抛错→catch 红 toast）或恒 200 success（成功 toast 一致），误报前提不成立，不改码。
- L1 隐患：`registerPush`(:50)/`applyLeave`(:104) 裸 showToast success 不读响应 type；若后端将来改为 HTTP200+type:error 口径会误报。注入对照（S2/S7）已佐证该前端行为，但当前真后端不产生此形状，不判缺陷。建议后续如统一加写响应守卫（type 为主不强求 bool），失败 toast.error 透传不弹成功。
- 产品语义差异：device_create 属主以会话覆盖客户端 userId（安全审计越权修复），假 userId 仍成功属预期；selfholidaysimple_create 缺日期 400 非 2xx，假日期无业务去重仍成功属产品语义。
- 取证限制：破坏性真写未执行（用假数据探针+route 注入替代，据实披露 2 条测试写入）；空态以「今日暂无打卡记录」据实记录；全量按钮按代表性覆盖。

## 7. 交付

- 唯一脱敏文档（本文件），相对路径 docs/ops/oa4rust-r426-h5-push-leave-write-audit.md，敏感扫描 0 真值（<YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位）。
- 证据（E2E 脚本、probe.json、evidence.json、9 张截图）落 gitignored test-results/r426-h5-push-leave-write/ 与 test-results/r426-*.cjs|sh，不入库。

