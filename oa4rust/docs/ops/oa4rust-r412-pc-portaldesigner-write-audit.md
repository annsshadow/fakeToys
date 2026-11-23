# R412 PC 门户设计器写链误报成功甄别（只读审计）

> 本轮为只读甄别，不改业务代码。浏览器实测为 1440x900 headless Chromium，**非物理真机**。所有截图为自动化浏览器截图，非真机截图。测试域名/主机以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，证据存于 gitignored 相对路径 `test-results/r412-pc-portaldesigner-write/`。

## 一、审计范围与判定

- 目标：`frontend/apps/desktop/src/views/PortalDesigner.vue`（门户设计器）写链在后端 HTTP 200 + `type:error` 下是否误报成功。
- 路由：`main.ts:242-244` path=`portal-designer` / name=`PortalDesigner` / meta.title=门户设计器；无 EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 门控（全文 grep 为空），生产构建真实渲染，DOM 审计 fold=0。
- **三分判定：3 个用户可达 P1（真缺陷）**，均为「后端业务失败仍弹成功 toast」，三全（真 UI 可复现 + 真请求响应 + 源码定位）满足。
- 审计基线：HEAD `5a89e83bee12ab78807b32e8047207cd31aaec82`，分支 `opt1000/oa4rust-pc-ai-refactor`。

## 二、源码甄别（坐实）

模板真实 @click 写入口（第 13/15/16/17/19/20 行）：按 ID 保存、删页面、模板页分类、删组件、资源内容上传、保存布局；第 8-12 行新建/资产/分类/分页/输出为只读或纯本地状态。

1. **saveDesign（:355-366，P1）**：`:356 if(!activeId)return`；`:359 await api.put(designerPaths.portalSave(activeId.value), {content})` 不接响应、不读 `type`；`:360` 无条件 `toast.success("门户布局已保存")`；`:361 catch` 仅非 2xx 报错。端点 `PUT /api/portal/assemble/designer/save/{id}`。
2. **saveById（pdDesignerOps :368-409 内 :370-372 + :405，P1）**：`:372 await api.post(/api/portal/assemble/designer/save/${id}, {})`（prompt 取 id）不读 `type`；`:405` 统一无条件 `toast.success("门户设计器操作已提交")`。
3. **fileUpload（pdDesignerOps :385-399 内 :395-396，P1）**：`:395 await api.post(file/upload/{fid}, {content})`（prompt 取 fid + 动态 file input 的异步 onchange）不读 `type`；`:396` 无条件 `toast.success("资源内容已上传")`；`:399 return` 提前跳出，不经过 :405，故只弹 :396 单个成功 toast。

剔除项（非本缺陷）：`pageDelete` / `widgetDelete` / `pageVersions` / `tplCategory` 同在 pdDesignerOps，:405 统一裸成功 toast 属同型风险；但 `widget_delete` 后端用 shared::crud_delete 恒 success（无 error 分支）、`tplCategory` 纯查询恒 success，二者无误报前提。`pageDelete` 后端可产生 200 error（见第三节），但受取证限制未能真 UI 走通（见第六节）。`SDK api.ts` 的 `parseJsonEnvelope` 不查 `type`，仅非 2xx 抛错，是误报根因之一。

## 三、后端铁证（HTTP 200 + type:error）

- `backend/crates/portal_assemble_designer/src/lib.rs` `save_design`(:216-249)：`:230-236 UPDATE x_portal_design ... WHERE id=$2`；`:238-240` 影响 0 行返回 `Ok(Json(ActionResult::error("design not found")))`，即 HTTP 200 + `type:error`；成功才返回 success。路由 `lib.rs:574 POST /designer/save/{id}`、`:697 PUT 同`。
- `delete_page`(:457-482)：DELETE 影响 0 行 → `:470-474 error("page not found or already deleted")` HTTP 200。路由 `:567 POST / :693 DELETE page/delete`。
- `widget_delete`(:4064-4076) 恒 success{deleted:bool} 无 error 分支；`templatepage_list_category`(:3052-3084) 纯查询恒 success。
- `file_id_upload`（路由 `lib.rs:599` POST file/upload/{fid}，shared/u2 实现）假 ID 实测返回 200 `type:error(file not found)`。
- `ActionResult::error` 走 HTTP 200 是 o2server 兼容既有设计，全批次同口径。

## 四、真浏览器实测（决定性对照，写响应不拦截走真后端）

登录经真实 UI，BASE 由占位 `<YOUR_TEST_DOMAIN>` 提供、API 为 `/oa4rust-api` 相对路径；全程 `pageerror=[]`。

1. **DOM 审计**：标题「门户设计器」命中；保存布局 / 按 ID 保存 / 删页面 / 删组件 / 模板页分类 / 资源内容上传各 1 个真实按钮；fold=0。
2. **真后端假 ID 探针（6 端点，直打不注入）**：`savePutFake`=200 error(design not found)；`savePostFake`=200 error(design not found)；`pageDeleteFake`=200 error(page not found or already deleted)；`widgetDeleteFake`=200 success(deleted:false)；`tplCategory`=200 success(count0)；`fileUploadFake`=200 error(file not found)。
3. **saveDesign 决定性对照**：注入 list 1 行（R412 测试设计，假 id）→ 点设计行 → 点「保存布局」→ 真 `PUT /api/portal/assemble/designer/save/{fake}` 不拦截走真后端 = 200 `type:error(design not found)`（newWrites 捕获真响应体），writesDelta=1，界面仍弹绿 ✓「门户布局已保存」= **误报坐实**。截图 `r412-decisive-save.png`（sha256 `d0a4e33334be404295a0d144b5e28eeb6ceb8f8973e1fed7bb4b3fb1ea4f459d`）。
4. **saveById 决定性对照**：点「按 ID 保存」prompt 填假 ID → 真 `POST /designer/save/{fake}` = 200 `type:error(design not found)`，writesDelta=1，仍弹绿 ✓「门户设计器操作已提交」= **误报坐实**。prompt 取消 writesDelta=0 零写。截图 `r412-decisive-savebyid.png`（sha256 `8144a06cde09aab24c96dbbf0c8f712c7c30ccb9790d45eb14bc1be13beb386a`）。
5. **fileUpload 决定性对照**：点「资源内容上传」prompt 填假 ID → filechooser 设测试文件 → 异步 onchange 真 `POST file/upload/{fake}` = 200 `type:error(file not found)`，writesDelta=1，仍弹绿 ✓「资源内容已上传」= **误报坐实**。截图 `r412b-decisive-fileupload.png`（sha256 `66a377ebecc931590b7058bf6c6bb26a7a85064bcf7488701f89d30e73aa044b`）。
6. **非 2xx 正确**：`500` → 红 ✗「保存失败: HTTP 500: Internal Server Error」；`403` → 红 ✗「保存失败: 没有权限执行该操作」。截图 `r412-500.png` / `r412-403.png`。
7. **失败态/空态**：列表 500 → 有失败文案、designs=0（可区分）；空 data → designs=0（仅列表空，无专门空态文案，据实记录）。截图 `r412-error-state.png` / `r412-empty-state.png`。

主 E2E：exit0、totalWrites=9、0pageerror；补测 exit0、0pageerror。

## 五、通过项（非缺陷）

- 假 ID 探针 6 端点信封与源码/后端一致。
- 500 / 403 非 2xx 正确 `toast.error`，不弹成功。
- prompt 取消零写（saveById writesDelta=0）。
- confirm 取消零写（writesDelta=0）。
- 失败态与空态可区分；无门控、真实可达；全程 0 pageerror。
- `widget_delete` / `tplCategory` 后端恒 success，无误报前提。

## 六、取证限制与产品语义差异

- 破坏性真写未执行，全部用假 ID + 列表注入替代；UPDATE / DELETE 影响 0 行，无真实数据副作用。
- `pageDelete` / `widgetDelete` 真 UI 决定性对照未打通：点击后 prompt（dialog 事件）被 accept(假 ID)，但 `#oa4-confirm-overlay` 始终 present=false、writesDelta=0，confirm 按钮 click 超时。根因为 prompt 同步阻塞与 confirmMsg 的 DOM overlay 渲染时序/上下文交互问题，未能走到真 DELETE。据此二者 200 error 误报**未真 UI 坐实**，列为同型隐患据实记录，不单独计为已坐实 P1；端点信封已由探针证实（pageDelete 200 error，widgetDelete 恒 success 无 error）。
- 空态无专门文案；全量写按钮按代表性覆盖（3 处 P1 均已决定性对照），未逐一开始变体。
- 产品语义差异：`ActionResult::error` 走 HTTP 200 属 o2server 兼容既有设计，权威判据为响应体 `type` 字段，不能仅凭 HTTP 2xx 判成功。

## 七、最小修复建议（本轮不改码）

新增 `portalDesignerWriteSucceeded(resp:any)`（以 `type` 为主、不强求 bool）：`saveDesign` 承接 :359 响应、`saveById` 承接 :372 响应、`fileUpload` 承接 :395 响应，仅成功分支才 toast.success；失败 `toast.error` 透传 message，不弹成功。`pageDelete` / `widgetDelete` 统一 :405 裸 toast 的同型风险随 helper 一并收口。
