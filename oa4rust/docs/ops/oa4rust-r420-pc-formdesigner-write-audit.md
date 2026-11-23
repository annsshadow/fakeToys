# R420 PC 表单设计器 FormDesigner 写链 200+type:error 误报成功甄别

> 取证环境：Playwright Chromium headless，视口 1440x900，针对已部署测试服；**非物理真机**。破坏性真实写未执行，使用假 ID + route 注入替代并据实披露。

## 一、审计范围与结论

- 目标文件：frontend/apps/desktop/src/views/FormDesigner.vue（2882 行）；路由 main.ts:218-222，path=form-designer，标题「表单设计器」。
- 门控核查：grep EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 全为 0；E2E DOM 审计 fold=0，生产构建真实可达。
- 结论：**1 个用户可达 P1 真缺陷**（saveForm 更新分支），三全满足；submitPreview 判死代码不判缺陷；其余为只读或正确模式。
- 次候选 FindDesignerApp.vue 未转页（本页已坐实 P1）。

## 二、源码甄别

### P1：saveForm（:1082-1123，模板 :15 真实 @click）

- :1083 name 空 → toast.info(请输入表单名称) return（校验类，剔除）。
- :1089 appId = currentForm.appId || routeAppId || "default"。
- :1106 payload = formSavePayload(definition, {name, appId, status})。
- :1111 更新分支：if(currentForm.id) await api.put(`/api/form/${id}`, payload) —— **不接响应**。
- :1113-1114 创建分支：await api.post("/api/form", payload)；currentForm.id = created?.data?.id —— 有 id 校验，属部分正确模式，真后端 default 桶恒 success，不误报。
- :1116-1118 写回 definition/appId + await loadForms()。
- :1119 **无条件 toast.info("保存成功")，不读响应 type**；catch :1120-1121 仅非 2xx 抛错。

### 死代码：submitPreview（:1267-1277，不判缺陷）

- :1270 await api.post("/api/form/submit", {formId, data}) → :1271 toast.info("提交成功") 不读 type；但**全量 grep submitPreview 仅 :1267 定义处 1 处**。
- 模板 :126 form @submit.prevent 无绑定，:163 提交按钮未绑 @click/@submit，无 defineExpose。
- E2E 实测 DOM 提交按钮数 submitBtnCount=0、点击后 submitWrites=[]、writesDelta=0 —— **真浏览器坐实不可达**。队长静态备料称「模板:163 提交按钮可达」据实纠正。

### 5 处 toast.info 校验/展示类（剔除）

- :1084 请输入表单名称 / :1289 JSON 格式错误 / :1420 请先选中一个字段 / :1427 已应用 N 条规则 / :1558 导入失败无效 JSON。

### 只读/本地（剔除）

- loadForms(:1031-1049 GET /api/form/list/all 或 /app/{appId}，:1045 listError=describeListError) / loadForm(:1050-1077 GET /api/form/{id}) 只读。
- resetForm(:1078) / togglePreview(:1124) 纯本地状态。
- SDK api.ts:82-88 parseJsonEnvelope 不查 type，仅非 2xx 抛错。

## 三、后端铁证（精确文件 grep 坐实，glob 0 命中不可信）

- routes.rs:554 POST /api/form=form_u2_create；:555 POST /api/form/submit=form_submit；:556 PUT /api/form/{id}=form_u2_update；:557 DELETE=form_u2_delete。
- lib.rs:9737-9768 form_u2_update：:9746 u2_check_owner → :9747 U2Gate::NotFound → Ok(Json(ActionResult::error("form not found"))) 即 **HTTP 200 + type:error**；:9748 Forbidden → AppError 非 2xx；:9750-9765 Allowed 才 UPDATE 返回 success。
- lib.rs:9603-9651 form_u2_create：:9628 application not found（200 error，仅 app_id≠default）；:9650 success+id；default 桶跳过校验恒 success。
- lib.rs:9659-9733 form_submit：:9669/:9675 formId/data 缺 → AppError::BadRequest(400)；:9689 form not found → AppError::BadRequest(400)；:9727 恒 ActionResult::success(valid/errors) 只校验不落库 —— 失败全非 2xx，无 200+error。
- xform.ts:323-334 formSavePayload：:327 if(!appId.trim()) throw(appId required)。
- ActionResult::error 为 HTTP 200 是 o2server 兼容既有设计。

## 四、真浏览器实测（1440x900 headless 非物理真机，不拦截写响应走真后端）

- DOM 审计：url=/oa4rust/app/admin/form-designer，标题命中，saveBtns=1，submitBtns=0，fold=0。
- 真后端假 ID 探针：PUT /api/form/R420-FAKE-FORM-<timestamp> = **200 type:error(form not found)**；POST /api/form/submit = **400 type:error(bad request: form not found)**（非 2xx，误报前提不成立）。
- 决定性对照：route 注入表单列表 1 行（id=假 ID），真 UI 点行加载 → 点保存，写响应全程不拦截走真后端：真 PUT /api/form/<假 ID> = 200 type:error(form not found)（newWrites 铁证），writesDelta=1，**仍弹蓝色 ℹ「保存成功」** —— P1 误报坐实。
- 非 2xx：注入 500 → 红 toast「✗保存失败: HTTP 500: Internal Server Error」；403 → 红 toast「✗保存失败: 没有权限执行该操作」均正确。
- 失败/空态：列表注入 500 → hasFail=true/retryBtns=1（表单列表加载失败+重试）；空 data → emptyHint=true（暂无表单）/items=0，可区分。
- 注入对照（仅佐证代码不读 type）：type:error/success:true、type:success/success:false、type:success/success:true 三种响应均弹 ℹ保存成功。
- 0 pageerror，totalWrites=8。
- 截图 sha256（1440x900 headless 非物理真机）：r420-decisive-save.png=0979f244d1ae8a11ad895dd7388c253a8d8b512274564e12a4b35af27296b8b6；r420-500.png=48837f1de4e4e01652ed1c0100b0cb46247c771a897b886ba3b6f64cd7f638b2；r420-403.png=b0d807f1710941a29f5db7be067031418d5a98dea9e850d59e1b5409a7e64966；r420-empty-state.png=1c43a10642520e0ee9ec26877aa4b67d74ef78f19b54487d1d5cd07c39db7e1b；r420-error-state.png=98057318f2cd5fd50b4621ac24c1c74fd846c1cd1f2872c97d845c66bd01cb39。三张关键图（决定性保存/500/空态）已目视确认。
- 取证说明：决定性对照与注入 stub 因保存后页面状态与 toast 相同，部分截图字节一致；请求真实发出以 evidence.json writes（含真响应体）为准，非注入伪造。

## 五、三分判定

- **1 个用户可达 P1 真缺陷**：saveForm :1111 更新分支 await api.put 不接响应 + :1119 裸 toast.info("保存成功") 不读 type；后端 form_u2_update 200+type:error 下仍显示成功语义 info。三全满足：真实 @click（模板 :15）+ 真请求响应体（newWrites 200 error）+ 源码定位（:1111/:1119）。
- 通过项：假 ID 探针两端点信封据实 / 决定性对照走真后端 / 500·403 正确 error / 失败与空态可区分 / 无门控真实可达 / 0 pageerror。
- 不适用：confirm/prompt 取消零写（saveForm 无守卫）。
- 死代码：submitPreview（DOM 提交按钮=0，无写请求）不判缺陷。

## 六、取证限制与产品语义差异

- 取证限制：破坏性真实写未执行，用假 ID（UPDATE 命中 0 行无副作用）+ route 注入替代；submitPreview 死代码未真 UI 触发（DOM=0）；创建分支真后端 default 桶恒 success 未单独坐实误报（本就不误报）；全量写按钮按代表性覆盖。
- 产品语义差异：ActionResult error HTTP200 为 o2server 兼容；form_submit 失败全为非 2xx（400）不产生 200+error；默认桶 create 恒 success 无误报前提。

## 七、最小修复建议（本轮不改码）

- 新增 formDesignerWriteSucceeded(resp)：以 type 为主（type==="error" 即失败），不强求 success 布尔。
- saveForm 更新分支 :1111 用 lastResp 承接 api.put 响应；:1119 仅在成功时 toast.info("保存成功")，失败 toast.error 透传 message 并 return，不写回 definition、不刷新。
- 创建分支保留现有 created?.data?.id 校验；submitPreview 死代码可在后续轮据实清理。