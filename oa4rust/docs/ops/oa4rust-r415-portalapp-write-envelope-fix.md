# R415 PC 门户管理（PortalApp）三写链误报成功 P1 修复

## 一、根因与范围
- 页面：PC 门户管理 PortalApp.vue（路由 /app/admin/portal，meta.title=门户管理），与 PortalPageDesignerApp.vue、PortalDesigner.vue 为不同页面。
- 三写链 designerUpdate（更新门户/页面/组件）、designerDelete（删门户/页面/组件/模板页/字典/脚本）、designerMisc（11 个设计器工具写端点）在 await api.put/post/delete 后无条件 toast.success，不读响应信封。
- SDK 仅非 2xx 抛错；后端 ActionResult::error 走 HTTP200 + type:error（o2server 兼容既有设计），故失败不进 catch，误报成功。
- 真父/RED 基线：当前 HEAD e21a5589c（甄别提交 5e88d17e02 与台账提交只改文档、代码未变）。

## 二、修复
- 新增 portalAppWriteSucceeded(resp: any): boolean：非对象/null/undefined 失败；type=error 或 success=false 失败；type=success 或 type 缺省且 success 非 false 成功；未知 type 失败。不强求权威 bool（后端成功返回 id+updated/deleted/saved 等非统一 bool 名，严格 bool 会假阴性），对齐 R402/R404/R406/R408/R410/R413 先例。
- 三写链 try 内用 lastResp 承接 await 响应，成功提示前统一守卫；失败 toast.error 透传后端 message（缺省为操作未生效请刷新后重试）并 return，不弹成功；成功才保留原 toast.success。
- designerDelete 的 prompt(:355) + confirmMsg(:357) 双守卫保持不变；取消零写、无成功 toast。
- 保持零改：端点/payload/method/模板 @click/catch 非 2xx 文案/只读 load*/surfaceOps/loadPortalTwin/Twin2（同型隐患列 L1 另案）/deletePage（useMutation+onError 正确模式）/后端/SDK/其它页/migrations。

## 三、验证
- 契约 oa4rust/frontend/apps/desktop/src/views/PortalApp.writeEnvelope.contract.test.ts：真执行注入桩（api/toast/prompt/confirmMsg/helper），参数带 string 标注避免 TS7006；23 例。真父 e21a5589c RED=10failed/13passed（0 桩错误），修复版 GREEN=23/23。
- 根全量 vitest：299 files / 2818 tests / 0 failed。desktop tsc --noEmit：exit 0。
- 构建入口 index-CAY9qp3L.js，整目录 tar + sha256 校验原子部署到测试服 <YOUR_TEST_HOST>（域名占位，真值不入库）， DEPLOY_R415_PC_FULL_OK。
- 真浏览器：Playwright Chromium headless 视口 1440x900，对已部署测试服做注入对照，17 断言全 true、failed=[]、totalWrites=14、pageErrors=[]。注入 portal/<fake> 与 input/compare 的 type:error / success:false / HTTP500 响应，并设真 success 与删除取消对照。
- 关键对照：type:error 三条红 toast（✗portal not found ×2、✗input not found）无一成功 toast；success:false 三条红 ✗R415 denied；真 success 三条绿 ✓portal 已更新 / ✓portal 已删除 / ✓门户设计器操作已提交；删除取消 delWrites=0 且无 ✓portal 已删除；HTTP500 走 catch ✗更新portal失败。
- 证据（gitignored，脱敏）：test-results/r415-pc-portalapp-write-fix/ 下 evidence.json（source 明标 Playwright Chromium headless 1440x900 against deployed test server; NOT a physical device）与 6 张 1440x900 截图。3 关键图 sha256：01-typeerror.png=08270171fd83e6ebc85cb2977953c8b2b491ee48d42437ed7d2603dd1b36b029；03-true-success.png=0fb6e33833442eaae2502c7fe744f4b1bd57c644776a21ca69481c064c12cb79；05-delete-cancel.png=b8fd7adc60c25c01d16a2fcd82f487fd77f5d2f1b01b0239e3e226aeaaf4e0e5。3 图均已目视确认，非物理真机。

## 四、脱敏
- 测试域名/主机一律用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位；不含口令、私钥、真实 IP/域名/SSH 端口。证据与脚本均在 gitignored test-results/ 下，不入提交。
