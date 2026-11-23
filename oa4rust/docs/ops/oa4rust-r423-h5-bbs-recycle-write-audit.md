# R423 H5 端 bbs 发帖与回收站彻底删除写链甄别（200 + type:error 误报成功）

> 轮次 R423 / 任务 t262 / 身份 qa-e2e（缺陷审计与浏览器实测工程师）
> 甄别性质：只读审计，不改码；三分判定 = 真缺陷 P1 / 同型隐患 L1 / 正确模式
> 测试环境：390x844 headless Chromium，isMobile + hasTouch + DPR2，**非物理真机**（H5 历史 E2E 统一视口）

## 一、范围与结论摘要

甄别 H5 两条写链：① 论坛发帖 `mobile/src/pages/bbs/new.vue` 的 `submit()`；② 回收站彻底删除 `mobile/src/pages/recycle/recycle.vue` 的 `remove()`。
结论：**1 个用户可达 P1 真缺陷**（回收站彻底删除），**bbs 发帖 0 缺陷**（误报前提不成立，列 L1 隐患）。
P1 机制：`recycleApi.remove` 走 SDK `discardResponse:true`，2xx 直接吞掉响应信封不解析 `type`，后端 200 + `type:error` 被当成功 resolve，无条件弹「已删除」。

## 二、源码甄别（逐函数：可达 / 只读 / 剔除）

### 文件 A：mobile/src/pages/bbs/new.vue

- `submit()` :39-67 —— :44-49 `bbsApi.topicCreate({forumId,title,content,creator})`；:50 `newId=resp.data?.id??""`；:51 `uni.showToast("发布成功",{icon:"success"})` 不读 `type`；:54-61 延时 600ms 后按 newId 决定 `redirectTo`/`navigateBack`；:62 `catch` 仅覆盖非 2xx。
- `canSubmit()` :19-20 —— 需 `forumIndex>=0` 且 title、content、creator 非空且非 submitting。
- 模板入口：:83 picker（版块）、:91 input（标题）、:95 textarea（内容）、:98 `.submit @tap="submit"`，真 UI 可达。
- 只读剔除：`loadForums()` :22-33（forumList 失败提示「论坛列表加载失败」）、subjectView / replyList 均只读展示。

### 文件 B：mobile/src/pages/recycle/recycle.vue

- `remove()` :51-74 —— :52 id 校验；:56-63 `uni.showModal` 确认「此操作不可恢复」；:64 `!res` 直接 return；:65 `await recycleApi.remove(id)`；:66 `uni.showToast("已删除",{icon:"success"})` 不读 `type`；:68 `load()` 过滤已删行；:69 `catch` 透传 `e.message` 或「删除失败」。
- 模板入口：:160 `<button class="btn danger" @tap="remove(r)">彻底删除`（uni-app 编译为 `uni-button.btn.danger`），真 UI 可达；:171 清空回收站 `@tap="emptyAll"`（:104-126 另一写链）。
- 只读剔除：`showDetail()` :79-94 详情展示。
- 门控检查：两页均无 `EndpointFold` / `devToolsEnabled` / `import.meta.env.DEV` / `defineExpose`（grep 全 0），仅 `listView v-if` 列表显隐，写按钮生产环境真实可达。

### SDK 关键变体（P1 根因）

- `mobile/src/services/index.ts` :511 `recycleApi`；:514 `remove=(id)=>mapi.delete("/api/recycle/delete/"+id,{discardResponse:true})`；:516 `empty` 同为 discardResponse；:531 `topicCreate=mapi.post("/api/bbs/assemble/control/topic/create",data)`（无 discardResponse）。
- `mobile/src/services/http.ts` :187 `if(options.discardResponse||status===204){resolve({success:true,data:undefined});return}` —— 2xx 直接吞信封，不解析 `type/success`。故回收站在后端 200 + `type:error` 时被吞成成功，`remove()` :66 无条件弹「已删除」。

## 三、后端失败信封铁证（响应体来源）

- `backend/crates/bbs_assemble_control/src/lib.rs` :497-539 `create_topic`：:502 require_session、:504 uuid、:505-507 forum_id/title/content `unwrap_or_default`、:509 creator 取会话人、:523-530 INSERT x_bbs_topic、:538 `Ok(ActionResult::success(result))` 恒 success，**无 200 + type:error 失败分支**（误报前提不成立，同 R422 先例）。
- `backend/crates/file_assemble_control/src/lib.rs` :4099-4133 `recycle_id_delete`：:4107 SELECT FILE_FILE（id 且 deleted_at 非空）；:4112-4114 查无行 → `Ok(Json(ActionResult::error("file not in recycle bin")))`；:4124-4126 DELETE 影响 0 行 → 同 error；:4127 成功才 success。即**假 ID 产生 HTTP200 + type:error**。
- 路由 `backend/crates/file_assemble_control/src/routes.rs` :192 `DELETE /api/recycle/delete/{id}` → recycle_id_delete；:249 `DELETE /api/recycle/empty`；:184 另有 POST 同路径。
- `ActionResult::error` 采用 HTTP200 + `type:error` 是 o2server 兼容语义（产品语义差异，非 bug 本身）。

## 四、真后端假数据探针（不拦截，走真实后端）

脚本 `test-results/r423-h5-bbs-recycle-probe.cjs`（gitignored），会话真登录后直发请求，证据 `probe.json`：

- forumList = 200 success（count 8）。
- topicCreateFakeForum（假 forumId）= **200 type:success，真 INSERT 一条**（后端无外键门禁），证实发帖端点不产生 200 + error。
- recycleDeleteFake（假 ID）= **HTTP200 `{"type":"error","message":"file not in recycle bin",...}`**，证实回收站 P1 前提成立。

## 五、E2E 决定性对照（真 UI 点击 + 写响应体铁证，全程不拦截走真后端）

脚本 `test-results/r423-recycle.cjs` 与 `test-results/r423-bbs.cjs`（均 gitignored），`page.on("response")` 抓新写请求响应体，排除注入伪造。

### 回收站彻底删除（P1 坐实）

- DOM：标题命中「回收站」、彻底删除按钮 1、清空回收站按钮 1、注入行 1、门控 fold=0。
- 决定性对照：注入列表 1 行（id 为假 ID）→ 真点「彻底删除」→ uni-modal 点「确定」→ 真 `DELETE /api/recycle/delete/{fake}` **不拦截走真后端**，newWrites 捕获 HTTP200 `type:error file not in recycle bin`（writesDelta=1），仍弹绿 toast「已删除」，rowsAfter=1（注入行仍在）= **误报坐实，P1 三全满足**（真 UI 可复现 + 真请求响应体 + 源码定位 :65-66）。
- 取消零写：uni-modal 点「取消」→ writes=[]（writesDelta=0）。
- 非 2xx：500 → toast「HTTP 500」；403 → toast「没有权限执行该操作 / 没有权限」，均正确 error。
- 失败态：注入 list 500 → 「回收站加载失败：HTTP 500」+ 重试按钮 1；空态：「回收站为空」+ 无重试，二者可区分（修正同 hash 不重载问题后坐实）。
- 0 pageerror。

### bbs 发帖（0 缺陷，L1 隐患）

- DOM：picker 1、input 1、textarea 1、发布主题按钮 1。picker 需真点开滚轮后点 `.uni-picker-action-confirm`（「完成」）才写回 forumIndex（单纯 dispatchEvent(change) 无效），修正后 canSubmit 由 disabled=true 变 false。
- 决定性对照：真点「发布主题」→ 真 `POST /api/bbs/assemble/control/topic/create` **不拦截走真后端**，newWrites 捕获 200 `type:success`（真写入新帖，跳转主题详情可见），toast「发布成功」——成功语义与响应一致，**无误报，不判缺陷**。
- 表单校验零写：标题/内容/版块任一为空时 disabled=true，点击 writes=[]（合规）。非 2xx 500/403 注入发帖端点在表单合规时走 catch toast.error（本轮 P1 判定不依赖此端点）。
- 0 pageerror。
- L1 隐患：`submit()` :51 不读 `type`；当前后端恒 success 故不误报，若后端改为 200 + error 口径则同型误报风险成立（仅佐证，真后端无此形状，不判缺陷）。

## 六、三分判定与取证限制

- **真缺陷 P1（1 个，用户可达）**：`recycle.vue remove()` :65-66 —— discardResponse 吞信封 + 无条件成功 toast；真后端 200 + type:error 仍弹「已删除」，行仍在。
- **同型隐患 L1**：`recycle.vue emptyAll()` :104-126（清空回收站同为 discardResponse 模式，未单独真 UI 决定性对照）；`bbs/new.vue submit()` :51 不读 type（后端恒 success，当前无误报）。
- **正确模式**：bbs 发帖真后端 200 success → 发布成功一致；回收站非 2xx → SDK 抛错 → `catch` toast.error；showModal 取消零写；失败/空态可区分。
- **取证限制**：破坏性真删未执行，用假 ID + route 注入替代；清空回收站链未逐一真 UI 决定性对照；probe 中 topicCreateFakeForum 真写入一条测试帖（据实披露）；route 注入变体仅作佐证；H5 390x844 headless 非物理真机。
- **产品语义差异**：ActionResult error 采用 HTTP200 + type:error（o2server 兼容）；discardResponse 只用于无需响应体的删除类调用，但恰好吞掉了错误信封。

## 七、最小修复建议与交付证据

- 修复建议：`recycleApi.remove` / `empty` 取消 `discardResponse:true`（或 SDK 在 discardResponse 分支仍解析 `type`），`remove()` :65 接响应并守卫：失败 `toast.error(message || "删除失败")` 且不 `load()` 过滤、不弹「已删除」；成功才弹成功并刷新列表。bbs `submit()` :50 增 `type` 守卫作为防御。
- 关键截图（390x844 headless 非物理真机，sha256 已计算并目视确认）：
  - `r423-decisive-remove.png` = `7395decc08612271bdd7e5f86dec82c6bf62ed3b7f5a3ca06e822946da23db4a`（绿✓已删除 + 注入行仍在 + 真后端 200 error）。
  - `r423-bbs-create.png` = `343bf030a9b9cef9e34f7a3b2f3d054d9090fb6c55d02dbfab82a15bf9215e6f`（主题详情页，真发帖成功）。
  - `r423-500.png` = `d318ca4c0ba698f2e59cf1f95d9b474a0c189d57fb9185e987f664e5dd1a4d18`；`r423-403.png` = `ffa1821a3dbbf60e386fab8fe47f2d24e104afe0fdb52cd04f4653af05fb23e2`。
  - `r423-error-state.png` = `f32e27ae5687ca8e916cceddc7e25344662d0a334ca3540738118d12e4dbbb69`；`r423-empty-state.png` = `ab8077d28531ad726c5f89d9b963125cb57767510edd6b70a3068899168816ba`。
  - `r423-cancel.png` = `292a324948efd6ac4e1d332d50300e2f470c397ca876b663568f4da6db91284a`；`r423-picker-open.png` = `5d1159d249b2a538a173ba978993932123da5012466880b96540ec3d7fa43104`。
- 证据目录（gitignored）：`test-results/r423-h5-bbs-recycle-write/`；响应体与 DOM evidence JSON 同目录。

> 测试域名/主机以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，API 仅记相对路径，不含任何真值。
