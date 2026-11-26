# R314 D2 文件上传 415 修复（multipart 提取器对齐）

> 状态：实现与静态验证完成，测试服真浏览器复验见文末证据索引。
> 来源声明：真浏览器证据由本机 Playwright Chromium headless 渲染远程测试服页面产生，属本地无头浏览器实测，**非物理真机**。

## 1. 问题与根因

- 真实缺陷：文件管理页「上传」选择文件后请求稳定返回 HTTP 415，上传功能不可用。
- 前端链路：`FileManager.vue` 的 `uploadFile()` 用 `FormData.append('file', file)` 经 SDK `api.uploadWithProgress()`（XHR `send(formData)`）以 `multipart/form-data` 提交到 `POST /api/file/assemble/control/file/upload`。
- 后端原实现：`file_assemble_control/src/lib.rs` 的 `upload_file()` 签名为 `axum::extract::Json(body): Json<Value>`，仅接受 JSON；与前端 multipart 冲突 → 415。
- 既有契约盲区：`backend-route-methods.fixture.ts` 仅登记该路由 `POST` 方法，不校验 Content-Type，故契约绿但真 UI 失败。本次未改该冻结文件。

## 2. 修复实现

- `upload_file()` 改签名为 `multipart: axum::extract::Multipart`，路由注册不变。
- 复用同 crate 的 `u2_read_multipart_file()` 读取真实文件字段（返回文件名、MIME、字节），取字节长度作为 size。
- 真实字节落盘：`u2_blob_key()` 生成存储键 → `shared::storage::storage_with_pool(pool.0.clone())` 取存储后端 → `u2_persist_verified()` 写入并校验；`x_file.path` 存存储键。
- 写库：`INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) VALUES ($1..$6, NOW(), $7)` 的既有六字段结构保持不变，creator 取会话身份（不可伪造）。响应追加 `uploaded=true` 与 `storageKey`。
- `security_gate_tests_r6.rs`：直调族中移除 `upload_file(Json(...))` 用例（该 handler 已收 Multipart，无法直调），匿名门禁改由路由层 oneshot 的 multipart 请求覆盖，保持「无会话必须 401」判据不放松。

## 3. RED → GREEN（两处独立缺陷）

### 缺陷一：Content-Type 冲突（HTTP 415）

- RED：新建 `FileUpload.R314.contract.test.ts` 初版断言 `mut multipart: axum::extract::Multipart`，原 Json 实现下 1 failed / 1 passed。
- GREEN：后端改为 Multipart 并修正编译后，契约 2/2 passed；`cargo test -p file_assemble_control` 98 passed / 0 failed / 29 ignored；`pnpm run typecheck` 全项目通过；冻结的 `route-methods.test.ts` 2/2 passed（无基线漂移）。
- 编译期连带修正：`pool.0.clone()` 传 `Pool` 而非 `Extension<Pool>`；去掉多余 `mut`；响应 `path.clone()` 避免 moved value。

### 缺陷二：上传写入 folder_id 与列表根目录口径不一致（上传后列表不可见）

- 首次真浏览器复验：上传 POST 已 HTTP 200 且响应含 176 字节真实内容、`uploaded=true`、`storageKey`，但「我的文件」列表不可见、刷新后仍不可见、0 pageerror。
- 根因坐实：原 `upload_file()` 写 `x_file.folder_id = String::new()`（空串），而前端 `loadFiles()` 对根目录用 `folderId || 'root'`，后端 `list_files()` 执行 `WHERE folder_id = $1`，两者口径不一致，上传文件永远不会出现在根目录列表。
- RED：契约加断言写入 `let folder_id = "root".to_string()`，原实现下 1 failed / 1 passed（新断言未满足）。
- GREEN：`upload_file()` 写入 `folder_id = "root"`，与前端根目录列表口径对齐；契约 2/2 passed，`cargo test -p file_assemble_control` 98 passed / 0 failed。
- 说明：`folder_id` 仅影响文件归属目录路径，不改变字节落盘路径与会话身份归属校验。

## 4. 真浏览器复验（结果：通过）

- 环境：测试服 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`（地址与凭据仅运行期从隔离环境读取，不入库、不回显），Playwright Chromium headless，视口 1440x900。
- 流程：真实登录 → `/app/file` → 点击「上传」 → 选择本地真实文本文件（176 字节） → 捕获上传 POST 状态码 → 列表可见性 → 刷新持久性 → 采集 pageerror / 4xx+ 响应。
- 实测结果：`uploadStatus=200`（非 415），响应 `type=success` 且 `uploaded=true`、`storageKey=attachment/<id>/<name>`、`folderId=root`、`size=176`；`fileNameInList=true`、`fileNameAfterReload=true`；`errors=[]`、`apiFailures=[]`；harness 输出 `BROWSER_DONE ok=true`（exit 0）。
- 首轮对照（未修 folder_id 前）：上传同样 200，但 `fileNameInList=false`、`fileNameAfterReload=false`，据此定位缺陷二。
- 证据索引：`oa4rust/test-results/r314-upload-415-fix/`（gitignored）：`evidence.json` + `01-file-before.png`、`02-after-upload.png`、`03-list-visible.png`、`04-after-reload.png`。
- 截图 SHA-256（本地无头浏览器渲染，非物理真机）：`01-file-before.png`=5cb7cd4a6e3d6ec72f14bda9fec57be0c65abee32b08b19fac9fc9efd5680f3d；`02-after-upload.png`=625112440293fb0c6514e1f2ce20a65153d5163d714ad60fd1eb9f9d18dee6e2；`03-list-visible.png`=5aaad759ffb73d81b5345b5f527fb52f38195372ea61858830bb4306b3345a2e；`04-after-reload.png`=9841c12ab967e134dbd2650912405d73cc583c3d49c669b07ed74b3e7d64839b。

## 5. 取证边界

- 全部证据均为本地无头浏览器渲染远程测试服页面，不是物理真机截图。
- 纯静态契约仅锁源码结构，真 2xx 与列表可见以本节真浏览器 evidence 为准。
- 项目 ≥1000 轮持续迭代，本文不代表项目完成。