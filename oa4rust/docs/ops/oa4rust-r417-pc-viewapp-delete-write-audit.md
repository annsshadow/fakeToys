# R417 PC 视图管理删除写链 200+type:error 误报成功 甄别报告

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图均为该 headless 浏览器真实截屏。

## 一、审计范围与方法

- 目标页：`frontend/apps/desktop/src/views/ViewApp.vue`（路由 `main.ts:194-198` path=`view` / name=`ViewApp` / title=视图管理，实际落点 `/app/admin/view`）。
- 任务性质：只读甄别（kind=work，不改代码），三分判定（真缺陷 P1 / 隐患 L1 / 正确模式）。
- 方法：逐函数甄别真实 UI 入口 → 真后端假 ID 探针坐实失败信封 → 决定性对照（写响应全程走真后端、不拦截，以 newWrites 响应体为铁证排除注入伪造）→ 非 2xx、confirm 取消、失败/空态、pageerror 守卫核验。
- 破坏性真删未执行：全程使用假 ID + 列表注入，UPDATE/软删命中 0 行，无真实业务数据被删除。
- 测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据。

## 二、源码甄别

- 模板真实入口：第 32 行 `<button class="btn-sm danger" @click="removeView(v)">删除</button>` 位于 v-for 视图表行内，E2E DOM 审计 `deleteBtns=2`、可见 `visibleDeleteBtns=2`，无需展开任何折叠门控。
- 门控核查：`grep EndpointFold|devToolsEnabled|import.meta.env.DEV|defineExpose` 全部命中 0，生产构建真实渲染，E2E `fold=0` 一致。
- `removeView`（:125-134）：:126 `if(!(await confirmMsg("确认删除该视图？")))return;`（有 confirm 守卫）；:128 `await api.delete(\`/api/view/${v.id}\`)`；**:129 `toast.success("已删除")` 无条件执行，不读取响应 type/success**；:130 `loadViews()`；catch :131-132 仅在非 2xx 抛错时 `toast.error`。
- 只读/展示类剔除（不判缺陷）：`viewDetail`（:101-123，GET view/{id} 等，展示 categoryCount/fieldCount）、`viewData`（:172-190，POST viewdata/list 只读展示）、`loadViews`（:158-170，GET view/list/all）、`loadViewCategories`（:137-146）、`loadFieldConfigs`（:147-156）及其 metaText 展示。
- SDK `frontend/packages/sdk/src/api.ts` 的 `parseJsonEnvelope` 不校验 type，`api.delete` 仅非 2xx 抛错；HTTP 200 + type:error 不会进入 catch。

## 三、后端铁证

- 实际 DELETE 路由：`backend/crates/cms_assemble_control/src/routes.rs:566` `.route("/api/view/{id}", delete(view_u2_delete))`。
- Handler：`cms_assemble_control/src/lib.rs:10056-10071` `view_u2_delete` —— :10061 `u2_check_owner(&pool,"x_cms_view","creator",&id,...)`；**:10062 `U2Gate::NotFound => Ok(Json(ActionResult::error("view not found")))`，即假 ID 命中 HTTP 200 + type:error（o2server 兼容既有设计）**；:10064-10068 Allowed 分支才 `soft_delete_by_id` 并返回 success{deleted:true}。
- 注意区分：`lib.rs:1191` 的 `view_delete` 属 `/api/cms/assemble/control/view/delete/{id}` 旧端点（routes.rs:269-270），非本写链。

## 四、真浏览器实测（决定性对照，写响应不拦截）

- DOM 审计：URL `/oa4rust/app/admin/view`、标题命中视图管理、删除按钮 2（可见 2）、fold=0、0 pageerror。
- 真后端假 ID 探针：`DELETE /api/view/R417-FAKE-VIEW-...` → HTTP **200** `{"type":"error","message":"view not found", ...}`（prompt=ExceptionEntityNotExist），误报前提成立。
- 决定性对照：注入列表 1 行「R417测试视图」(id=假 ID) → 真 UI 点「删除」→ confirm 确认 → 真 `DELETE /api/view/{fake}` **全程走真后端、不拦截**；newWrites 捕获真响应体 `s:200 / type:error / view not found`，`writesDelta=1`，界面仍弹绿色 `status|✓已删除`，且 `rowStill=1`（本地行仍在，失败未被识别）——**误报坐实，三全满足**。
- confirm 取消：再次点删除 → confirm 选「取消」→ `writesDelta=0`、无 toast，零写守卫正确。
- 非 2xx：注入写响应 500 → 红 `alert|✗删除失败: HTTP 500: Internal Server Error`；403 → 红 `alert|✗删除失败: 没有权限执行该操作`，均正确不误报。
- 失败/空态可区分：列表注入 500 → `hasFail=true`/重试按钮 1/`rows=0`（视图列表加载失败）；空 data → `hasFail=false`/重试 0/`emptyHint=true`（暂无视图数据），两者可区分。
- 注入变体对照：`type:error,success:true` 与 `type:success,success:false` 两种 200 响应下仍弹 `status|✓已删除`，佐证前端不读 type；真后端不产生这些形状，仅作佐证不单独判缺陷。
- `pageErrors=[]`、`totalWrites=6`。

## 五、三分判定

**判定：1 个用户可达 P1 真缺陷（三全满足）。**

- P1：`removeView`（:128 写、:129 裸 `toast.success("已删除")` 不读 type）在后端 HTTP 200 + type:error 下误报删除成功，真 UI 可复现 + 真响应体 + 源码定位三项齐备。
- 通过项：假 ID 探针 200 error、决定性对照走真后端、500/403 正确 toast.error、confirm 取消零写、失败/空态可区分、无门控真实可达、0 pageerror。
- 不适用：prompt 取消零写（本写链仅 confirm 无 prompt）。

## 六、取证限制与产品语义差异

- 取证限制：破坏性真删未执行，用假 ID + 列表注入替代（软删命中 0 行无副作用）；全量按钮按代表性覆盖；注入假 ID 走真后端时 loadViews 可能被详情 GET fallback 影响，据实说明。
- 产品语义差异：`ActionResult::error` 走 HTTP 200 是 o2server 兼容既有设计，权威失败标志为响应 `type`。

## 七、最小修复建议

- 新增 `viewDeleteWriteSucceeded(resp:any)` 以 `type` 为主、不强求 bool（type!==error 视为成功）；`removeView` 承接 :128 的 await 响应，仅成功分支才 `toast.success("已删除")` 并 `loadViews()`，失败分支 `toast.error` 透传 message 并 return，恢复列表原状不误删行。

## 八、证据截图（1440x900 headless 非物理真机，sha256）

- `r417-decisive-delete.png` = `43426b350c87eb6716f019b99ff502c69eab3cfab6df3e399a49841382002372`（绿 toast「✓已删除」+ 注入行仍在 + 真后端 200 error，已目视确认）。
- `r417-500.png` = `e32c46762594ec06508b63d5d4f1b49b3600c77c36ae8ec06e33b6791c094ea6`（红 toast「删除失败: HTTP 500」，已目视确认）。
- `r417-403.png` = `360720dd3c0a1fcd18da5ec30ba7760a2934d39b3567c78755235a31735bea45`。
- `r417-error-state.png` = `969dc13eced2fd706afc7fe03339399b77a55bce6cc2dbcb3ca5fcdbc72073d5`。
- `r417-empty-state.png` = `b5bc35b8af9e4445259b1e25699665bb63e5b2e198f0e9e598a143890f492d50`。
- `r417-dom-audit.png` / `r417-probe-done.png` = `5a4f1ec3437002f1149168a1b7b2476002880904419dad722062c0b4eaf35579`。
- 证据目录（gitignored）：`test-results/r417-pc-viewapp-delete-write/`（含 evidence.json 与全部截图）。