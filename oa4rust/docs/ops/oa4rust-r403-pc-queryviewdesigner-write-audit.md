# R403 PC 视图设计器写链甄别报告（200+type:error 误报成功）

- 轮次：R403 / 任务 t234；角色：qa-e2e（缺陷审计与浏览器实测）
- 性质：只读甄别，未修改任何业务代码
- 结论：**1 个 P1 真用户可达缺陷**（三全满足：真 UI 可复现 + 真请求响应 + 源码定位）
- 实测环境：Playwright Chromium headless，视口 1440x900，**非物理真机**（对已部署测试服的真实页面实测）
- 证据目录（相对路径，gitignored）：test-results/r403-pc-queryviewdesigner-write/
- 基线提交：8cf29ae4d7484a5751e8ad20a91d998faddd27ef（分支 opt1000/oa4rust-pc-ai-refactor）

## 一、范围与判定口径

目标文件：frontend/apps/desktop/src/views/QueryViewDesignerApp.vue（166 行）。
甄别其视图设计器写链在后端返回 HTTP 200 + type:error 失败信封时，是否无条件向用户弹出成功提示。
判定口径承接同批次 R390/R391/R398/R402：真缺陷需同时满足「真实 UI 可达入口」「真实后端失败信封」「源码可定位的裸成功提示」，并以「决定性对照（写响应不拦截、完全走真后端）」排除注入伪造质疑。破坏性真写未授权，一律以假 ID + 不拦截对照替代。

## 二、源码甄别（已坐实）

- 路由：frontend/apps/desktop/src/main.ts:272 path=query-view-designer，:273 name=QueryViewDesignerApp，:275 meta.title=视图设计器，:528 重定向 /app/admin/query-view-designer。
- 无 EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 命中（grep 全空），生产构建真实渲染；DOM 审计 fold=0，页面渲染 11 个按钮，新建视图/保存/模拟/Bundle/加载各 1 个，空态文案「该 Query 下暂无视图」可见。
- saveView（:105-126，模板 :9 真实 @click 保存按钮，:disabled=!canSave||saving）：
  - :106 if(!canSave)return；canSave=computed（:66）= name.trim && queryFlag.trim && sql.trim。
  - :108-112 组装 payload {name, queryFlag, data:serializeViewDefinition}。
  - :114-116 response = activeId ? await api.put(designerPaths.viewSave(activeId), payload) : await api.post(designerPaths.viewCreate, payload)。
  - :117 activeId.value = response.data?.id ?? activeId.value；:118 form.viewFlag = response.data?.viewFlag ?? form.viewFlag。
  - **:119 无条件 toast.success('视图已保存')**；:120 await loadViews()；:121-122 catch 仅处理非 2xx 抛错（toast.error 保存失败）；:123-125 finally saving=false。
  - 读取了 response.data 的 id/viewFlag，但**未判断 type**——这是缺陷根因。
- simulate（:127-140，模板 :9 模拟按钮）：:130 先 await saveView()（复用其成功 toast），:131-134 api.put(viewSimulate(activeId))，:135-136 仅设 result/resultTitle=模拟结果，无独立成功 toast，:137-138 catch toast.error。
- bundle（:141-149，模板 :9 Bundle 按钮）：:144-147 api.put(viewBundle(activeId))，:148-149 仅设 result/resultTitle=Bundle 结果，无成功 toast，:150-151 catch toast.error。
- 只读函数剔除：loadViews（:79-87，api.get viewList）、openView（:88-104，api.get viewGet，detailSeq 竞态保护）、newView（:67）、addFilter（:73）、addSort（:76）——均非写。无 confirmMsg、无原生 prompt、无删除操作。
- SDK：frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 不检查 type；:275-296 仅非 2xx 抛错，2xx 的 type:error 被原样 resolve 成 fulfilled。

## 三、后端信封铁证（已坐实，非臆造）

handler 均在 backend/crates/query_assemble_designer/src/u2_closures.rs：
- view_create（:1277-1335）：:1289-1295 空 name 返回 Ok(Json(ActionResult::error('name is required'))) HTTP 200；:1297-1308 重名 200 error；:1310-1334 成功 success({id, viewFlag})。
- view_edit（:1338-1379）：:1358-1364 UPDATE x_query_view；:1366-1368 更新 0 行 → Ok(Json(ActionResult::error('view not found'))) HTTP 200；:1370-1378 成功 success({id, updated:count})。
- view_bundle_put（:2343-2383）：:2351-2361 属主校验失败 200 error('view not found')；:2365-2375 UPDATE 0 行 200 error；:2377-2382 成功 success({id, bundleSaved:true})。
- view_simulate_put（:2387-2436）：:2395-2405 属主校验失败 200 error；:2407-2418 row None 200 error；:2435 成功 success(payload)。
路由 backend/crates/query_assemble_designer/src/lib.rs:937 POST view、:938 PUT view/edit/{id}、:990 PUT view/{id}（saveView 编辑分支命中）、:991 PUT view/{id}/bundle、:993 PUT view/{id}/simulate。
ActionResult::error 返回 HTTP 200 是 o2server 兼容的既有设计（共享 response.rs:188-200），并非本轮新增缺陷。

## 四、真浏览器实测（决定性对照，1440x900 headless 非物理真机）

脚本（gitignored）：test-results/r403-pc-queryviewdesigner-write.cjs + 同目录 run.sh。登录走测试服真实 UI，写响应全程不拦截。

S1 真后端假 ID 探针（fetch 直打，不触发 UI 破坏性写）：
- viewCreateEmptyName（POST view，空 name）= 200 type:error「name is required」。
- viewSaveFake（PUT view/R403-FAKE-VIEW）= 200 type:error「view not found」。
- viewBundleFake（PUT .../bundle）= 200 type:error「view not found」。
- viewSimulateFake（PUT .../simulate）= 200 type:error「view not found」。
- viewListProbe（GET list/query/...）= 200 type:success data:[]。
坐实后端确为 200+type:error 口径。

S2 决定性对照（仅注入列表行与详情读路由，写路由 PUT 完全走真后端不拦截）：
- 注入列表 1 行 R403测试视图（id=R403-FAKE-VIEW）+ 详情 GET，点加载→点视图行激活（openState：saveDisabled=false，name=R403测试视图，sql=SELECT 1）。
- 真 UI 点保存：writesDelta=1，捕获真响应 PUT /api/query/assemble/designer/view/R403-FAKE-VIEW = **200 type:error「view not found」**，但页面 toast=['✓视图已保存'] → 误报成功坐实。截图 vd-decisive-save.png（sha256 adaecef6191d4346afd7240c139fb0ffad325174fd5a616d0f5a879fdc48af73）目视确认绿色「视图已保存」与后端 200 error 并存。
- 真 UI 点 Bundle：真 PUT .../bundle = 200 type:error，toast 仍=['✓视图已保存']（来自上一次 saveView 残留，bundle 自身无成功 toast），result 为空。
- 真 UI 点模拟：writesDelta=2，真 PUT view/{id}=200 error + PUT .../simulate=200 error，toast=['✓视图已保存']（simulate :130 复用 saveView 的成功提示）。
- totalWrites=11，0 pageerror。

S3 人造 200+type:error stub（仅佐证前端不读 type，本后端本就产生此形状）：仍弹「✓视图已保存」。
S4 非 2xx：写注入 500 → toast '✗保存失败: HTTP 500: Internal Server Error'；403 → toast '✗保存失败: 没有权限执行该操作'，均正确报错（catch 生效）。
S5 列表态：500 注入 → 「✗加载视图失败: HTTP 500: Internal Server Error」+ 空态；空 data → 「该 Query 下暂无视图」，可区分。

## 五、三分判定

- 1 个 P1 真缺陷：saveView（:119）在后端 200+type:error 下无条件 toast.success('视图已保存')；simulate（:130 复用 saveView）连带误报。三全满足（真 UI 保存按钮 + 真 PUT 200 error 响应体 + :114-119 源码定位）。
- 通过项：500/403 正确 toast.error；只读 loadViews/openView 无误报；bundle/simulate 的 result 仅展示返回数据、无独立成功 toast，不单独判误报成功缺陷；列表失败/空态可区分；0 pageerror；无 EndpointFold 门控、真实可达。
- 不判缺陷项：viewCreate 空 name 200 error（前端新建走 canSave 校验，空 name 无法触发），人造 stub 仅佐证。
- 取证限制：破坏性真写（真实新建/更新/撤销视图定义）未执行，用假 ID + 不拦截对照替代；bundle/simulate 的成功提示来自 saveView 残留，其自身 result 展示错误数据属体验隐患非本任务「误报成功」范围；S5 失败/空态基于读路由注入。
- 产品语义差异：ActionResult::error HTTP 200 为 o2server 兼容既有设计；本页 saveView 读取 response.data.id/viewFlag 说明作者预期读取响应体，但未判 type 是缺陷点。

## 六、最小修复建议（本轮不改码，供后续轮次参考）

对齐同批次 R363/R402 方案 B：新增 queryViewDesignerWriteSucceeded(resp:any):boolean，以 type 为主（type==='error' 或 response 失败即判失败）；saveView 在 :114-116 await 后先判成功：成功才写 activeId/viewFlag、toast.success('视图已保存')、loadViews；失败则 toast.error 透传 message，不弹成功、不关状态、不刷列表。simulate 因复用 saveView 自动受益；bundle/simulate 的 result 建议在失败信封下不覆盖为「模拟结果/Bundle 结果」或显式标注失败（可选，非必改）。

## 七、交付与证据

- 本报告：docs/ops/oa4rust-r403-pc-queryviewdesigner-write-audit.md（脱敏，0 真值；测试域名/主机用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位；仅相对路径；明标非物理真机）。
- 原始证据：test-results/r403-pc-queryviewdesigner-write/（gitignored，含 evidence.json 与 vd-*.png 截图）。决定性截图 vd-decisive-save.png sha256=adaecef6191d4346afd7240c139fb0ffad325174fd5a616d0f5a879fdc48af73；探针截图 vd-probe-done.png sha256=2cab54f9b112dbff94da2f64751cf0faa1d2091a18254970eb032b87c7fba63f。
- 判定：1 个 P1 真缺陷；不改码；精确单文档提交。
