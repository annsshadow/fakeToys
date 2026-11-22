# R361 H5 审批写操作链甄别报告（process.vue）

> 轮次：R361 / 任务 t173 / 角色 qa-e2e（只读审计）
> 测试环境：<YOUR_TEST_DOMAIN>（脱敏占位，真值运行期从 SSH 凭据读取，未入库）
> 取证方式：Chromium headless 390x844、isMobile/hasTouch、DPR2，**非物理真机**（截图为浏览器模拟渲染，不代表真机）
> 范围：H5 审批页 oa4rust/mobile/src/pages/process/process.vue 的 approve/reject（完成/驳回）写操作链及其错误反馈；只读审计，不改源码/契约/后端

## 1. 结论（三分判定）

**0 真缺陷（通过）**。队长候选「complete/reject 200+success+bool:false 误报成功」经实测证伪：真实后端这两个端点在任务不存在时返回 **404**（非 200+bool:false），前端正确报错；404/403/500 均显示正确错误 toast，0 pageerror。注入的 200 失败形状属人造 route 响应，真实契约不产生，不判真缺陷。

- 真缺陷：无。
- 取证限制：真实任务状态翻转属破坏性写，未授权不执行；注入对照仅改 route 响应模拟，非真实后端写。
- 产品语义差异：complete 成功自动认领下一任务或收尾 work 属后端既有设计；假 ID→404（NotFound）与 R359 会议写端点 0 行 UPDATE→200+bool:false 是不同后端实现口径，非前端缺陷。

## 2. 覆盖甄别

docs/ops 全库对 process.vue / 审批写操作链命中 6 处，均非本轮专项：
- oa4rust-r325-h5-process-audit.md：R325 首次深度审计，P3 真驳回真写闭环 200+toast+列表刷新 PASS，但未测 bool:false/type:error 失败信封——本轮增量正在于此。
- oa4rust-r317-h5-deep-audit.md:27、oa4rust-r300-readable-process-titles.md、其余 R325 相关条目：均非写链失败态实测。

本轮首次对审批写操作链的「真实后端失败信封 + UI toast 一致性 + 错误文案可区分」做真浏览器实测，属有效增量。

## 3. 源码分析（坐实候选不成立）

- process.vue(523 行)：act(:219-240) 先校验 taskId、占用 busy 守卫，uni.showModal 收集意见后调 runAction；runAction(:242-253) await completeTask/rejectTask 后无条件 uni.showToast（已通过/已驳回，icon success）+loadFirst()，catch 才错误 toast，finally busy=false。模板审批按钮 :280 驳回 / :283 通过，disabled=busy，@tap.stop=act。
- services/index.ts:75-143 processApi.completeTask(:92-99) POST /api/task/${id}/complete、rejectTask(:101-108) POST /api/task/${id}/reject、saveWorkData(:121-122) PUT data/work/${workId} 三处 discardResponse:true。
- http.ts:67-77 parseEnvelope 仅字符串 JSON.parse 不解包；performRequest:186-192 2xx discardResponse→emptyResponse(data undefined)；:204-208 403→PermissionError（没有权限执行该操作），其它非 2xx→httpStatusError（HTTP N）。
- processList.ts:13-67 PROCESS_PAGE_SIZE=50；loadProcessPage 单层 Array.isArray(resp.data)；mergeProcessRows 按 id 去重；describeProcessError 区分失败文案与空态。

**后端契约关键**（processplatform_service_processing/src/lib.rs）：task_complete(:3795)/task_reject(:3977) 先 SELECT ... FROM x_task WHERE id=$1 FOR UPDATE → .ok_or(NotFound)(:3811/:3993)，假 ID 直接 404；状态非 active/processing→400；权限→非 2xx；成功返回 ActionResult::success(row_to_json(&row)) 完整 task row(:3973/:4056)，**无 bool 字段**。shared/src/response.rs:149-219 ActionResult 设 type=success/message 空/data=Some。因此 R359 的「UPDATE 0 行→200+success+bool:false」路径在本端点不可达——complete/reject 成功响应体本无业务失败标志，discardResponse 丢弃它不改变正确性。

## 4. 真浏览器实测证据

Chromium headless 390x844 isMobile/hasTouch DPR2（非物理真机），证据 oa4rust/test-results/r361-h5-process-write/（gitignored）：evidence.json + process-injected.png / process-ops-done.png / process-fail500.png。共 25 次 API，pageerrors=0。

- S0 真实后端假 ID 探针：POST /api/task/R361-NOSUCH-.../complete=404、/reject=404，body={count:0,message:not found,type:error,prompt:com.x.base.core.project.exception.ExceptionEntityNotExist,size:-1}——坐实假 ID 走 404，非 200+bool:false。
- S1 route 注入列表 1 行 R361 注入待办（id R361-INJECT-1）带驳回/通过按钮，UI 已加载 1 条，截图肉眼确认。
- S2 UI toast 一致性 6 操作（全部 clicked+confirmed）：boolFalse+驳回 POST reject 200→toast[已驳回]；boolFalse+通过 200→[已通过]；typeError+通过 200 type:error→[已通过]（误报但真实后端不产生此形状）；http403+通过 403→[没有权限执行该操作] 正确；http500+通过 500→[HTTP 500] 正确；success+通过 200→[已通过] 正确。
- S3 状态：emptyText=暂无待办，failText=流程加载失败：HTTP 500，二者可区分。

注入的 200+type:error/bool:false 误报成功属人造形状：后端 complete/reject 的 200 成功信封不含 bool 字段、失败一律非 2xx，故不判真缺陷。

## 5. 通过项与限制

- 通过：404/403/500 正确错误 toast、busyId 并发守卫、空态与失败文案可区分、0 pageerror、列表回显正确；R325 真实驳回真写闭环已 PASS。
- 取证限制：真状态翻转属破坏性写未授权未执行；注入对照改 route 响应非真实后端；saveWorkData discardResponse 未专项测（仅源码确证，R354 已覆盖写闭环）。
- 产品语义差异：complete 成功自动认领下一任务或收尾 work 属设计；假 ID→404 与 R359 会议 0 行 UPDATE→200+bool:false 为不同后端实现口径。

## 6. 建议

本轮不改码。若未来后端改为 200+业务失败标志信封，可考虑在 completeTask/rejectTask 去除 discardResponse 并在 runAction 校验 type/bool 后再弹成功 toast——但当前契约不成立，无需改动。
