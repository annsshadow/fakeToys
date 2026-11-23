# R407 PC 门户页面设计器（PortalPageDesignerApp）布局写链甄别报告

> 审计性质：只读甄别（不改业务代码）。测试环境：部署测试服 + Playwright Chromium headless，视口 1440x900，**非物理真机**。所有截图均为真浏览器渲染，非模拟图。

## 一、审计范围与结论

- 目标文件：frontend/apps/desktop/src/views/PortalPageDesignerApp.vue（336 行）；路由 main.ts:362-364 path=portal-page-designer，:507 重定向。
- 任务：甄别门户布局写链在后端 HTTP 200 + type:error 失败信封下是否误报成功。
- 结论：**1 个用户可达 P1 真缺陷（删除分支，三全满足）**；保存分支为 L1 潜在隐患（真后端恒 success，误报前提不成立，本轮不判缺陷）。
- git 基线：c17afe973eb8b1fe1ae3c8e58ca120f55222f5a6，工作树干净。

## 二、源码甄别（逐函数）

- 门控检查：EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 全部 grep 命中 0，DOM 审计 fold=0，无生产门控，写按钮真实可达。
- confirmMsg 在 :62 已正确导入（与 R386 ServerApp 缺失 import 的 bug 不同）。
- 真实 @click 写入口仅 layoutOp(:101-119) 四分支，模板 :18 savePost / :19 savePut / :20 deletePost / :21 deleteDel：
  - 保存分支（:103-108）：prompt 取名后 :106 POST / :107 PUT /api/portal/assemble/surface/save/layout，随后 :108 无条件 toast.success（布局已保存 (op)），不读取响应 type。
  - 删除分支（:110-114）：prompt 取 id，:111 confirmMsg 守卫，:112 POST / :113 DELETE /api/portal/assemble/surface/delete/layout，随后 :114 无条件 toast.success（布局已删除 (op)），不读取响应 type。
  - catch(:116) 仅对非 2xx 抛错走 toast.error，对 2xx + type:error 不设防。
- 只读剔除：loadPortalMeta(:134)/loadPortalMeta2(:120)/loadPortalMeta3(:84)/loadData(:289)/loadPageMeta(:223) 等 GET 探测；无死代码。
- CRUD 主列表正确模式：saveItem(:270)→saveM(:250-269) 与 deleteItem(:283)→delM(:276-282) 用 useMutation，onError toast.error、onSuccess 仅 invalidateQueries + closeModal，无成功 toast，不误报。
- SDK：frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 不查 type，:296 仅非 2xx 抛错；故 200 + type:error 被原样 resolve。

## 三、后端信封铁证

- backend/crates/portal_assemble_surface/src/lib.rs：
  - save_layout(:799-859)：admin 门 :805，updated=0 时 :826-834 直接 INSERT 返回 success，:844 ActionResult::success（含 id/name/content），全程无 ActionResult::error 分支——保存分支真后端恒 success，不产生 200 + type:error。
  - delete_layout(:862-895)：DELETE 0 行 → :883 Ok(Json(ActionResult::error("layout not found"))) 即 HTTP 200 + type:error；成功时 :886 success + deleted。
- 路由 lib.rs:585 POST delete / :610 POST save / :620 DELETE delete / :622 PUT save（双注册）。
- ActionResult::error 走 HTTP 200 是 o2server 兼容的既有设计（与同型批次一致）。

## 四、真浏览器实测

- 脚本（gitignored）：test-results/r407-pc-portalpage-layout-write.cjs + run.sh；证据同目录 evidence.json 与截图。1440x900 headless 非物理真机，pageErrors=[]，totalWrites=7。
- S0 DOM：按钮 13 个，页面标题命中，savePost/savePut/deletePost/deleteDel 各 1，fold=0，空态（暂无门户页面）可见。
- S1 真后端假 ID 探针（仅 delete 假 ID 安全，save 真打会 INSERT 真布局故不打）：deleteLayoutPostFake / deleteLayoutDelFake 均 200 + type:error（layout not found，prompt=com.x.base.core.project.exception.ExceptionEntityNotExist）；listProbe=200 success+count 0。
- S2 决定性对照（真 UI 点删除 + prompt 填假 ID + confirm 确认，写响应全程不拦截走真后端）：
  - deletePost：真 POST /api/portal/assemble/surface/delete/layout = 200 + type:error（newWrites 捕获真响应体），仍弹绿 toast「布局已删除 (deletePost)」，writesDelta=1。截图 r407-decisive-delete-post.png。
  - deleteDel：真 DELETE 同路径 = 200 + type:error，仍弹绿 toast「布局已删除 (deleteDel)」，writesDelta=1。
  - prompt 取消：writesDelta=0 零写；confirm 取消：writesDelta=0 零写、toasts=[]。
- S3 非 2xx：500 → 红 toast「布局操作失败: HTTP 500: Internal Server Error」；403 → 红 toast「布局操作失败: 没有权限执行该操作」。截图 r407-500.png。
- S4 人造 200 + type:error stub（save 分支）：仍弹绿 toast「布局已保存 (savePost)」——仅佐证前端不读 type；真后端不产生此形状，不单独判缺陷。截图 r407-stub-save.png。
- S5 失败/空态：errorState.hasFail=false、retryBtns=0（列表 500 注入未坐实失败文案，据实记取证限制）；emptyState 空态（暂无门户页面）可见、rows=0。

## 五、三分判定

- **真 P1（1 个）**：删除分支 layoutOp(:110-114) 在真后端 200 + type:error（layout not found）下 :114 无条件 toast.success（布局已删除），三全满足——真 UI 可复现（deletePost/deleteDel 两变体）、真请求响应（newWrites 200+error 铁证）、源码定位（:114 不读 type）。deletePost 与 deleteDel 同一函数同一缺陷，计 1 个。
- **L1 潜在隐患（非本轮缺陷）**：保存分支 :108 无条件成功不读 type；真后端 save_layout 恒 success 不产生 200+error，误报前提不成立，仅记录建议。
- **通过项**：delete 假 ID 探针 200+error 坐实；决定性对照写响应不拦截走真后端；prompt 取消零写；confirm 取消零写；500/403 正确 toast.error；空态可区分；无门控、fold=0；CRUD 主列表 useMutation 正确模式；confirmMsg 已导入；0 pageerror。

## 六、取证限制

- 破坏性真写（真建布局 / 真删真布局）未授权未执行，用假 ID + 不拦截对照替代。
- save 分支真打会 INSERT 真布局，故未做真后端 save 假 ID 探针，以源码甄别 + stub 佐证其不读 type。
- 列表 500 失败态文案未坐实（errorState.hasFail=false / retryBtns=0），据实记录，不虚报通过。
- 全量按钮按代表性覆盖（四分支删除两变体 + 保存 POST 变体实测）。

## 七、最小修复建议与产品语义差异

- 最小修复（对齐 R363/R402/R406 范式）：新增 portalLayoutWriteSucceeded(resp:any) 以 type 为主不强求 bool；layoutOp 删除/保存分支接住响应，仅成功时 toast.success，失败时 toast.error 透传 message 且不弹成功。
- 产品语义差异：ActionResult::error 走 HTTP 200 是 o2server 兼容既有设计；save_layout 用 INSERT 兜底（updated=0 即新建返回 success）与 delete_layout 的 0 行 error 口径不同，属既有实现差异。
- 测试环境占位：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>（真值不入库）；本报告只含相对路径与 API 路径，无本机绝对路径、无密钥口令。