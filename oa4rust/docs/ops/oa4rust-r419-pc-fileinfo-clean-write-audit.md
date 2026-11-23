# R419 PC 文件信息清理写链 + BAM 门控 甄别报告（只读）

> 证据来源：Playwright Chromium headless 1440x900 对已部署测试服的真实页面实测，**非物理真机**；截图均为该 headless 浏览器真实截屏。测试服地址按脱敏占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 表示，本文不含任何真实域名/IP/主机/凭据/本机绝对路径。

## 一、审计范围与方法

- 首选目标：`frontend/apps/desktop/src/views/FileInfoApp.vue` 的 `cleanUnusedFiles`（:113-122），路由 `main.ts:200-202/496` path=`fileinfo` / name=`FileInfoApp`，实际落点 `/app/admin/fileinfo`。
- 备选目标（首选判 0 缺陷后据实转）：`frontend/apps/desktop/src/views/BamApp.vue` 的 `bamWrite`（:544）/ `bamMore`（:571），路由 `main.ts:113` path=`bam` / name=`BamApp`。
- 任务性质：只读甄别（kind=work，不改任何源码/契约/后端/SDK），三分判定（真实缺陷 P1 / 0 缺陷 / 产品语义差异）。
- 方法：逐函数甄别真实入口 → 真后端假数据探针据实判定失败信封 → 决定性对照（写响应 route 注入，仅作前端是否读 type 的佐证）→ 非 2xx、confirm 取消、失败/空态、pageerror 核验；BAM 另做 EndpointFold 展开前后 DOM 审计。
- 破坏性约束：`cleanUnusedFiles` 对应后端 `DELETE FROM FILE_FILE WHERE reference_type=$1` 全表批量删，**未以 admin 会话真调用**；改用无会话探针（401 在 DELETE 前拦截，无副作用）+ route 注入替代，据实披露取证限制。

## 二、源码甄别

### FileInfoApp.cleanUnusedFiles
- 模板真实入口：第 11 行 `<button class="btn-primary" @click="cleanUnusedFiles">清理无引用文件</button>`，与第 10 行刷新按钮并列于 toolbar；无 EndpointFold 门控（grep EndpointFold|devToolsEnabled|import.meta.env.DEV|defineExpose 全 0）。E2E DOM 审计 `cleanBtns=1`/`visibleCleanBtns=1`/`fold=0`，生产构建真实可达。
- `cleanUnusedFiles`（:114-122）：:115 `if (!(await confirmMsg(...))) return`（confirm 守卫）；:117 `await api.get("/api/file/clean/unused/referencetype/cmsdocument/manage")`；:118 `toast.success("已清理无引用文件："+(data.deleted ?? data.count ?? 0))` 不读 type；catch :119-120 仅在非 2xx 抛错时 `toast.error`。
- 只读/展示类剔除：`docMeta`（:103-112，GET fileinfo/{id}/document/{docId} 元数据展示）、`downloadFile`（:77-87，GET 下载链接）、`loadFiles`（:89-101，GET fileinfo/list/all）—— 均非本写链。
- SDK `api.ts` 不校验 type，`api.get` 仅非 2xx 抛错。

### BamApp.bamWrite / bamMore
- 源码：模板第 9/39 行 `<EndpointFold label="高级操作" ... test-id="bam-endpoint-fold">`，写按钮（第 26-37 行 bamWrite 七分支 + bamMore 五分支）全部包在 `<template #group-write>` 内；第 21-23 行 bamMore 三分支在 `#group-read` 内（只读展示类存根查询，属本链外）。
- 门控组件 `components/admin/EndpointFold.vue:6` `v-if="devToolsEnabled"`，:55 `devToolsEnabled: import.meta.env.DEV` —— 生产/测试构建 `DEV=false`，整段 `<section>` 及全部插槽按钮不渲染。
- 写函数 `bamWrite`（:544）/: `bamMore`（:571）虽裸 `toast.success("BAM操作已提交")` 不读 type（同型静态候选），但生产构建不可触达，列 L1 潜在隐患不判缺陷。

## 三、后端铁证（失败信封形状）

- 路由：`file_assemble_control/src/routes.rs:152` `.route("/api/file/clean/unused/referencetype/cmsdocument/manage", get(file_clean_unused_referencetype_cmsdocument_manage))`。
- Handler：`lib.rs:2814-2843` `file_clean_unused_referencetype_cmsdocument_manage` —— :2819-2825 会话门禁：无会话 `Err(AppError::Unauthorized)`（401）、非 admin `Err(AppError::Forbidden)`（403），均非 2xx；:2827-2833 `DELETE FROM FILE_FILE WHERE reference_type = "cmsdocument_manage" AND deleted_at IS NULL`（result=rows）；**:2834-2842 唯一返回 `Ok(Json(ActionResult::success({deleted: N, success: true})))`，即 DELETE 命中 0 行仍是 HTTP 200 + type:success + success:true**。
- **结论：该端点不存在 HTTP 200 + type:error 或 type:success + success:false 的失败信封分支**，误报成功前提不成立。后端单测 `lib.rs:1103-1288`（clean_unused 401/403/admin 放行断言）印证门禁语义。

## 四、真浏览器实测（决定性对照与门控审计）

- DOM 审计：URL `/oa4rust/app/admin/fileinfo`、标题命中文件信息、清理按钮 1（可见 1）、fold=0、0 pageerror。
- 真后端假数据探针（安全，无会话在 DELETE 前被拦截）：`GET /api/file/clean/unused/referencetype/cmsdocument/manage` credentials=omit → HTTP **401** `{"type":"error","message":"unauthorized", ...}`，SDK 抛错 → catch → toast.error（非 2xx 正确错误路径）。
- 决定性对照（route 注入，仅作前端是否读 type 的佐证，**非真后端失败信封**）：注入 200 `type:error/success:true`、200 `type:success/success:false`、200 `type:success/success:true` 三种形状，真 UI 点「清理无引用文件」→ confirm → 均弹绿 `status|✓已清理无引用文件：0`，`writesDelta=1`，newWrites 捕获注入响应体。证实前端不读 type，但真后端不产生前两种失败形状，故不判缺陷。
- confirm 取消：点清理 → confirm 选「取消」→ `writesDelta=0`、无 toast，零写守卫正确。
- 非 2xx：注入 500 → 红 `alert|✗清理失败: HTTP 500: Internal Server Error`；403 → 红 `alert|✗清理失败: 没有权限执行该操作`，均正确。
- 失败/空态可区分：列表注入 500 → `hasFail=true`/重试按钮 1/`暂无文件信息=false`；空 data → `hasFail=false`/重试 0/`emptyHint=true`（暂无文件信息），两者可区分。
- BAM 门控审计（严格按 R416 ProcessManagerApp / R388 CmsIndexApp 口径）：URL `/oa4rust/app/bam`，**展开前** `fold=0`/`foldTrigger=0`/写按钮 count=0；尝试点击展开触发器（`[data-testid=bam-endpoint-fold-toggle]` count=0，无法点击）；**展开后** `fold=0`/写按钮 count=0/可见写按钮 0。页面 DOM 仅「🔄刷新」按钮与统计卡片，高级操作整段不渲染。生产门控后写按钮 DOM=0，判不可达。
- `pageErrors=[]`，`totalWrites=6`。

## 五、三分判定

**判定：0 个用户可达真缺陷（两页均不改码）。**

- FileInfoApp.cleanUnusedFiles —— 0 缺陷（误报前提不成立）：后端失败仅 401/403 非 2xx（SDK 抛错→catch→toast.error），DELETE 0 行仍 200 success，无 200+type:error 失败信封。route 注入 200 error 仍弹成功仅证明前端不读 type，真后端不产生此形状，按 R400/R411 口径不判缺陷；列 L1 潜在隐患（若后端未来改口径则前端会误报）。
- BamApp.bamWrite/bamMore —— 0 缺陷（生产门控不可达）：EndpointFold v-if=devToolsEnabled(DEV=false)，E2E 展开前后写按钮 DOM=0，无真实可达入口；列 L1 潜在隐患（仅 DEV 构建可达）。
- 通过项：真后端 401 探针、route 注入三形状对照、500/403 正确 toast.error、confirm 取消零写、失败/空态可区分、BAM 门控展开前后审计、0 pageerror。

## 六、取证限制与产品语义差异

- 取证限制：① 破坏性真清理（admin 会话真删 FILE_FILE）未执行，用无会话 401 探针 + route 注入替代；② admin 会话真后端调用返回 200 success(deleted:N) 未实调（会真删数据），依据源码 :2834-2842 唯一返回判定；③ route 注入 200 error 属伪造形状仅佐证；④ 全量按钮按代表性覆盖；⑤ BAM 门控后无写按钮，非 2xx/取消零写在该页不适用据实标注。
- 产品语义差异：`ActionResult::error` 走 HTTP 200 是 o2server 兼容既有设计，但本端点失败（401/403）走 AppError 非 2xx，成功走 200 success，与"误报成功"模式的失败信封不同；EndpointFold 高级操作默认仅 DEV 模式可见是刻意的产品设计（生产首屏清爽）。

## 七、证据截图（1440x900 headless 非物理真机，sha256）

- `r419-fileinfo-decisive-typeerror.png` = `17815fd6768c5c787ef19192434c568ad04f5656a101b91d7e4f0de5e02c32b4`（注入 200 type:error 仍弹绿 toast 已清理 0，佐证前端不读 type，已目视确认）。
- `r419-fileinfo-500.png` = `b80314d2f2b66b9020f2f3f9f1dc0df928506062acfd6ee2bdb55fab6899607e`（红 toast 清理失败 HTTP 500，已目视确认）。
- `r419-bam-fold-audit.png` = `82fd4b64deb2cd42ee2895e7ebac73c7df7cc01629f53628659e69acce0d6d45`（BAM 页仅刷新按钮、无高级操作与写按钮，门控不可达坐实，已目视确认）。
- `r419-fileinfo-probe.png` = `7d9a1226798c6a63e03bd3d6cab9bb6ff79a68a840f05129647beec837c2091b`（无会话探针阶段 DOM）。
- `r419-fileinfo-403.png` = `5c1fd1e9c90544702a865dfe44fe94c53027c40d79fa74a8ccdf64b309d4f7b3`；`r419-fileinfo-cancel.png` = `756081d63c97fd3b3e58954d96b30110114d6d51347696f5800541abc2f7a846`；`r419-fileinfo-error-state.png` = `47b2ac80392634e09a319e356fedf60d498d9105dab9644e44b14d660fdf37aa`；`r419-fileinfo-dom-audit.png` = `7d9a1226798c6a63e03bd3d6cab9bb6ff79a68a840f05129647beec837c2091b`。
- 证据目录（gitignored）：`test-results/r419-pc-fileinfo-clean-write/`（含 evidence.json 与全部截图）。