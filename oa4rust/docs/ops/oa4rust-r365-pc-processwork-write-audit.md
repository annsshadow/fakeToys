# R365 PC 工作流写操作链甄别报告（ProcessWork.vue）

- 轮次：R365（任务 t178），身份 qa-e2e，只读甄别，未改任何源码/契约/后端。
- 对象：`oa4rust/frontend/apps/desktop/src/views/ProcessWork.vue`（3258 行）中 72 处 `toast.success`（任务描述约 33 处为下限）。
- 测试环境：部署测试服（域名见占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`），Playwright Chromium headless 1440x900，**非物理真机**。
- 证据目录（gitignored）：`oa4rust/test-results/r365-pc-processwork-write/`（evidence.json / evidence-v2.json / evidence-v3.json 与截图）。
- 硬约束遵守：破坏性真状态翻转未执行，仅假 ID 探针 + route 注入；凭据经 ssh 运行期读取未回显。

## 1. 覆盖甄别

`docs/ops/` 命中 current-state.md、oa4rust-baseline-contract-mismatch-audit-r304.md、oa4rust-business-work-r300-b4.md、oa4rust-page-audit-r296.md、oa4rust-r314-deep-functional-audit.md、oa4rust-token-consumption-audit-r302.md，均为整体/其它模块审计，无一专项覆盖本页写链的成功 toast 与失败信封一致性。本轮首次专项实测，属有效增量。

## 2. 源码坐实

- 模板（:4-243）共 25 个 `@click`（含 v-for 重复）：openStart / activeTab / query.refetch / openWork / closeWork / viewAttachment / viewRead / pressTask / taskV2Action(pause|resume|reset) / engineTaskAction(will|v3add|processing) / submit(reject|approve) / closeStart / submitStart。模板在 :243 结束，无 surfaceOps3-20 / engineRest / engineRest2-6 / engineTouch / engineReviewWork / engineAttachmentOrder / loadTwinA-H / collabAction / engineWorkAction 的绑定。
- 有 UI 入口的写函数：pressTask(:1326-1339) POST task/press/{id} → toast.success(已催办)；engineTaskAction(:1374-1388) will/v3add/processing 三分支 POST → toast.success(任务操作已提交)；taskV2Action(:3041-3063) pause/resume/reset → toast.success(操作成功)；submit(:3165-3192) PUT data/work + POST task/{id}/complete|reject → toast.success(审批通过|已驳回)；submitStart(:3134-3163) POST work + PUT data/work → toast.success(流程已发起)。这些函数 await 后不读响应体，仅 catch 才 toast.error。
- 无 UI 入口的批量写函数（死代码候选）：collabAction(:1390+)、engineRest(:1608)、engineRest2(:1680)、surfaceOps3-20(:1769-2352)、engineRest3(:2360)、engineRest4(:2415)、engineRest5(:2579)、engineRest6(:2617)、loadTwinA-H(:2647-2875)，各自 toast.success 文案如「流程表面操作已提交」「引擎操作已提交」「流程附件操作已完成」等。全文件无 defineExpose / window 挂载 / 事件总线（grep 无命中），生产构建剥离 devtools 后无外部调用面。
- 后端契约：`oa4rust/backend/crates/processplatform_service_processing/src/lib.rs` 大量写端点用 `SELECT ... FOR UPDATE` + `.ok_or(AppError::NotFound)?`（如 task_id_press:1326-1349、task_id_will:1407），假 ID 走 404；成功返回 `ActionResult::success(row_to_json(&row))` 无 bool 业务失败标志。例外：`work_v2_id_rollback_core`(:800-833) 无快照时返回 `ActionResult::success({id, rolled_back:false})`，且 work_v2_id_rollback(:836-844) 的 u4_require_work_owner 对不存在 work 放行——假 ID 真实可达 200 + success + rolled_back:false，但对应 engineWorkAction(rollback) 无 UI 入口。

## 3. 真浏览器实测（三轮，390 非物理真机对照为 1440x900）

- **S0 DOM 审计（首轮）**：列表 52 个可交互元素；`htmlHasDeadFnName=false`、`htmlHasDeadApiPath=false`，无死代码函数/调试端点渲染入口；待办列表正常渲染。
- **S1 真实后端假 ID 探针（17 端点）**：press/will/v3add/processing/v2pause/v2resume/v2reset/goback/reroute/retract 全部 404 `type:error`（not found）；rollback 200 + `type:success` + `rolled_back:false`（lib.rs:800-833 铁证）；draftDelete 200 + `type:error`（draft not found）；roomPhoto/coreMeetingSave 等会议式端点另见 R364。坐实工作流原生端点 404 口径，与 R359 会议端点 0 行 UPDATE → 200 + bool:false 不同。
- **S2 UI toast 一致性（注入一条待办，点击 7 个有入口写按钮，各 200 响应）**：`type:error`(200) 注入下 7/7 仍弹成功 toast（已催办 / 操作成功 / 任务操作已提交），坐实「await 后不读 type」的误报逻辑；`bool:false` 注入下催办仍弹「已催办」；真 HTTP 500 注入下正确弹「任务操作失败: HTTP 500: Internal Server Error」。即：注入的人造失败信封会误报，但真后端写端点不产生 200 + type:error 形状（错误走 AppError → 非 2xx），当前无用户可达路径。
- **S3 v2 真契约补测**：rollback 假 ID 真探针 200 + success + `rolled_back:false` 再次确认；有入口三按钮在 type:error(200) 下仍误报成功；0 pageerror。
- **S4 v3 失败态/空态**：精准注入任务列表 500（4 次请求命中）→ 页面显示「加载失败: HTTP 500: Internal Server Error」+ 重试按钮；空注入 → 「暂无待办任务，去流程中心发起」，二者可区分；0 pageerror。
- 截图：process-dom-audit.png / process-injected-detail.png / process-toast-results.png / process-fail-state.png / process-empty-state.png / process-v2-typeerror-toast.png / process-v3-fail-state.png（均 headless 1440x900 截屏，非物理真机）。

## 4. 三分判定

**真缺陷：0 个（用户可达口径）。** 判据：真缺陷须真 UI 可复现 + 真请求/响应 + 源码定位三全。7 个有 UI 入口写按钮在真实后端假 ID 探针下全部 404（前端 catch 正确错误 toast），真成功路径的 `ActionResult::success` 无 bool 失败标志；误报只在 route 注入的人造 200 + type:error / bool:false 下出现，真实端点不产生此形状。rollback 真实 200 + rolled_back:false 可达，但对应 engineWorkAction(rollback) 无 UI 入口，属死代码，不构成用户可达缺陷。

**取证限制**：破坏性真状态翻转（催办落库、complete/reject 真审批、rollback 真回滚）未授权未执行；S2 用 route 注入替代真实失败信封；rollback 仅探针与源码确认，未通过 UI 触发（无入口）。

**产品语义差异**：rollback 无快照返回 rolled_back:false 属设计（未持快照非错误），与 R362/R359 会议 Bool(result>0) 的 0 行 UPDATE 口径相近但本页无入口；loadTwin* / surfaceOps* 等批量端点操作集合为历史调试函数，业务契约未要求渲染入口。

**L1 死代码潜在隐患（非真缺陷，仅记录）**：25+ 无入口写函数（surfaceOps3-20 / engineRest* / collabAction / loadTwinA-H / engineWorkAction 等）await 后无条件 toast.success，若未来接入 UI 入口，遇会议式 200 + type:error / rolled_back:false 信封会复现 R362 同型 P1（误报成功）。建议届时对齐 R363 方案 B：解析响应 `type` 与对应布尔标志，仅 `type === success` 且标志为 true 才弹成功，否则 toast.error。本轮不改码。

## 5. 通过项

- 404 / 500 真错误：前端 catch → toast.error，文案含 HTTP 状态，无误报成功。
- 有入口写函数的 busy 守卫（pressing / v2Busy / submitting / startSubmitting）与 engineWorkAction 的 confirmMsg 取消守卫源码确证。
- 列表失败态「加载失败: HTTP 500...重试」与空态「暂无待办任务」可区分；三轮 0 pageerror。
- 发起流程 submitStart 校验 workId 非空（后端未返回 ID 抛错），R354 H5 已覆盖真写闭环。

## 6. 交付

- 唯一文档：本报告 `oa4rust/docs/ops/oa4rust-r365-pc-processwork-write-audit.md`。
- 证据（gitignored）：`oa4rust/test-results/r365-pc-processwork-write/`，脚本 `oa4rust/test-results/r365-pc-processwork-write.cjs` 及 -v2/-v3 版。
- 截屏为 headless 浏览器实测，非物理真机；域名与主机以占位符表示，无真值入库。