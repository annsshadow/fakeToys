# R429 PC 个人中心签名保存写链甄别记录（静默 CRUD 吞错 / 后端 200+error 反查）

> 甄别任务 t270；只读审计，不改码；三分判定。真父 HEAD=f285453ed830e728804accb429f8ba6c89db51f3。
> 测试环境：部署测试服 + Playwright Chromium headless，视口 1440x900，**非物理真机**；真域名仅存 gitignored 证据，本文一律占位。

## 一、结论摘要

**0 个用户可达真缺陷（误报前提不成立）**，不改码。
- 首选目标 Personal.vue `saveSignature` 的 `.then` 确实不读响应 type（L1 隐患），但其后端 `/api/person/signature/save` handler 全函数恒 `ActionResult::success`，**不存在 HTTP200+type:error / success:false 失败信封**；失败仅非 2xx（401/DB 内部错误），SDK 会抛错走 catch 红 toast。
- 真浏览器决定性对照：真 UI 填签名提交、写请求全程走真后端不拦截，真 POST = 200 type:success，弹绿「签名已保存」，语义一致，无误报。
- route 注入 200+type:error 仍弹绿成功 toast，仅佐证前端不读 type 的 L1 隐患；真后端不产生该形状，**不判缺陷**。

## 二、甄别范围与源码定位

- 文件：frontend/apps/desktop/src/views/Personal.vue。
- `saveSignature()`（:227-232）：:229 `api.post('/api/person/signature/save',{signature,mimeType:'image/png'})`；:230 `.then(()=>toast.success('签名已保存'))` **不读响应 type**；:231 `.catch(()=>toast.error('保存失败'))` 仅非 2xx。
- 真实可达：模板 :57 `<form @submit.prevent=saveSignature>`，:60 提交按钮；E2E DOM `sigSection=1 / textareas=1 / saveBtns=1`。门控 grep（EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose）全空，`fold=0`，无生产门控。
- 剔除（正确模式）：`uploadSignatureImage()`（:208-225）用 `await api.upload(...)` + `try/catch`，失败 `toast.error`；非 `.then` 静默写法，不判。各页 load*/open* 只读展示 toast 剔除。
- 全库系统性反查：frontend/apps/desktop/src 匹配 `\.then\(\(\)\s*=>\s*toast\.(success|info)` 仅命中 Personal.vue:230 一处；useMutation 页 onError 守卫齐备，正确模式剔除。

## 三、后端失败信封核实（精确 grep，glob0 不可信）

- 真实 handler：`backend/crates/personal_extend/src/personal.rs:177-266 save_signature`。路由 `backend/crates/personal_extend/src/routes.rs:46-47` POST `/api/person/signature/save`。
- 失败路径**均为非 2xx**：:183-188 token/session 校验失败 `AppError::Unauthorized`（401）；:208/:218/:229/:240/:249/:257 DB 错误 `AppError::internal_with_source`（非 2xx）。
- :191-200 `signature` / `mimeType` 用 `unwrap_or_default` / `unwrap_or('image/png')`，空签名与缺字段**不报错**。
- :202-231 SELECT 后 UPDATE 或 INSERT（0 行也走正常分支）；:260-265 全函数唯一出口 `Ok(Json(ActionResult::success({id, saved:true})))`。
- **全函数 grep `ActionResult::error` = 0**，无 200+type:error 失败形状，误报前提不成立。
- legacy stub 非真实 handler：`mcp_server/src/generated_routes.rs:20225-20228`、`openapi/src/lib.rs:32935/32945` 仅工具描述/空签名占位，实际由 personal_extend 路由接管。
- SDK：`frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope` 仅 JSON.parse 不查 type；:287-296 仅 403/非 2xx 抛错，2xx 原样返回；:303-304 post。故 200+type:error 会走 .then（但真后端无此形状）。

## 四、真后端假数据探针与决定性对照（1440x900 headless 非物理真机）

- 真后端假数据探针（证据 gitignored `probe.json`，apiMasked=`<HOST>/oa4rust-api`）：
  - 空签名 authed → HTTP200 `type:success`（`{id,saved:true}`）；
  - 假签名 authed → HTTP200 `type:success`；
  - 无会话 → HTTP401 `type:error message:unauthorized`。均坐实无 200+error 失败形状。破坏性真写未执行，用假签名字符串替代，据实披露。
- 决定性对照（真 UI 填 `R429-E2E-TEST-SIGNATURE-*` → 点「保存签名」，写请求不拦截全程走真后端，`page.on(response)` 捕获响应体）：真 POST `/api/person/signature/save` = HTTP200，body `{"data":{"id":...,"saved":true},"type":"success"}`，`writesDelta=1`，toast 绿「✓签名已保存」——**语义一致，无误报**。
- 注入对照（仅佐证 L1）：route 注入 200 `{"type":"error","message":"signature save denied","success":false}`，仍弹绿「✓签名已保存」；真后端不产生该形状，不判缺陷。
- 关键截图 sha256（命令行原始值，1440x900 headless 非物理真机，已目视确认）：
  - r429-decisive-real.png = e7af214ae807497b1199d6f4fd5a09e8944fa5c59017c5a242fbda77b139eced（真后端 200 success + 绿「✓签名已保存」）；
  - r429-inject-typeerror.png = 9943f7c38c08d7f02cd8e2e44d91b4cb2ae3c1c9b59a840a0eed9c5cda5b540e（注入 200 error 仍绿，L1 佐证）；
  - r429-500.png = 6257ed5e192c1c18c223091d4b0075e676568a4dae9ad95d919d61081afaf0ab（红「✗保存失败」）；
  - r429-403.png = 85466d1c846c8f2f98f674f85d0e750a9a8dd1a5403cbd3258d8aaf629c87fb3；
  - r429-dom-audit.png = ca3ee2b4f54cafed89369014b3afe6748936a3633b8aeaf81d05b5449c63da14。

## 五、非 2xx / 失败空态 / pageerror

- 非 2xx 正确：注入 500（`{"type":"error","message":"Internal Server Error"}`）→ 红「✗保存失败」；403（`{"type":"error","message":"Forbidden"}`）→ 红「✗保存失败」；writesDelta 各 1。
- 真后端无会话探针 401 `unauthorized`，SDK 抛错走 catch，符合预期。
- 0 pageerror（E2E `pageErrorCount=0`），`totalWrites=4`。
- 失败/空态：本写链为 upsert 即时反馈，无独立加载失败列表态；签名表单始终可见可编辑，成功/失败 toast 可区分，据实记录。

## 六、三分判定

- **0 个用户可达真缺陷（P1=0）**：saveSignature 的 200+error 误报前提不成立（后端恒 success，失败仅非 2xx）；决定性对照真后端 200 success 成功 toast 一致。
- **L1 隐患**：saveSignature :230 不读响应 type，若后端未来改 200+error 口径会误报（注入对照佐证）。当前真后端无此形状，不判缺陷、不改码。
- **正确模式剔除**：uploadSignatureImage await+try/catch；非 2xx 红 toast；useMutation onError 守卫页。

## 七、取证限制与最小修复建议

- 取证限制：破坏性真写未执行，用假签名字符串 + route 注入替代；注入变体仅佐证 L1 不判缺陷；失败/空态按本链无独立列表态据实记录；全量按钮按代表性覆盖。
- 产品语义差异：ActionResult success HTTP200 为 o2server 兼容；save_signature 空值不校验、upsert 幂等（id 稳定），属既有业务语义。
- 最小修复建议（仅后备，本轮不改）：若后续后端新增 200+error 失败口径，新增 personalSignatureWriteSucceeded（type 为主不强求 bool），saveSignature().then 接响应守卫，失败 toast.error 透传 message 不弹成功，成功才 toast.success 签名已保存。

