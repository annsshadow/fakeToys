# R427 PC 查找设计器（FindDesignerApp.vue）保存写链误报甄别审计

> 轮次 R427 / 任务 t267 / 只读甄别（kind=work，不改码）
> 环境：1440x900 headless Chromium 真浏览器，非物理真机；证据目录 gitignored（test-results/r427-pc-finddesigner-write/）。
> 真值脱敏：文中域名/主机一律用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位；API 仅记相对路径。

## 1. 结论摘要

判定 **1 个用户可达 P1 真缺陷**（真 UI 可复现 + 真请求响应体 + 源码定位，三全满足）。
保存函数在后端返回 HTTP 200 + `type:error`（`query design not found`）时，不读响应信封，仍弹绿色「保存成功」，用户误以为配置已落库，实际 0 行 UPDATE 静默失败。
两条真实可达路径均坐实：新建配置（createNew 本地假 id）、点击真实列表行。
只读甄别，未改任何代码。

## 2. 甄别范围与源码定位

- 视图：`frontend/apps/desktop/src/views/FindDesignerApp.vue`（约 170 行）。
- 模板入口：第 9 行 `<button class="btn-primary" @click="save">💾 保存</button>`（真实可达）；第 22 行 `.sb-add @click=createNew`；第 19 行 `.sb-item @click=selectItem`；第 28 行 `textarea.code-editor v-model=config`。
- `save()`（第 92–108 行）：第 100–106 行 `api.put("/api/query/assemble/designer/save/"+encodeURIComponent(item.id), {...item, query:config.value}).then(()=>toast.info("保存成功"))` —— 第 106 行 `.then` 完全不读响应 type；第 107 行 `.catch` 仅覆盖非 2xx。
- `createNew()`（第 84–88 行）：`item.id = Date.now().toString()` 本地假 id 入列表并选中；`selectItem()`（第 80–83 行）。
- `preview()`（第 89–91 行）：仅 `toast.info("配置预览…")` 展示，剔除。
- 列表：`GET /api/query/assemble/designer/list/all`（第 54–66 行），`useListError` label=`查找设计配置加载失败`（第 74–79 行）。
- 门控核查：`EndpointFold` / `devToolsEnabled` / `import.meta.env.DEV` / `defineExpose` 全为空（grep 0 命中），DOM `fold=0`，无生产隐藏门控。
- 路由：`frontend/apps/desktop/src/main.ts` 第 434–436 行 path=`find-designer`，第 497 行重定向；真实访问 `/app/admin/find-designer`。
- 剔除项：`preview()`、各 `load*`、列表展示类 toast，均非写链。

## 3. 后端失败信封铁证

- `backend/crates/query_assemble_designer/src/lib.rs` 的 `save_designer()`（第 607–645 行）：第 614 行 `require_admin_opt`（无会话 401 / 非 admin 403，函数定义第 59–70 行）；第 621–628 行 `UPDATE x_query_design SET ... WHERE id=$4 AND deleted_at IS NULL`；第 630–631 行 `result == 0` 时返回 `Ok(Json(ActionResult::error("query design not found")))`，即 **HTTP 200 + `type:error`**；第 634 行仅成功时返回 `success({id,saved,name,query})`。
- 路由：同文件第 889 行 `.route("/api/query/assemble/designer/save/{id}", put(save_designer))`；第 903 行 `list/all`。
- `ActionResult::error` 走 HTTP 200 是 o2server 兼容语义，前端必须显式检查 `type`。

## 4. SDK 响应处理判定

- `frontend/packages/sdk/src/api.ts` 的 `request()`（约第 275 行）：`if(!resp.ok)` 才抛错（401 refresh / 403 PermissionError / 其余 httpStatusError）；2xx 直接 `parseJsonEnvelope` 原样返回，不检查 `type` / `success`。
- `put()` = `request("PUT")`。故后端 200 + `type:error` 不抛异常，直接进入 `.then`，误报成立。

## 5. 真浏览器实测（决定性对照）

### 5.1 真后端假数据探针（脚本 gitignored，证据 probe.json）

- `saveFakeId_authed` = HTTP 200 / `type:error` / `query design not found`（0 行 UPDATE，无副作用）。
- `listAll_authed` = HTTP 200 / 真列表返回。
- `saveFakeId_noSession` = HTTP 401 / `error` / `unauthorized`。

### 5.2 决定性对照（写响应全程不拦截、走真后端，`page.on(response)` 抓响应体）

- S1 新建配置：真点「新建」→ textarea 填 JSON → 真点「保存」→ 真 `PUT /api/query/assemble/designer/save/{假id}` = HTTP 200，响应体铁证 `{"type":"error","message":"query design not found",...,"prompt":"com.x.base.core.project.exception.ExceptionEntityNotExist"}`，`writesDelta=1`，仍弹 `status|ℹ保存成功` —— 误报坐实。
- S2 真实列表行：注入列表行 `R427测试查询`（id=`R427-FAKE-ROW-*`）→ 真点行改 config → 真点保存 → 同一真 PUT = 200 `type:error`，仍弹 `ℹ保存成功` —— 第二条路径坐实。
- S3 非 2xx：注入 500 → 红 `✗保存失败: HTTP 500: Internal Server Error`；注入 403 → 红 `✗保存失败: 没有权限执行该操作`，均正确。
- S5 预览：真点「预览」→ `ℹ配置预览: {}`，`writesDelta=0`，只读剔除确认。
- `totalWrites=4`，`pageErrors=0`。

### 5.3 截图 sha256（命令行原始值；1440x900 headless 非物理真机）

- `r427-decisive-create.png` = `a55a0272454616e07f8ca9a5018754922803723d7232bbfb8605139f863bd691`（已目视：绿 toast ℹ保存成功 + 选中未命名 + 输入框 JSON）。
- `r427-decisive-row.png` = `0f6d0e30941945cf5f5e4bfe1c469a87f0b7af07f59c25e5b0160391d9fc9521`（已目视：绿 toast + 选中 R427测试查询）。
- `r427-500.png` = `82d3f78a9a9ff35ba085bf95694dcc4a9d856dd1800e9722abdb91ae053e34a0`。
- `r427-403.png` = `cb312762f6ee1d22f96316c1f9d7fb8d39f8206dff37bc69a24cd62e451e20a6`。
- `r427-dom-audit.png` = `fae59b310dc056febec395863bf51583bda18b013cea20b8c16e141b12875070`。
- 失败态/空态 `r427-error-state.png` / `r427-empty-state.png` 同 sha `a7f81617b60a8cadd9491bfec1eded6b39fc455898f82dfdc2f27270fb19283c`（注入未生效，见取证限制）。
- 说明：`read_image` 返回的 attachmentId sha 为重编码值；文档记录的是命令行 `sha256sum` 原始值。

## 6. 三分判定与取证限制

- **P1（1 个）**：`save()` 第 106 行在真后端 200 + `type:error` 下走 `.then` 弹 `ℹ保存成功`，且不刷新列表；两条路径均决定性对照坐实。
- 剔除/只读：`preview()`（第 89–91 行展示）`writesDelta=0`；各 `load*` 与列表展示 toast。
- 无额外 L1 隐患需单列（save 即 P1）。
- 产品语义差异：`ActionResult::error` 走 HTTP 200 为 o2server 兼容，前端须显式判 `type`。
- 取证限制：破坏性真写未执行，用 createNew 本地假 id + 0 行 UPDATE 替代；失败态/空态注入未生效（谓词路由+URL 嗅探 listUrls=3 仍 `hasFail=false`/`retryBtns=0`/`items=0`），该页失败/空态「可区分」未坐实，据实记录，P1 判定不依赖此项；全量写按钮按代表性覆盖。

## 7. 最小修复建议与交付

- 建议新增 `findDesignerWriteSucceeded(type 为主、不强求 bool)`，与既有写链守卫模式一致。
- `save()` 的 `.then` 接响应：失败时 `toast.error` 透传 `message` 并 `return`，不弹成功；仅 `type:success` 才 `toast.info("保存成功")` 并刷新列表。
- 交付：本审计文档（唯一入库文件，脱敏 0 真值）；证据与脚本在 gitignored `test-results/r427-pc-finddesigner-write/`，不入库。
