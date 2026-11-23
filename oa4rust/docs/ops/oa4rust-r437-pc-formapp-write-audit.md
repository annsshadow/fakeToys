# R437 PC 表单管理 FormApp.vue 写链静默吞错甄别记录

> 环境：Playwright Chromium headless 1440x900 部署测试服真浏览器实测，**非物理真机**；截图取自 headless 页面，如实标注来源，不冒充真机。
> 性质：只读甄别（kind=work，不改任何代码），三分判定；真父 HEAD=a00b8f7521fd664ea2c9efdfb81868bda383380c。
> 脱敏：本文不含真实域名/IP/账号，统一用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；真域名仅留 gitignored 证据，不回显不入库。

## 一、结论摘要

判 2 个用户可达 P1 真缺陷（均三全满足），同为「静默吞 200+type:error」维度，但表现不同：
- 缺陷1 saveForm 更新分支（FormApp.vue:311-335，:324 PUT）：真后端 HTTP200+type:error(form not found) 下不读 type，:328 关 modal + :329 loadList 静默刷新。用户看到弹窗关闭、列表无变化，推断保存成功，实际未落库。
- 缺陷2 deleteForm（FormApp.vue:337-345，:340 DELETE）：confirm 确认后真后端 HTTP200+type:error(form not found) 下不读 type，:341 乐观 filter 移除行。用户看到表单行消失（甚至进入空态），实际未删除。
新建分支 :326 POST 与批量导入 :390 POST 后端恒 type:success（失败仅 400 非2xx，SDK 抛错→catch），判 0 缺陷、登记 L1 隐患（不读 type/业务布尔）。
readonly 剔除：loadList/loadV2/viewDetail/previewForm/useTemplate/restoreVersion/compareVersion/doExport。门控 grep 全空、DOM fold=0，写按钮生产可见可达。

## 二、审计范围与方法

- 目标页：frontend/apps/desktop/src/views/FormApp.vue（500 行）；路由 main.ts:188-190 path=form、:498 redirect /app/admin/form，实际访问 /app/admin/form。
- 触发原因：含 toast.success/info 的 48 个 .vue 与审计文档引用的 86 个 .vue 名求差集得 6 页，逐一核实后端后 4 页为只读/恒 success，FormApp 为唯一强候选。注意与已修的 FormDesigner（R421，另一页面）不同。
- 三全判定：真 UI 可复现 + 真请求响应体（page.on(response) 抓真写 body）+ 源码定位。决定性对照写响应全程不拦截走真后端，排除 route 注入伪造；route 注入仅用于非2xx/失败空态对照，明确区分。
- 破坏性真写（真更新/真删）不执行：用假 form id（U2Gate::NotFound → 0 行 UPDATE/DELETE，无副作用）+ 列表注入替代，据实披露。

## 三、源码甄别（逐函数真实可达/只读/死代码）

- 模板真实 @click：:31 编辑 editForm、:33 删除 deleteForm(f)（confirmMsg 守卫）、:112 保存/更新 saveForm、:146 导入 doImport；均为生产可见按钮，无门控。
- import { confirmMsg, toast }（:160）已正确导入；无 R430 那种漏 import 不可达问题。
- saveForm（:311-335）：:314 在途守卫 saving、:315-318 空名校验、:322 payload={name,definition:schema}；:323-324 有 id 走 api.put('/api/form/{id}',payload)，:326 无 id 走 api.post('/api/form',payload)；两分支 await 均不接响应；:328 showCreate=false 关 modal、:329 loadList() 刷新；catch（:330-331）仅非2xx toast.error。更新分支构成静默吞错 P1。
- deleteForm（:337-345）：:338 confirmMsg 守卫；:340 await api.delete('/api/form/{id}') 不接响应；:341 items.filter 乐观移除行；catch（:342-343）仅非2xx。构成静默吞错 P1。
- doImport（:378-405）：:390 await api.post('/api/form',f) 逐条导入，catch 计 fail、否则 ok++；:396-399 汇总文案。后端 create 恒 success（失败仅400），非静默吞错维度，L1。
- 只读剔除：loadList（:254-266 GET list/all，失败 describeListError 显式报错并保留旧表单）、loadV2（:268-281）、viewDetail（:283-292 GET detail）、previewForm（:304-305 toast.info 仅展示）、useTemplate（:347-356 本地填充表单）、restoreVersion/compareVersion（:358-362 toast.info 展示）、doExport（本地下载）。

## 四、后端失败信封与 SDK 判定（精确源码行）

- form_u2_update（backend/crates/cms_assemble_control/src/lib.rs:9737-9768）：:9746 u2_check_owner；:9747 U2Gate::NotFound → Ok(Json(ActionResult::error("form not found")))＝HTTP200+type:error；:9748 Forbidden → 403；Allowed 分支 :9753-9765 UPDATE 成功才 success。路由 routes.rs:556 PUT /api/form/{id}。
- form_u2_delete（lib.rs:9772-9787）：:9777 u2_check_owner；:9778 U2Gate::NotFound → 200+error(form not found)；:9779 Forbidden → 403；Allowed 分支 :9781 soft_delete + success。路由 routes.rs:557 DELETE /api/form/{id}。
- form_u2_create（lib.rs:9603-9651）：:9615 definition 缺失 → AppError::BadRequest 400 非2xx；:9629 非 default 桶且应用不存在 → 200+error(application not found)，但前端 payload 不带 appId（走 default 桶 :9618-9619 跳过校验），该分支不可达；:9650 正常恒 success。路由 routes.rs:554 POST /api/form。
- shared ActionResult error 为 HTTP200（o2server 兼容）。SDK frontend/packages/sdk/src/api.ts：parseJsonEnvelope（:82-88）仅 JSON.parse 不查 type；request（:275-296）仅非2xx 抛错（401 refresh、403 PermissionError、其余 httpStatusError），2xx 原样返回；put/delete（:307-309 等）不查 type。故 200+type:error 走 await 不抛错，前端不读 type 即静默吞错。

## 五、真后端假数据探针与 E2E 决定性对照

- 真后端假数据探针（job bash-a892db162a8a4ef1，exit0，证据 gitignored probe.json）：updateFakeId=HTTP200 type:error message=form not found；deleteFakeId=200 error form not found；updateNoSession=401 error unauthorized；createNoDefinition=400 error bad request: definition required；listAll=200（data=null，真列表由 UI  reads，据实记录）。假 ID 触发 U2Gate::NotFound，0 行 UPDATE/DELETE 无副作用。
- PC E2E（job bash-bc48bc203ac349a7，exit0）：R437_DONE totalWrites=4 pageErrors=0 fatal=none；1440x900 headless 非物理真机。
- 决定性对照 S1 更新：注入列表 1 行 R437测试表单 id=R437-FAKE-* → 真点编辑 → modal 填名 → 真点更新 → 真 PUT /api/form/{假id} 不拦截走真后端，page.on(response) 捕获 newWrites:1 条 body={type:error,message:form not found,prompt=ExceptionEntityNotExist}，modalBefore=1/modalAfter=0（modal 静默关闭），无成功 toast，列表行仍在——静默吞错坐实（缺陷1）。
- 决定性对照 S2 删除：同注入行 → 真点删除 → confirm 确定 → 真 DELETE /api/form/{假id} 不拦截走真后端，newWrites:1 条 200 body=type:error/form not found，overlay=1，rowsAfter=0（行从视图移除、显示暂无表单空态），无错误 toast——乐观删行坐实（缺陷2）。
- 非2xx：500 → 红 toast「✗保存失败: : HTTP 500: Internal Server Error」且 modal 保留（modalAfter=1）；403 → 红 toast「✗保存失败: : 没有权限执行该操作」且 modalAfter=1，均正确（catch）。
- confirm 取消：overlay=1、writesDelta=0 零写。
- 失败/空态可区分：errorState hasFail=true/retryBtns=1（表单列表加载失败+重试）；emptyState emptyHint=true（空态提示）、hasFail=false/retryBtns=0，两者可区分。
- DOM：url=/oa4rust/app/admin/form，title 命中，editBtns=1/delBtns=1/rows=1/tabs=1，fold=0。

## 六、三分判定与缺陷清单

- 缺陷1（P1，用户可达，三全满足）：saveForm 更新分支（FormApp.vue:324 PUT）不读 type，真后端 200+error 下 :328 关 modal + :329 刷新，视觉成功但 actual 未保存。最小修复：新增 formWriteSucceeded（以 type 为主、不强求 bool 字段），PUT/POST 两分支 const lastResp 接响应守卫，失败 toast.error 透传 message 且不关 modal 不刷新不写本地状态，成功才关 modal + loadList。
- 缺陷2（P1，用户可达，三全满足）：deleteForm（FormApp.vue:340 DELETE）不读 type，真后端 200+error 下 :341 乐观 filter 移除行。最小修复：const lastResp 接响应守卫，失败 toast.error 透传 message 并 return（不清 selected、不移除行），成功才 filter 移除行；confirm 守卫与 catch 非2xx 保留。
- L1 隐患（不判缺陷）：saveForm 新建分支（:326 POST）与 doImport（:390 POST）不读响应 type；后端 create 失败仅 400 非2xx（SDK 抛错→catch），无 200+type:error 失败形状，按口径判 0 缺陷；建议未来统一接 formWriteSucceeded 守卫。
- 正确模式：loadList/loadV2 失败 describeListError 显式报错并保留旧列表；confirm 取消零写；非2xx catch 红 toast。
- 产品语义差异：ActionResult error 为 HTTP200（o2server 兼容），非 4xx/5xx。
- 取证限制：破坏性真更新/真删未执行，用假 form id（0 行 UPDATE/DELETE）+ 列表注入替代，据实披露；全量按钮按代表性覆盖（两写链决定性对照）。

## 七、证据清单（截图 sha256 为命令行原始值，1440x900 headless 非物理真机）

- r437-decisive-update.png = 057dc2d180fac629be9c0b60b1b0d76a4403716b5bb7ebb510dd519169d27717（已目视：modal 关闭、注入行仍在、无错误提示）。
- r437-decisive-delete.png = e8db2658e4c797e34f96395886a416ba32b2e40f5d50d6da4b55ca92e18a8a60（已目视：行消失、显示暂无表单空态）。
- r437-500.png = 0113cb4c5da7559cba15d1cc6d813884ea9f8ddcd64eac42d8c1e6a42eb18bd6（红 toast + modal 保留）。
- r437-403.png = 2a3b9dcd7a814c16674a66a9e0d4166d29cce70a06184e7e11c9e1fecefdf204（红 toast + modal 保留）。
- r437-cancel.png = 2811abe56076f3847cda76c6c9ecc6828a79a19e6945c7b68b6a1aec5168afc5（confirm 取消零写画面）。
- r437-dom-audit.png = 2811abe56076f3847cda76c6c9ecc6828a79a19e6945c7b68b6a1aec5168afc5（与 cancel 同场景画面）。
- r437-error-state.png = 0262c0b40ef5545d794995da4aa2c262a9d15c35276822dc7a1fa30b3e00111e（加载失败+重试）。
- r437-empty-state.png = f40cc2643d7bd17286f126796a3ada64984ba25aa0ed7976cda25fdf922e1560（空态提示）。
- 证据落 gitignored 目录 test-results/r437-pc-formapp-write/（probe.json、evidence.json、截图），真域名不回显不入库。
