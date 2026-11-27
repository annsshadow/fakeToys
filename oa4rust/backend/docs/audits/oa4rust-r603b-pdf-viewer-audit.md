# R603b 验证型甄别：PDF 查看器在线预览恒失败（P2 确凿）

- 标识: oa4rust-r603b-pdf-viewer-audit / 轮次 R603b / 任务 t170
- 类型: 验证型甄别（零源码改动，唯一脱敏审计文档）
- 父提交: 160be792361850f529a8c1fba09664ca8cc1d5bd
- 结论: **P2 确凿** —— PDF 查看器在线预览功能 100% 不可用：探测流必被 SDK 判为 malformed JSON 而抛错，try/catch 两分支赋同一坏 URL，iframe 指向的端点是 JSON 元数据而非 PDF 字节，UI 恒显示"在线预览中"假成功。非越权、非安全问题、非 IDOR 族。

## 一、立项与范围

台账 docs/ops/current-state.md 未见对 PDF 查看器数据源的专项甄别。本次只甄别、不改任何业务源码；唯一入库产出为本脱敏文档。区别于 R419（DEV 门控）、R370（写 envelope 假成功）：本缺陷是数据源端点选错 + 探测死分支，属功能可用性问题。

## 二、机制锚点（逐行核实）

- 前端 oa4rust/frontend/apps/desktop/src/views/PdfViewerApp.vue:42-55 loadPdf()：:48 try 内 await api.get(`/api/attachment/download/${fid}/stream`) 探测二进制流，:49 赋 pdfUrl=`/api/file/download/${fid}`；:51 catch 内赋**完全相同**的 pdfUrl；:52-53 finally 恒置 loading=false。探测结果被丢弃 = 死分支。
- 同文件模板 :12 `<div v-if="pdfUrl">` + :13 `<iframe :src="pdfUrl">` + :14 "文件ID: {{ fileId }} · 在线预览中" —— pdfUrl 非空即渲染 iframe 并恒显假成功。
- SDK 链 oa4rust/frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope() 对 resp.json() 失败抛 ApiError('malformed JSON response', resp.status)；request() :296 与 get() :299-301 均经此路径。既有测试 api.malformed-json.test.ts:34-43 锁定 get() 对 2xx 非法 JSON 归一化 ApiError（STATUS 保留）。
- 后端宣称数据源 crates/file/src/lib.rs:508-572 file_download() 返回 `Result<Json<ActionResult<Value>>, AppError>`，:568 `Ok(Json(ActionResult::success(result)))`，取 FILE_FILE 元数据 JSON（id/name/person/referenceType/extension/length/mimeType/createTime），非 PDF 字节；路由 file/src/routes.rs:20 /api/file/download/{id} → file_download。iframe 无法渲染该 JSON。
- 正确数据源 crates/file_assemble_control/src/lib.rs:1339-1388 attachment_id_download_stream → attachment_id_download(:1339-1378) :1366-1374 `Response::builder().header("Content-Type", sanitize_mime(&mime)).body(Body::from(bytes))` 返纯二进制，且 :1354 require_file_read_owner 归属门禁；路由 file_assemble_control/src/routes.rs:127 /api/attachment/download/{attid}/stream → attachment_id_download_stream。前端探测的正是这个 stream 端点，却把结果丢弃后改指向元数据端点。
- 真实可达：apps/desktop/src/main.ts:452-455 路由 'pdf-viewer' → PdfViewerApp.vue，:503 redirect '/app/admin/pdf-viewer'；components/admin/adminNav.ts:110 {path:'/app/admin/pdf-viewer',label:'PDF查看器',icon:'📑'}；adminNav.test.ts:95 覆盖。缺陷在真实管理导航路径上可达。

## 三、方法（真测试，非静态推断）

- 夹具: oa4rust/tests/r603b_probe.test.ts（临时，运行后已删除，gitignored 思路）。
- 日志: /tmp/r603b-probe-final.log，文件 sha256 `38928110cffffca2f0092dc9645b198bd74fd39f8abcf6829f0663217b3ad23e`。
- 命令: `cd oa4rust && npx vitest run tests/r603b_probe.test.ts`，EXIT 0，4 passed / 0 failed。
- L1 真 ApiClient.get 对 PDF 二进制 Response（2xx application/pdf）→ 断言抛 ApiError('malformed JSON response', status=200)，坐实 stream 探测必失败。
- L2 从 Vue 源码正则抽 loadPdf 函数体，new Function 真执行：try 成功与 catch 抛错两路径 pdfUrl 均 = `/api/file/download/att-1` 且恒等、loading 均归 false，坐实死分支。
- L3 源码守卫真测：file/src/lib.rs 含 `Result<Json<ActionResult<Value>>, AppError>` 与 `Ok(Json(ActionResult::success(result)))`；file_assemble_control/src/lib.rs 含 `Body::from(bytes)` 与 Content-Type sanitize_mime，坐实两数据源差异。
- L4 UI 假成功真测：模板含 `v-if="pdfUrl"`、在线预览中、`:src="pdfUrl"`；真执行 ok/throw 两路径 pdfUrl 均非空且恒等 → v-if 恒真、iframe src 恒为坏 URL、假成功文案恒显。

## 四、结论与建议（不在本任务写域，仅记录）

- 影响：PDF 在线预览 100% 不可用——探测流必抛错被静默吞，iframe 加载的是元数据 JSON 而非 PDF 字节，用户永远看到"在线预览中"却无内容；因错误被 catch 吞没，无任何用户可见错误提示。
- 修复方向（供后续实施）：loadPdf 应直接把 stream 端点 URL（/api/attachment/download/{fid}/stream）赋给 iframe/pdf 查看器，删除无效 JSON 探测；或对二进制探测改用 fetch + arrayBuffer 判定 Content-Type，而非 api.get 的 parseJsonEnvelope；UI 应依据探测结果区分加载中/成功/失败，避免恒显假成功。
- 三分裁决：功能 100% 不可用但无越权、无敏感数据泄露、非安全绕过 → 不判 P0/P1，维持 P2 确凿。

## 五、证据 sha 表

- runnerSha（紧凑 JSON，`sha256(JSON.stringify(meta))`）：`e72b9c2e4bc255bd466819a25ba2d0a3728481f88d1d736d2faa2faee397cbca`
- 探针日志 sha256：`38928110cffffca2f0092dc9645b198bd74fd39f8abcf6829f0663217b3ad23e`（gitignored，非入库交付物）。
- 文档不含自身 sha（避免自引用悖论）；脱敏：无数据库/服务器地址、无口令、无个人路径，端口占位 `<TEST_HOST>:<PORT>`（本任务未连库，仅占位声明），0 真值入库。
- SKIP/未覆盖：未跑浏览器 GUI 真页面（本环境无可用 GUI 会话；以真 ApiClient.get + Vue 函数体真执行 + 源码守卫三层真测试覆盖核心判定，据实标注）。
