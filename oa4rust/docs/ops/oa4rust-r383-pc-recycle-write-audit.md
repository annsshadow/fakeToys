# R383 PC RecycleApp.vue 写操作链失败信封甄别（只读审计）

## 1. 范围与环境

- 轮次：R383，任务 t206；性质：只读深度审计，不改源码/契约/配置/后端/SDK。
- 对象：PC 回收站 RecycleApp.vue（路由 /app/recycle，frontend/apps/desktop/src/main.ts:107-110）。
- 环境：测试服 <YOUR_TEST_HOST>，本地 Playwright Chromium headless 1440x900。**截图与 evidence 均为本地无头浏览器实测，非物理真机。**
- 凭据：运行期经 gitignored 部署脚本 + ssh 通道读取，未入库、未回显；域名/主机统一用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位，全文 0 真值。
- 方法：真实 UI 点击 + 真后端请求（决定性对照只注入列表行、写请求完全走真后端不拦截）+ page.on(response/request) 记录信封 + pageerror 全程监听。证据落 gitignored 目录 test-results/r383-pc-recycle-write/。

## 2. 覆盖甄别（静态文档核对，先于实测）

docs/ops 命中 13 处回收站相关文档。oa4rust-r336-settings-recycle-triage.md:15-25 明确：R301/R321/R304 覆盖了列表加载/筛选、批量恢复（仅功能成功路径 6 项→0 项）、FileManager 侧触发恢复、源码级破坏性守卫断言（destructiveGuards.test.ts）；RecycleApp 页内单条恢复、永久删除、批量删除、清空的真 UI 写闭环与失败信封 UI 反馈从未实测。本轮首次专测失败信封，属有效增量。

## 3. 源码甄别（frontend/apps/desktop/src/views/RecycleApp.vue）

真实 @click 写入口（模板非空代码，非死代码）：
- :32 data-testid=recycle-batch-resume → batchResume（:disabled selectedIds.length===0||busy）
- :33 data-testid=recycle-batch-delete → batchDelete
- :34 清空回收站 → emptyRecycle（:disabled !hasItems||busy）
- :64 行内「恢复」 → resume(item)；:65 行内「永久删除」 → permanentDelete(item)
- :31 刷新列表 → loadItems（只读）；:52/:59 全选/行 checkbox 为纯本地选择。

误报函数群（await 后不读响应 type/bool，直接落态）：
- resume(:148-156)：await api.post(/api/recycle/resume/{id}) 后 :151 删本地行 + :152 删选中；catch(:153) 才 toast.error。失败信封仍删行 → 状态矛盾。
- permanentDelete(:158-171)：busy 守卫 :159、confirmMsg 守卫 :162、await api.delete(/api/recycle/{id}/delete) 后 :164 删行 + :165 删选中；失败信封仍删行。
- batchResume(:173-189)：busy+selected 守卫 :174、confirmMsg :175、results=await Promise.allSettled(ids.map(id=>api.post(/api/recycle/resume/{id}))) :179，ok=results.filter(r=>r.status 为 fulfilled).length :180，:181 删全部 ids、:182 清空选中、:183 toast.success「已恢复 N 个项目」。**核心疑点**：SDK 仅非 2xx 抛错，2xx+type:error 原样 resolve → fulfilled → ok++ 把失败误计成功，且无条件删行。
- batchDelete(:191-207)：同构，Promise.allSettled(ids.map(api.delete(/api/recycle/{id}/delete))) :197，ok 按 fulfilled 计 :198，:199 删全部行、:201 toast.success「已永久删除 N 个」。
- emptyRecycle(:209-222)：confirmMsg :213、await api.delete(/api/recycle/empty) :214、:215 清空 allItems + :216 清空选中；无成功 toast，失败信封仍清空。
- loadRecycleTwin(:224-232)：s() 吞错的可用性探测（DELETE /api/recycle/0/delete + POST /api/recycle/0/resume），模板无 @click 绑定，属死代码，按 R364/R365/R372 口径不判缺陷。

对比组：loadItems(:134-146) api.get(/api/recycle/list) → r.data??[]，catch(:142) describeListError「回收站加载失败」；模板 :39/:41 ListLoadError+retry，空态 :42-45「回收站为空，暂无可恢复或清理的项目」，筛选空态 :46-49「没有符合当前筛选条件的回收项目」。

## 4. 后端契约与 SDK 铁证

后端 backend/crates/file_assemble_control/src/lib.rs：
- recycle_id_delete(:4099-4133)：SELECT person WHERE id AND deleted_at NOT NULL；:4112 无行 → Ok(ActionResult::error(file not in recycle bin))；require_owner :4116；DELETE result==0 → :4124-4125 error；成功 :4127 success({id,deleted:true})。假ID → HTTP200+type:error。
- recycle_id_resume(:4137-4168)：同口径，:4150 无行 → error；成功 :4162 success({id,resumed:true})。假ID → HTTP200+type:error。
- recycle_empty(:3968-3986)：恒 Ok(ActionResult::success({value:true}))，即便空回收站（W12 对齐 o2server ActionEmpty）。
- 路由 routes.rs：DELETE /api/recycle/empty :249、GET /api/recycle/list :250、DELETE /api/recycle/{id}/delete :252、POST /api/recycle/{id}/resume :253、POST /api/recycle/resume/{id} :185、DELETE /api/recycle/delete/{id} :192。
- shared/response.rs:188-200：ActionResult::error 设 type=error/message/size=-1/prompt=ExceptionEntityNotExist，但 HTTP 状态仍 200。
SDK frontend/packages/sdk/src/api.ts：:82-88 parseJsonEnvelope 仅 JSON.parse 不检查 type；:275-296 仅 !resp.ok 抛错（401 刷新/403 PermissionError/其它 httpStatusError），2xx+type:error 原样 resolve。

## 5. 真浏览器 E2E（两轮，1440x900 headless，非物理真机，0 pageerror）

轮1（job bash-7223aaccab054a24，evidence.json）：
- DOM 审计 29 按钮，批量恢复/批量永久删除/清空回收站/恢复×2/永久删除×2 全部真渲染，4 个 testid 各命中 1。
- 真后端假ID探针：resumeFake/deleteFake/resumeAltFake 全 status200 + type:error「file not in recycle bin」；emptyReal status200 + type:success{value:true}。
- 决定性对照（只注入列表 2 行、写响应走真后端不拦截）：单条恢复点真 POST resume → 200+type:error，UI 无成功 toast 却把行从 2 删到 1（失败仍删行）；批量恢复勾选 → 真 POST → 200+type:error，UI 弹「✓已恢复 1 个项目」且删行。
- confirm 取消守卫：永久删除弹窗点取消，writesBefore=5/writesAfter=5，零 DELETE 写。

轮2（job bash-f925c8f8e3964be7，evidence-v2.json）：
- 注入 3 行，勾选全选（checked=3）批量恢复 → 3 条真 POST resume 全 200+type:error，UI 弹「✓已恢复 3 个项目」、afterRows=0（截图 recycle-v2-batch-resume-toast.png 肉眼确认）。
- 注入 3 行批量永久删除 → 3 条真 DELETE 全 200+type:error，UI 弹「✓已永久删除 3 个项目」、afterRows=0（截图 recycle-v2-batch-delete-toast.png）。
- 单条永久删除 → 真 DELETE 200+type:error，无成功 toast 但 beforeRows=3→afterRows=2（失败仍删行）。
- **对比证据**：非2xx 500 → toast「✗恢复失败: : HTTP 500: Internal Server Error」、403 → toast「✗恢复失败: : 没有权限执行该操作」，且 afterRows=3（行保留）；与 200+type:error 下 SDK resolve 导致误删行形成鲜明对照，证明失败信封被误当成功。
- 失败态（列表 500）：「回收站加载失败：HTTP 500: Internal Server Error」+重试按钮（截图 recycle-v2-fail-state.png）；空态（空 data）：「回收站为空，暂无可恢复或清理的项目」；二者可区分。
- 两轮 pageErrors 均为空。

## 6. 三分判定

### 6.1 真正缺陷（P1）——1 个

RecycleApp.vue batchResume(:173-189)/batchDelete(:191-207) 的 Promise.allSettled 按 fulfilled 计 ok，后端 HTTP200+type:error（假ID「file not in recycle bin」真后端可复现）仍 fulfilled→ok++ 误报「已恢复/已永久删除 N 个项目」并无条件删本地行；resume(:148-156)/permanentDelete(:158-171)/emptyRecycle(:209-222) 同样 await 后不读响应即删行落态。三全满足：真 UI 可复现（点批量恢复/删除，行注入、写响应走真后端不拦截）+ 真请求响应（真 POST/DELETE status200 + type:error）+ 源码定位（上述行号）。
最小修复方向：对齐 R363 方案 B（frontend/apps/desktop/src/views/MeetingApp.vue:634-640 meetingWriteSucceeded）——新增写响应成功判定 helper（判 type=success 且权威布尔 resumed/deleted 严格为 true，否则 false）；Promise.allSettled 的 ok 只统计 helper 判定为真的结果而非 fulfilled 数量；await 后仅在判定成功时才删本地行与 toast.success，失败则 toast.error 透传 message 并保留行。

### 6.2 取证限制

- 破坏性真状态翻转/真删未授权不执行，采用假ID + 只注入列表行、写响应走真后端不拦截的对照替代。
- emptyRecycle 后端恒返回 200+success{value:true}，无法构造其失败信封验证「失败仍清空」分支，仅按源码归类记录。
- loadRecycleTwin 死代码未 UI 触发；全量写函数按代表性覆盖，未逐一 UI 触发每个分支。

### 6.3 产品语义差异

- 后端 ActionResult::error 用 HTTP200+type:error 是 o2server 兼容既有设计（shared/response.rs:188-200），非本轮改动。
- recycle_empty 恒 200+success{value:true} 是 W12 对齐 o2server ActionEmpty 的幂等设计。
- 非2xx 错误文案「恢复失败: :」含双冒号属既有字符串拼接形态，不影响功能判定。

## 7. 通过项

- confirm 取消守卫零写；非2xx（403/500）正确 toast.error 且保留行；失败态与空态可区分；0 pageerror；只读 loadItems 失败提示正确；emptyReal 真后端幂等成功。

## 8. 证据索引（gitignored）

- test-results/r383-pc-recycle-write/：evidence.json、evidence-v2.json；
- 截图：recycle-dom-audit.png、recycle-probe-done.png、recycle-injected.png、recycle-single-resume-toast.png、recycle-cancel-guard.png、recycle-batch-toast.png、recycle-v2-batch-resume-toast.png、recycle-v2-batch-delete-toast.png、recycle-v2-single-delete-toast.png、recycle-v2-500-toast.png、recycle-v2-403-toast.png、recycle-v2-fail-state.png、recycle-v2-empty-state.png。
- 脚本：test-results/r383-pc-recycle-write.cjs、r383-pc-recycle-write-v2.cjs（及对应 run.sh）。