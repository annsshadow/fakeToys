# R435 PC 服务调用设计器写链甄别记录（ServiceInvokeDesignerApp.vue）

> 环境：Playwright Chromium headless 1440x900 部署测试服真浏览器实测，**非物理真机**；截图均为 headless 页面截图，如实标注来源，不冒充真机。
> 性质：只读甄别（kind=work，不改任何代码），三分判定；真父 HEAD=03d3e0426a96d6427f265efc7a7b44bd1886c2d0。
> 域名/凭据脱敏：本文不含真实域名/IP/账号，统一用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；真域名仅留 gitignored 证据不回显。

## 一、结论摘要

判 1 个用户可达 P1 真缺陷（三全满足）：runInvoke 执行写链在真后端 HTTP200+type:error（invoke not found）下仍无条件弹绿 toast「服务调用已执行」。
另 2 条写链 saveItem/deleteItem 后端恒 type:success（0 行 UPDATE 仅把 saved/deleted 业务布尔置 false，无 200+type:error 失败信封），前端 onSuccess 不读业务布尔直接关 modal/刷新——按本轮口径判 0 缺陷，登记 L1 隐患；confirm 取消零写已坐实。
loadData/list 只读剔除；门控 grep（EndpointFold/devToolsEnabled/import.meta.env.DEV/defineExpose）全空，DOM 实测 fold=0，写按钮生产可见可达。

## 二、审计范围与方法

- 目标页：frontend/apps/desktop/src/views/ServiceInvokeDesignerApp.vue（232 行）；路由 main.ts:410-414 path=service-invoke-designer，实际访问 /app/admin/service-invoke-designer。
- 触发原因：审计文档引用集合 85 个 .vue 与 41 个含 toast.success 页面差集锁定 4 页，本页为从未被任何审计文档引用的最强候选。
- 三全判定：真 UI 可复现 + 真请求响应体（page.on(response) 抓真写 body）+ 源码定位；决定性对照写响应全程不拦截走真后端，排除 route 注入伪造。
- 破坏性真写（真执行/真删）不执行：用假 flag/假 id + 列表注入替代（假 id 触发 0 行 UPDATE，无副作用），据实披露。
- 其余 3 缺口页（QueryStatDesignerApp/TemplateApp/ProcessApplicationApp）各 1 处 toast.success 为只读展示类（统计详情/分类文案/暂无分类），据实剔除，不在本任务写域。

## 三、源码甄别（逐函数真实可达/只读）

- 模板真实 @click：:28 编辑 editItem、:29 执行 runInvoke(item)、:30 删除 deleteItem(item)（confirmMsg 守卫）、:44 保存 saveItem；均为生产可见按钮，无门控。
- runInvoke（:186-188）经 runM（:174-185）：mutationFn 取 flag=item.alias||item.name||item.id（:176），api.post('/api/program_center/invoke/{flag}/execute',{})（:177）；onSuccess（:179-181）无条件 toast.success('服务调用已执行') 不读响应 type；onError（:182-184）仅非 2xx toast.error。→ P1。
- saveItem（:153-157）经 saveM（:136-152）：POST invoke/save/{id} 或 invoke/create（:144-145）；onSuccess（:147-150）只 invalidateQueries + closeModal，不读 type/data.saved；onError（:137）仅非 2xx。→ 恒 success 形状，L1。
- deleteItem（:166-171）经 delM（:159-165）：confirmMsg 守卫（:168）；POST invoke/delete/{id}（:161）；onSuccess（:162-164）只 invalidateQueries，不读 type/data.deleted；onError（:160）仅非 2xx。→ 恒 success 形状，L1。
- 只读剔除：loadData/reload、useQuery list（:91-102）、filtered 计算属性、editItem/openCreate/closeModal、liveQueryItems；均非写链。
- import { confirmMsg, toast }（:54）已正确导入，无 R430 那种漏 import 不可达问题。

## 四、后端失败信封与 SDK 判定（精确源码行）

- 执行 handler u3_invoke_execute_inner：backend/crates/program_center/src/lib.rs:11528-11578；:11534-11535 flag 查无 → Ok(ActionResult::error('invoke not found'))；:11544-11545 row 无 → error；:11550-11563 token 错误 → error('token required'/'invalid token')；成功 :11575-11577 success。均为 HTTP200 + type:error（o2server 兼容，shared/response.rs:188）。路由 program_center/src/routes.rs:632 POST /api/program_center/invoke/{flag}/execute。
- 保存 handler invoke_save：lib.rs:11495-11509，require_admin + shared::crud_save，恒 Ok(ActionResult::success({id,saved:bool}))，无 error 分支；路由 routes.rs:472-473（PUT/POST）。
- 删除 handler invoke_delete：lib.rs:11513-11526，require_admin + shared::crud_delete，恒 success({id,deleted:bool})，无 error 分支；路由 routes.rs:474-475（DELETE/POST）。
- shared/crud.rs:83-138 crud_save、:141-158 crud_delete：0 行 UPDATE/DELETE → Ok(false)，被上游包成 saved:false/deleted:false 但 type 仍 success，不构成 200+type:error。
- SDK：frontend/packages/sdk/src/api.ts parseJsonEnvelope（:82-88）仅 JSON.parse 不查 type；request（:275-290）仅非 2xx 抛错（403→PermissionError、其余 httpStatusError），2xx 原样返回；post（:303-304）不查 type。故 200+type:error 走 mutation onSuccess 不抛错 → 误报。

## 五、真后端假数据探针与 E2E 决定性对照

- 真后端假数据探针（job bash-6228f3b85bf148e1，exit0）：executeFakeFlag=HTTP200 type:error message=invoke not found；executeNoSession=401 error unauthorized；saveFakeId=200 type:success saved:false；deleteFakeId=200 type:success deleted:false；listAll=200 success。0 行 UPDATE 无副作用。
- PC E2E（job bash-8134761c827b4161，exit0）：R435_DONE totalWrites=5 pageErrors=0 fatal=none；1440x900 headless 非物理真机。
- 决定性对照 S1（execute）：注入列表 1 行 R435测试调用 id=R435-FAKE-* → 真点执行 → 真 POST /api/program_center/invoke/{fake}/execute 不拦截走真后端，page.on(response) 捕获 newWrites:1 条，body=type:error/message:invoke not found/prompt=ExceptionEntityNotExist，仍弹绿 toast「✓服务调用已执行」=P1 三全坐实。
- S2 save：真 POST save/{fake}=200 type:success saved:false，modalAfter=0（弹窗关闭），无成功 toast；业务布尔 saved:false 被静默吞。
- S3 delete：真 POST delete/{fake}=200 type:success deleted:false，无成功 toast；列表刷新后行仍在（用户可见真实状态）。confirm 取消：overlay=1，writesDelta=0 零写。
- 非 2xx：500→红 toast「✗执行失败：HTTP 500: Internal Server Error」；403→红 toast「✗执行失败：没有权限执行该操作」均正确（onError）。
- DOM：url=/oa4rust/app/admin/service-invoke-designer，title 命中，executeBtns=1/editBtns=1/deleteBtns=1/createBtns=1，fold=0，rows=1。
- 失败/空态：emptyHint=true（暂无服务调用定义可见），但 errorState hasFail=false/retryBtns=0（列表注入 500 未生效），据实列取证限制；P1 判定不依赖此项。

## 六、三分判定与缺陷清单

- P1（1 个，用户可达，三全满足）：runM.onSuccess（ServiceInvokeDesignerApp.vue:179-181）在真后端 200+type:error（invoke not found）下无条件 toast.success('服务调用已执行')，不读 type。最小修复：新增 serviceInvokeWriteSucceeded（type 为主不强求 bool），runM 接响应守卫，失败 toast.error 透传 message 且不弹成功，仅成功才 toast.success。
- L1 隐患（不判缺陷）：saveM/deleteM onSuccess 不读 data.saved/deleted 业务布尔；后端恒 type:success，0 行 UPDATE 时 saved:false/deleted:false 被静默吞。真后端不产生 200+type:error，按口径判 0 缺陷；建议未来接业务布尔守卫，失败保留 modal/行并提示。
- 正确模式：execute 失败非 2xx→SDK 抛错→onError 红 toast；delete confirm 取消零写（writesDelta=0）。
- 产品语义差异：ActionResult error 为 HTTP200（o2server 兼容）；crud 0 行命中以业务布尔 false 表达而非 type:error。
- 取证限制：破坏性真执行/真删未执行，用假 flag/id + 列表注入替代；失败态列表注入未生效据实记录；全量按钮按代表性覆盖。

## 七、证据清单（截图 sha256 为命令行原始值，1440x900 headless 非物理真机）

- r435-decisive-execute.png = 2fcffef5c117fdcc2c8ac18e488188db06df23352539ff40c75cf563b35c6495（已目视：绿 toast 服务调用已执行 + 注入行仍在）。
- r435-500.png = 981f34cd2511500d9ba0034bf81773e2555a031c55e3db829113e5ebd7660a5d（已目视：红 toast 执行失败 HTTP500）。
- r435-403.png = 9a1a36f74559cb1751b2e6a228d318180e1ecc0ca9c4a0f3c42e5e9b05da0097。
- r435-cancel.png = d297d31163f73201d3b02ea8f35c17203c2395fdb26c9ff1a86bb22c9993db7e（confirm 取消画面）。
- r435-decisive-save.png = d297d31163f73201d3b02ea8f35c17203c2395fdb26c9ff1a86bb22c9993db7e（与 cancel 同机无 toast 画面，响应体铁证在 evidence.writes）。
- r435-decisive-delete.png = dafef460faacbc76ba034edd2bc79cfcc0ed648eaf0151a44305e48d2b20d150。
- r435-dom-audit.png = dd161057acc987719aa963499d5f0eb6d9fe0319126b84d322e8b5b6f1dffd77。
- r435-empty-state.png / r435-error-state.png = c62411d21233145aa71b828070999cf0466ec3faa651c160d3f442756c8029f2（同 sha，注入未生效据实记录）。
- 证据落 gitignored 目录 test-results/r435-pc-serviceinvoke-write/（probe.json、evidence.json、截图），真域名不回显不入库。
