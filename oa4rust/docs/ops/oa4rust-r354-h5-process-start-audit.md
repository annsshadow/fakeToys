# oa4rust H5 发起流程页 start.vue 只读甄别审计（R354）

- 轮次：R354 / 任务 t164；角色 qa-e2e（缺陷审计与浏览器实测工程师）
- 对象：oa4rust/mobile/src/pages/process/start.vue（发起流程页）
- 性质：只读审计，不改源码 / 契约 / 后端 / SDK / 其它页
- 测试服：域名以占位符 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 代替，真值运行期从 .local.keys / ssh oa4rust-test 读取，未入库、未回显
- 取证环境：Playwright Chromium headless，390x844，isMobile + hasTouch，DPR 2
- 重要声明：本轮截图均为 headless 模拟移动视口，**非物理真机**，不得冒充真机证据

## 1. 任务与验收标准

甄别发起流程页是否存在数据信封解析缺陷（参 R340 / R344 / R351 同型问题），并验证写端点闭环。五项验收：
1. startableProcesses 信封探针 + UI 流程选项一致性
2. getProcess / getForm 解包与动态表单渲染
3. startWork → saveWorkData 写闭环（发起测试流程，断言 id 非空 + 跳转 + 回显）
4. 附件 upload 解包
5. 0 pageerror

## 2. 覆盖甄别

docs/ops 全库 grep startWork / startableProcesses / process/start 命中 7 处：R301 h5-skeleton.md:9、R316 deep-audit.md:56、R325 process-audit.md:18/20/32/38、R300 readable-process-titles.md:8。
- R325 P6 记发起入口直连正常（designer/list/all 200 + 12 流程渲染），但 fab 点击受无头 tabbar 遮挡属取证限制，未走完整发起写闭环。
- 结论：start.vue 发起链（选流程 → 填表 → startWork → saveWorkData → 回显）从未专项甄别，本轮为有效增量。

## 3. 源码梳理

### 3.1 请求层（oa4rust/mobile/src/services/http.ts）
- parseEnvelope（:67-77）：value = data；若 string 则 JSON.parse（失败抛 ApiError malformed JSON response）；直接 return value as ApiResponse。坐实：HTTP 层只做字符串解析，**不解包顶层 data**。
- performRequest（:186-192）：2xx 走 resolve(parseEnvelope(data,status))；discardResponse 或 204 resolve({success:true,data:undefined})。
- performUpload（:223-249）：upload 2xx 直接 resolve(parsed as ApiResponse)，不解包。

### 3.2 服务层（oa4rust/mobile/src/services/index.ts）
- :24-26 list() 仅类型断言不解包。
- :110 startableProcesses = list(mapi.get(/api/processplatform/assemble/designer/list/all))
- :113 getProcess = mapi.get(designer/get/{id})
- :116 getForm = mapi.get(/api/form/{formFlag})
- :118 startWork = mapi.post(processing/work, {process,title})
- :121 saveWorkData = mapi.put(data/work/{workId}, values, {discardResponse:true})
- :230-234 fileApi.upload = mapi.upload(/api/attachment/upload/folder/{folderId}, ...)

### 3.3 页面层（start.vue）
- 导入 loadList（:19，来自 @/utils/loadList）。
- loadProcesses（:54-70）：loadList(() => startableProcesses(), {map:(payload)=>{ rows = Array.isArray(payload) ? payload : (payload as {data?:unknown[]})?.data ?? [] ; return rows.map(...).filter(p=>p.id) }})（:61 双层兼容）。
- pickProcess（:75-85）：p.id === selectedProcessId 则 return（幂等）。
- loadBoundForm（:87-121）：getProcess（:92）→ definition = detail.data?.processDefinition ?? detail.data（:94）→ begin.form 或 manualList[0].form 得 formFlag；无则 showForm=false（:101-105）；有则 getForm（:107）→ parseMobileForm(formResp.data)（:109，定义于 utils/form.ts:213）。
- submit（:134-171）：校验 selectedProcessId + title.trim（:136-138，否则 submitTip=请选择流程并填写标题）；validateSimpleForm（:141）；startWork（:150）→ workId = String(created?.data?.id ?? 空)（:151）；有 fields 则 saveWorkData（:161）；成功 showToast 流程已发起 + setStorageSync(oa_process_active_tab, started) + switchTab(/pages/process/process)（:163-165）；catch submitTip=发起失败（:167）。
- 模板：proc-item data-testid=h5-start-process-item（:290）内 proc-name（:293）；标题 input v-model=title placeholder=工作标题（必填）（:299-303）；表单 card v-if=showForm（:312）内字段 input/textarea（:326-334）；submit button class=submit ai-brand-fill（:403）。

## 4. 真浏览器 E2E 实测（四轮，证据 gitignored）

脚本与证据：test-results/r354-h5-process-start/ 下 evidence.json + evidence-v2/v3/v4.json，截图 start-initial / start-pick-0..3 / start-submitted / start-v2/v3/v4-submitted.png（均 390x844 headless，非物理真机）。

### 4.1 首轮（S0 探针 + 只读）
- designer/list/all：status 200；原始体 dataIsArray=false，innerKeys=[count,data,page,size]，count=12，innerArrayKey=data，n=12，样本 6 条真实流程。
- UI：itemCount=12，names 12 条真实流程名（S3 流程 x10 + seed-approval-flow + Test Flow），hasEmpty=false，hasLoadErr=false。
- 判定：原始 HTTP 是双层信封 data:{count,data:[...]}，但 loadList 内联 map 已解包，UI 12 行正确 —— **双层信封非缺陷**（与 R340/R351 未解包缺陷不同型）。

### 4.2 v2（遍历全部流程 + upload）
- S1：遍历 12 流程，10 个 S3 流程 fieldCount=0（begin.form 为空，showForm=false，属产品设计）；seed-approval-flow fieldCount=3，渲染 备注（textarea）/ 请假天数（number）/ 申请原因（text）动态表单 —— 动态表单取证限制补齐。
- S3：who 200 unique=admin；attachment/upload/folder/admin status 200，dataIsObject=true，id/name 在 resp.data 中正确解包 —— PASS。
- S2 未成功：titleVal 空、fieldsFilled=0，uni-app DOM 选择器不匹配（见 6 取证限制）。

### 4.3 v3（真实 input 定位）
- inputDiag：真实 DOM 4 个 input/textarea，class=uni-input-input / uni-textarea-textarea，placeholder=null（uni-app 把 placeholder 渲染为覆盖层 div）。
- 标题填入第一个 input → readTitle=R354E2E... 成功，hasSubmitTip=false（标题校验已过）；提交触发校验 申请原因 为必填项 / 请先修正表单校验错误 —— 证明确实执行 submit，workResp 空系必填字段未填。

### 4.4 v4（evaluate 填全字段，写闭环坐实）
- 用 page.evaluate 对 form card 内 input/textarea 直接设值 + dispatchEvent(input/change)：filled=[TEXTAREA R354备注, INPUT number 7, INPUT text R354申请原因]，readFields 一致。
- startWork：POST /oa4rust-api/api/processplatform/service/processing/work status 200，body.data={id:605e294d-fb98-466d-af9d-03c64df0e030, taskId:94f52fde-...}，type=success —— id 非空 PASS。
- saveWorkData：PUT /oa4rust-api/api/processplatform/service/processing/data/work/605e294d-fb98-466d-af9d-03c64df0e030 status 200，body.data={bundle:id, scope:work, value:{comment:R354备注, days:7, reason:R354申请原因}}，type=success —— 表单值回写 PASS。
- 跳转：URL 变为 #/pages/process/process，新流程 R354E2E1795326456801 置顶可见 —— 回显 PASS。
- 四轮合计 0 pageerror。

## 5. 三分判定

### 5.1 真缺陷：0 个
五项验收全部 PASS：双层信封被 loadList 内联解包（UI 12 行正确）、getProcess 解包正确且绑定表单流程动态渲染 3 字段、startWork → saveWorkData 写闭环 200 + 跳转 + 置顶回显、upload 解包正确、0 pageerror。未发现 R340 / R344 / R351 同型未解包缺陷。

### 5.2 取证限制
- 10 个 S3 流程 begin.form 为空，动态表单仅在 seed-approval-flow（fieldCount=3）覆盖；其余流程无表单可填属后端流程定义，非前端缺陷。
- 破坏性 / 越权写未授权执行：仅走测试流程发起，未触碰审批流转 / 撤回 / 删除等端点。

### 5.3 产品语义差异
- begin.form 为空时 showForm=false 属设计（无绑定表单的流程无需填报表单），不是渲染缺失。
- parseEnvelope 只做字符串解析、upload 不解包均为既有分层约定；本页各自在 loadList map / resp.data.id 处正确解包，无需改动。

## 6. 结论与最小修复方向

无可修复真缺陷，不改码。uni-app H5 的 input placeholder 为覆盖层（真实元素 placeholder=null），自动化测试须按 class=uni-input-input / uni-textarea-textarea 定位并以 evaluate + dispatchEvent 填值；此为测试脚本适配要点，非产品缺陷。

## 7. 敏感自检

文档仅含 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位符，无真域名 / IP / 口令 / 私钥 / 个人路径；真值运行期读取未入库。截图来源均如实标注为 headless 非物理真机。
