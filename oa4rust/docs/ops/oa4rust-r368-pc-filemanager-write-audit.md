# R368 PC FileManager.vue 写操作链失败信封甄别（只读审计）

- 任务：t184 / R368，attempt 0da9e84a-da07-457f-bc2c-d2446cc56b51。
- 性质：**只读甄别**，不改源码、契约、后端、SDK。
- 测试环境：Playwright Chromium headless，**1440x900 非物理真机**；测试服域名与凭据运行期注入，文档仅用占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`。
- 证据目录：`oa4rust/test-results/r368-pc-filemanager-write/`（gitignored，不入库），含 evidence.json、evidence-v2.json、evidence-v3.json、evidence-v4.json 与截图 file-dom-audit.png、file-create-toasts.png、file-v2-fail-state.png、file-v4-guard-cancel.png、file-v4-delete-toast.png 等。

## 一、判定：1 个 P1 真缺陷

**缺陷**：文件管理页两处有真实 @click 入口的写函数在后端返回 HTTP 200 + 业务失败信封（type:error）时，无条件弹出成功 toast，用户误以为操作已生效。

- `createRecyclableFile()`（`oa4rust/frontend/apps/desktop/src/views/FileManager.vue:210-226`）：:214 `await api.post(/api/file/core/entity/file, …)` 后不读响应 `type`，:219-220 取 `resp.data.id`，:221 用该 id（失败时为 **空串**）在 UI 造出一条记录，:222 无条件 `toast.success(文件已创建，可删除进入回收站)`。
- `deleteFile()`（同文件 `:896-909`）：:898 `confirmMsg` 守卫通过的删除走 `deleteMutation.mutate`，其 `onSuccess`（:901-905）不读 api 响应 `type`，:904 无条件 `toast.success(文件已删除)`；:906 `onError` 只覆盖 HTTP 非 2xx。
- 三全满足：①真 UI 可复现（真实点击「删除」→ confirmMsg 确认）；②真请求响应（DELETE 假 ID → 200 + type:error file not found）；③源码定位（await 后裸 toast.success）。

**最小修复方向（本轮不改码）**：对齐 R363 方案 B——新增写响应成功判定，判 `type === success` 且权威 bool 严格为 true，否则 `toast.error(响应 message 或 操作未生效)`；`createRecyclableFile` 还需在 type 非 success 或 data.id 为空时不得构造空 id 记录；`deleteFile` 的 deleteMutation mutationFn/onSuccess 须接住响应并判 type。参考实现：`oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:634-640` 的 `meetingWriteSucceeded`。

## 二、源码与契约证据

- 后端 `file_create`（`oa4rust/backend/crates/file_core_entity/src/lib.rs:412-425`）：name/person/reference_type 任一为空即 `return Ok(Json(ActionResult::error(name, person and reference_type are required)))`——**HTTP 200 + type:error**。tests.rs:232 明文测试 `test_file_create_missing_fields_returns_200_with_error_body`。
- 后端 `u2_file_delete_by_id`（`oa4rust/backend/crates/file_assemble_control/src/lib.rs:5016-5030`）：`SELECT person FROM FILE_FILE WHERE id=$1` 未命中即 `return Ok(Json(ActionResult::error(file not found)))`——同样 HTTP 200 + type:error。
- `ActionResult::error`（`oa4rust/backend/crates/shared/src/response.rs:188-200`）：设 `type=error`、`message`、`size=-1`、`prompt=ExceptionEntityNotExist`，但 HTTP 状态仍 200。
- SDK（`oa4rust/frontend/packages/sdk/src/api.ts:82-88` `parseJsonEnvelope` 仅解析 JSON 不检查 type；:275-296 仅 `!resp.ok` 抛错，2xx 的 type:error 原样 resolve）。因此前端 await 不抛错，裸 toast.success 误报。

## 三、静态侦察与 fail-loud 更正

- 队长静态备料列三处真入口；实测**更正**：`uploadWithReference()`（`:1022-1047`）全文件仅在定义处出现，模板无 @click、无 defineExpose/window 挂载，属**无 UI 入口的死代码**，不判用户可达缺陷，not pass 静默。
- 真入口仅 `:17` `data-testid=file-create-json` → createRecyclableFile、`:64`/`:85` 删除按钮 → deleteFile；DOM 审计 36 个可交互元素含真「新建可清理文件」按钮。
- 覆盖甄别：docs/ops 既有 r301/r321/r336/r358 等覆盖 FileManager 的上传 415、回收站刷新、创建修复与 H5 doc 双来源，均未覆盖写链失败信封；本轮首次专测。

## 四、E2E 实测（1440x900 headless，非物理真机）

1. 首轮（job bash-437771cd65b44f72，evidence.json）：
   - 真后端假 ID 探针：`POST /file/core/entity/file` 缺字段 → 200 + type:error「name, person and reference_type are required」；`DELETE /file/assemble/control/file/{假ID}` → 200 + type:error「file not found」；冒名 person → 403。
   - 注入对照：create 在 `type:error` 下仍弹「✓文件已创建，可删除进入回收站」且 UI 造出空 id 记录；`data:{}` 无 id 同样误报；非 2xx（500）→「✗新建文件失败: HTTP 500: Internal Server Error」正确。
   - 删除 confirm 取消（首轮只做 DELETE 计数）0 写。
2. v2（job bash-7e55a11536914871，evidence-v2.json）：失败态 `.list-error-empty` →「⚠️文件列表加载失败：HTTP 500: Internal Server Error」+重试；空态 `.empty-state.modern-empty` →「📁当前文件夹为空📤 上传文件，开始协作」；二者可区分。真后端 create 探测 200 + type:success + data.id（此为真系统测试数据，非物理真机，已如实记录）。
3. v3（job bash-0a594ad3574c4de7）：证实 confirmMsg 是自定义 DOM 弹窗（`#oa4-confirm-overlay`，非原生 dialog），原首轮 dialog 监听不到的原因坐实，据此改 DOM 点击。
4. v4（job bash-c245333d01894410，evidence-v4.json，**决定性对照**）：
   - 取消守卫：真实列表注入 1 行，点「删除」→ confirmMsg 弹窗文案「确定删除「R368v4注入文件」？」→ 点「取消」→ deletesBefore=0 / deletesAfter=0，零 DELETE 写。
   - 真后端误报：同一行点「删除」→ 点「确认」→ 真 `DELETE …/{fakeID}` → **200 + type:error file not found** → UI 仍弹「✓文件已删除」（截图 file-v4-delete-toast.png 肉眼确认）。
   - 全程 0 pageerror。

## 五、三分判定

- **真缺陷（1 个 P1）**：上述 createRecyclableFile / deleteFile 在 200+type:error 下误报成功，三全满足，最小修复方向见第一节。
- **取证限制**：破坏性真状态翻转/真删未授权未执行（用假 ID + 列表注入 + 真 DELETE 假 ID 对照）；uploadWithReference 无入口属死代码，仅源码甄别未 UI 触发；create 真后端 200+error 由 HTTP 探针直接坐实而非 UI 点击造失败（UI 层用注入对照），delete 真后端 200+error 由真实 UI 点击坐实。
- **产品语义差异**：后端 ActionResult::error 用 HTTP 200 + type:error 是 o2server 兼容既有设计（非本轮改）；confirmMsg 为自定义 DOM 弹窗而非原生 confirm 属既有设计；可清理文件列表为前端本地 ref，create 写入后端仅用于演示删除流程属既有设计。
- **通过项**：HTTP 403/500 正确 toast.error；confirm 取消零写；失败态/空态文案可区分；0 pageerror；无 UI 入口函数不判真缺陷。