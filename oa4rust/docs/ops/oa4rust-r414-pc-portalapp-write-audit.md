# R414 PC 门户管理（PortalApp）写链 200+type:error 误报成功甄别报告

## 一、审计范围与取证口径
- 目标：PC 门户管理主页面 PortalApp.vue（路由 path=portal、name=PortalApp、meta.title=门户管理），与 R407 的 PortalPageDesignerApp.vue、R412 的 PortalDesigner.vue 均为不同页面，未重复审计。
- 只读甄别、不改码；三分判定（0 缺陷 / P1 / P2）。
- 取证环境：Playwright Chromium headless，视口 1440x900，对已部署测试服实测；<YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 为占位，非物理真机，截图均为真浏览器实测渲染。
- 三全判据：真 UI 可复现 + 真请求响应 + 源码定位；决定性对照全程不拦截写响应、直接走真后端，排除注入伪造。
- 破坏性真写（建/改/删真实门户数据）未执行，统一用假 ID + 真后端探针替代（UPDATE/DELETE 命中 0 行无副作用）。
- 无 EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 门控（grep 全空、DOM fold=0），4 组 details 端点按钮展开后真实可达。

## 二、源码甄别（逐函数区分真实可达 / 只读 / 死代码）
- designerCreate(:321-335)：模板:38/41/44/47/50 真实 @click；prompt 取名后 :325-330 await api.post 建门户/页面/组件/模板页/脚本，:331 裸 toast.success(`${kind} 已创建`)，不读响应 type，catch(:332) 仅非 2xx。
- designerUpdate(:336-353)：模板:39/42/45 真实 @click；prompt 取 ID 后 :341/:346/:348 await api.put，:349 裸 toast.success(`${kind} 已更新`)，不读 type。
- designerDelete(:354-370)：模板:40/43/46/48/49/51 真实 @click；prompt(:355)+confirmMsg(:357) 双守卫，:359-365 await api.delete，:366 裸 toast.success(`${kind} 已删除`)，不读 type。
- designerMisc(:372-404)：模板:57-67 真实 @click，含 portalIcon/portalPerm/pageSave/search/inputCompare/inputCover/inputCreate/inputPrepCover/inputPrepCreate/summaryV2/scriptManager 共 11 分支写端点，:400 裸 toast.success(门户设计器操作已提交)，不读 type。
- surfaceOps(:406-454)：模板:15-20/:26-27 真实 @click，含 create/publish/dictSet/dictDel/scriptByName 等写端点，:450 裸 toast.success(门户表面操作已提交)——队长备料遗漏，列为同型隐患。
- loadPortalTwin(:298-318)/loadPortalTwin2(:499-517)：模板:31/:32 真实 @click，各含 3-4 条写端点，但每个请求被 s() 吞错、:314/:513 统一 toast.success(已提交)——同型隐患（提交语义且吞错，未单独坐实）。
- 剔除（只读/正确模式）：loadPortalSurface/List/Resources/Detail/SurfaceEntities/MobileFacets/Deep 全部为 GET 只读探测；deletePage(:495-497) 走 deleteMutation(:480-487) useMutation+onError，失败弹红 toast 不弹成功，正确模式；designerSearch 端点返回 200 success 只读不误报。
- SDK frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 不查 type；仅非 2xx 抛错，200+type:error 不进 catch。

## 三、后端铁证（HTTP200+type:error 确认存在）
- shared/response.rs:188 ActionResult::error 返回 HTTP200（o2server 兼容既有设计），:149 ActionResult 结构 type/success 为权威字段。
- portal_assemble_designer/src/lib.rs：update_portal(:3407 portal not found)/delete_portal(:3360 portal not found)/update_portal_icon(:3439 portal not found)/portal_id_permission_post(:3468 portal not found)/input_compare(:1601 input not found)/input_cover(:1630 input not found)/page_save/page_delete 均 UPDATE/DELETE 命中 0 行→Ok(Json(ActionResult::error)) HTTP200。
- 路由 lib.rs:724-750（portal create/summary/delete/update/icon/permission）、:792-810（input 系列）、:784（designer/search）均注册可达。
- 成功响应为 ActionResult::success(带 id+updated/deleted/saved:true)；失败无权威 success=true，前端应以 type/error 判定。

## 四、真浏览器实测（决定性对照，写响应不拦截走真后端）
- 假 ID 探针（9 端点）：portalUpdate/portalDelete/portalIcon/portalPerm=200 error(portal not found)；pageSave=200 error(page not found)；widgetUpdate=200 error(widget not found)；inputCompare/inputCover=200 error(input not found)；designerSearch=200 success(只读)。
- P1-1 designerUpdate：真 UI 点改门户→prompt 填假 ID→真 PUT /api/portal/assemble/designer/portal/<fake>，newWrites 捕获真响应 200+type:error(portal not found)，writesDelta=1，仍弹绿 toast ✓portal 已更新。截图 r414-decisive-update.png。
- P1-2 designerDelete：真 UI 点删门户→prompt+confirm 双守卫→真 DELETE portal/<fake>，newWrites 捕获 200+type:error(portal not found)，writesDelta=1，仍弹绿 toast ✓portal 已删除。截图 r414-decisive-delete-designer.png。
- P1-3 designerMisc-inputCompare：真 UI 点输入比对→真 PUT /designer/input/compare，newWrites 捕获 200+type:error(input not found)，writesDelta=1，仍弹绿 toast ✓门户设计器操作已提交。截图 r414-decisive-misc.png。
- 通过项：prompt 取消 writesDelta=0 零写；注入 500→红 toast ✗更新portal失败: HTTP 500: Internal Server Error；403→✗更新portal失败: 没有权限执行该操作；空态 ✓暂无门户页面；pageErrors=[]；totalWrites=14；截图均 1440x900 非物理真机且已目视确认。
- 截图 sha256：r414-decisive-update.png=c8d25877cdb47af278be770f9c115e6914d75d2a17fb0165af9c144b9dfe3e26；r414-decisive-delete-designer.png=8c703f1ae5fda0c4f482f621317e36d4e40e1f36c9db2aeb662b5ab133a0a16a；r414-decisive-misc.png=69c46aa8573053a5790e3c4661cc47ca67518332adc9983ad33f05c0e9c7ea4d；r414-500.png=d60d149af3f0ab7e1fdbbae3496c68894b1f771525e158a59c3f0935f5f4105b；r414-403.png=ea6d54b647fceae590d8e8196d3c8dc0ba5412858e8c62be256bf427fdc8dc95；r414-empty-state.png=a341179a720bc35d7eed1f934e0688711120a8ddfa30e0acb0da6f6eb88dffb7。

## 五、三分判定：3 个用户可达 P1 真缺陷（三全满足）
- P1-1 designerUpdate 更新失败误报成功（:341/:346/:348 await 后 :349 裸 toast.success）。
- P1-2 designerDelete 删除失败误报成功（:359-365 await 后 :366 裸 toast.success，双守卫不覆盖业务失败）。
- P1-3 designerMisc 多写端点失败误报成功（:374-398 await 后 :400 裸 toast.success，inputCompare 已决定性对照坐实）。
- 同型隐患（L1，未逐一决定性对照，据实记录）：designerCreate(:331)、surfaceOps(:450)、loadPortalTwin(:314)/Twin2(:513)；create_portal 查重返回 200 error(portal already exists) 亦可能误报。
- 不判缺陷：deletePage/deleteMutation 正确模式、所有只读 load 探测、designerSearch 返回 200 success。

## 六、取证限制与产品语义差异
- 取证限制：破坏性真写未执行用假 ID 替代；失败态注入 list 500 后「门户页面加载失败」文案未出现（hasFail=false、retryBtns=0），失败态未坐实据实记录，空态可区分已坐实；designerCreate/surfaceOps/Twin/Twin2 同型隐患未逐一真 UI 决定性对照；全量写按钮按代表性覆盖。
- 产品语义差异：ActionResult::error HTTP200 为 o2server 兼容既有设计，前端须以 type/success 判成败而非仅 HTTP 状态；designerSearch 为只读返回 200 success 不误报。

## 七、最小修复建议（本轮不改码）
- 新增 portalAppWriteSucceeded(resp: any)：type 为主不强求 bool，识别 type=error 为失败。
- designerUpdate/designerDelete/designerMisc 三处 await 接住响应，失败 toast.error 透传 message 并 return，不弹成功；成功才 toast.success。
- 建议同步覆盖 designerCreate(:331)、surfaceOps(:450) 同型分支；loadPortalTwin/Twin2 的吞错+成功语义另行评估。
