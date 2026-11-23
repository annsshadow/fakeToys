# R413 PC 门户设计器三写链失败信封误报成功 P1 修复

## 根因
PortalDesigner.vue 的 saveDesign/saveById/fileUpload 三处写链裸 await 后端响应后无条件 toast.success。后端写端点在业务失败时返回 HTTP 200 + type:error（如 design not found / file not found），SDK api.ts 仅对非 2xx 抛错，2xx 失败信封原样 resolve，导致失败被误报成功。

## 修复
新增 portalDesignerWriteSucceeded(resp:any):boolean：type 为主不强求 bool——type:error 或 success:false 判失败，type:success 或 type 缺省且 success 非 false 判成功，非对象/null/undefined 判失败。后端 save 成功返回 success({id,saved:true,content})，权威字段 saved 非统一 bool 名，严格 bool 校验会假阴性。
saveDesign(:355-366) 用 const lastResp 承接 api.put 响应并守卫，失败 toast.error 透传 message 并 return，成功才 toast.success(门户布局已保存)。saveById(pdDesignerOps 分支) 用局部 return 收口，失败不触发 (:424) 统一成功 toast；fileUpload 的异步 onchange 用 lastResp 守卫，失败不弹资源内容已上传。widgetDelete/tplCategory 后端恒 success，无误报前提，保持不动；只读 load*、端点/payload/method、模板 @click、后端、SDK 零改。

## 验证
- 契约 oa4rust/frontend/apps/desktop/src/views/PortalDesigner.writeEnvelope.contract.test.ts（23 it，真执行注入桩）：真父 1a6b07ce8 RED 10failed/13passed → 修复版 GREEN 23/23。
- 根全量 pnpm test：298 files / 2795 tests / 0 failed。
- desktop pnpm run typecheck：exit 0。
- PC 构建入口 index-BEI0_YJu.js，整目录原子部署到测试服 <YOUR_TEST_HOST>，浏览器经 <YOUR_TEST_DOMAIN> 访问。
- 真浏览器：Playwright Chromium headless 1440x900（非物理真机，viewport=1440x900）注入对照，13 断言全 true、0 pageerror；证据落 gitignored test-results/r413-pc-portaldesigner-write-fix/（evidence.json + 5 张 1440x900 截图）。关键图 01-typeerror.png（红 ✗design not found/file not found，无成功 toast）、02-success-false.png（红 ✗R413 denied）、03-true-success.png（绿 ✓门户布局已保存/门户设计器操作已提交）。

## 取证限制
浏览器为 headless 自动化，非物理真机人工操作；失败响应通过 Playwright route 注入后端同构 200+type:error 信封对照，真实 UI 点击流程、真实部署测试服、真实 toast DOM 均实跑并截图。
