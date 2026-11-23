# R409 PC 流程设计器（ProcessDesigner）流程定义保存写链甄别报告

> 审计性质：只读甄别（不改业务代码）。测试环境：部署测试服 + Playwright Chromium headless，视口 1440x900，**非物理真机**。所有截图均为真浏览器渲染，非模拟图。

## 一、审计范围与结论

- 目标文件：frontend/apps/desktop/src/views/ProcessDesigner.vue；路由 main.ts:212-215 path=process-designer / name=ProcessDesigner / meta.title=流程设计器。
- 任务：甄别流程定义保存写链在后端 HTTP 200 + type:error 失败信封下是否误报成功。
- 结论：**1 个用户可达 P1 真缺陷（三全满足）**。saveProcess(:5857-5869) 用 toast.info 显示「保存成功」，虽为 info 提示类型，但文案是明确成功语义；后端 200 + type:error 时仍显示，构成与 toast.success 同型的失败误报。
- git 基线：3fb89137258a6aeb4c9028a08bef05846445015d，工作树干净。

## 二、源码甄别（逐函数）

- 门控检查：EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 全部 grep 命中 0；DOM 审计 fold=0，保存按钮真实可达。
- 真实 @click 写入口：模板:39 `<button class="btn btn-primary" @click="saveProcess" :disabled="!currentProcess">💾 保存</button>`；需先点流程列表行（模板:52-59 @click=loadProcess）使 currentProcess 非空。
- saveProcess(:5857-5869)：:5858 `if (!currentProcess.value?.id) return`；try 内 :5860 definition=currentO2Definition(currentProcess.value)，:5861-5863 `await api.post(/api/processplatform/assemble/designer/save/${id}, {processDefinition: definition})` 不接响应、不读 type；:5864 currentProcess.value.definition=definition；:5865 `toast.info("保存成功")` 无条件显示成功语义文案；catch(:5866-5867) 仅非 2xx 抛错才 toast.error。
- toast 实现（frontend/apps/desktop/src/utils/toast.ts:38/52-61/123-126）：info 类型用 ℹ 图标蓝色、role=status；success 用 ✓ 绿色。文案「保存成功」是确定性成功语义，info 仅改变视觉样式不改变语义，失败时仍显示属误报。
- 剔除（不判缺陷）：createProcess(:5838-5856) 有 `const id=created?.data?.id; if(!id) throw` 校验，且走 create→save 两步，非本写链；loadDesignerFileApp(:5886-5916) 只读 GET 探测，:5912 toast.success 是文件定义展示文案（真只读）；loadProcesses(:5870-5882)/loadProcess(:5777-5816) 只读；大量画布/节点/规则/版本/模板等纯本地状态与 UI 操作（如模板:977 saveRules 保存规则）非流程定义保存写链。
- SDK：frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 不查 type，仅非 2xx 抛错；故 200 + type:error 被原样 resolve，saveProcess 无感知。

## 三、后端信封铁证

- backend/crates/processplatform_assemble_designer/src/lib.rs save_flow(:303-347)：:310-313 admin 门禁（无会话 401 / 非 admin 403）；:325-331 `UPDATE x_process_definition SET process_definition=$1, update_time=NOW() WHERE id=$2`；:333-334 `if result==0 { return Ok(Json(ActionResult::error("process definition not found"))) }` 即 HTTP 200 + type:error；成功时 :337-346 ActionResult::success({id, saved:true, updatedAt})。
- routes.rs:22 `.route("/api/processplatform/assemble/designer/save/{id}", post(save_flow))`。
- ActionResult::error 走 HTTP 200 是 o2server 兼容的既有设计（与同型批次一致），UPDATE 0 行无副作用，故假 ID 探针安全。

## 四、真浏览器实测

- 脚本（gitignored）：test-results/r409-pc-processdesigner-write.cjs；证据同目录 evidence.json 与截图。1440x900 headless 非物理真机，pageErrors=[]，totalWrites=4。
- S0 DOM：按钮 34 个，页面标题「流程设计器」命中，btn-primary 保存按钮 1 个，fold=0。
- S1 真后端假 ID 探针：POST /api/processplatform/assemble/designer/save/{fake} = 200 + type:error（"process definition not found"，prompt=ExceptionEntityNotExist），坐实 UPDATE 0 行的失败信封。
- S2 决定性对照（注入流程列表 1 行 R409测试流程 / id=R409-INJ-PROC，先点流程行使 currentProcess 非空，再真 UI 点「💾 保存」，写响应全程不拦截走真后端）：真 POST /api/processplatform/assemble/designer/save/R409-INJ-PROC = 200 + type:error（newWrites 捕获真响应体），writesDelta=1，界面仍显示蓝色 ℹ「保存成功」，误报坐实。截图 r409-decisive-save.png（sha256 d7e9addb471453e0457164458f487da79f6386abd117dffcde80e485724a0cba）。
- 说明：注入列表行走的是假 ID，loadProcess 的 GET /get/{fake} 走真后端可能失败并落入其 catch fallback（:5812 以 p 构造 currentProcess），但保存按钮的写请求是真打假 ID、真后端返回 200+error，排除注入伪造质疑。
- S3 非 2xx：500 → 红 toast「✗保存失败: HTTP 500: Internal Server Error」（截图 r409-500.png，sha256 89c58e061b19bf9e54ba1cb49973afffb54cc94d2b97e5bc3579a46ac574f106）；403 → 红 toast「✗保存失败: 没有权限执行该操作」。二者均正确。
- S4 失败/空态（全新 context 规避缓存）：列表 500 → hasFail=true、retryBtns=1、文案「流程列表加载失败: HTTP 500: Internal Server Error」（截图 r409-error-state.png，sha256 26c93d62b98fe9f875db82eabe9a7edf82eaaa6702af59c758cd8553e2ab0928）；空 data → emptyHint=true「暂无流程」、retryBtns=0，二者可区分。
- 0 pageerror。本写链无 confirm/prompt 守卫（保存按钮直接触发），故「取消零写」不适用，据实说明。

## 五、三分判定

- **真 P1（1 个）**：saveProcess(:5861-5865) 在真后端 200 + type:error（process definition not found）下，:5865 无条件 toast.info("保存成功") 显示成功语义文案且 :5864 把 definition 写回 currentProcess（本地状态被当作已保存）。三全满足——真 UI 可复现（点💾保存）、真请求响应（newWrites 200+error 铁证）、源码定位（:5865 不读 type）。
- **通过项**：假 ID 探针 200+error 坐实；决定性对照写响应不拦截走真后端；500/403 正确 toast.error；列表失败态与空态可区分；无 EndpointFold 门控真实可达；createProcess 有 id 校验、loadDesignerFileApp 只读展示均正确剔除；0 pageerror。
- **不适用项**：confirm/prompt 取消零写（本写链无确认守卫）。

## 六、取证限制

- 破坏性真写（真保存有效流程定义）未授权未执行，用假 ID + 不拦截对照替代；假 ID 走 UPDATE 0 行无副作用。
- 全量按钮按代表性覆盖（保存写链单点深挖）；画布/节点/规则/版本/模板等纯本地状态操作未逐一触发，但它们不产生本写链写请求，不影响结论。
- 注入列表行使用假 ID，loadProcess 的 GET 走真后端可能失败并 fallback，据实说明；保存写请求本身为真打真后端，证据充分。

## 七、最小修复建议与产品语义差异

- 最小修复（对齐 R402/R404/R406/R408 范式）：新增 processDesignerWriteSucceeded(resp:any) 以 type 为主不强求 bool；saveProcess 用 lastResp 承接 :5861 的 api.post 响应，仅成功时才 :5864 写回 definition 并 :5865 提示成功，失败时 toast.error 透传 message 且不写回、不提示成功。toast.info 的建议可保留其视觉样式但文案须改为成功语义专属，或失败改走 error 提示。
- 产品语义差异：ActionResult::error 走 HTTP 200 是 o2server 兼容既有设计；save_flow 成功响应权威字段为 saved:true（非统一 bool 名），故 helper 不强求特定 bool 字段。
- 测试环境占位：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>（真值不入库）；本报告只含相对路径与 API 路径，无本机绝对路径、无密钥口令。