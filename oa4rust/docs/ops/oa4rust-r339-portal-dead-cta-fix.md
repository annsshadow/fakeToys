# R339 门户管理死按钮修复（PortalApp 新建页面入口）

## 1. 范围与环境
- 分支：opt1000/oa4rust-pc-ai-refactor（禁止 main）。
- 改动文件：frontend/apps/desktop/src/views/PortalApp.vue；契约 PortalApp.newpage.contract.test.ts；本文档。
- 测试环境：测试服 Web（域名用 <YOUR_TEST_DOMAIN> 占位，主机 <YOUR_TEST_HOST> 占位，零真值入库）。

## 2. 根因与方案
依赖 t141 只读审计：PortalApp.vue 头部「+ 新建页面」按钮 @click="showEditor = true"，但全组件仅第 460 行 const showEditor = ref(false) 一处定义，模板无消费点；真浏览器点按 newPageModalCount=0 / newPageEndpointCalls=0，属 100% 死按钮（P1）。

队长裁决方案 A（fail-loud）：
- 移除头部误导性死 CTA 与 showEditor 死变量，不补弹窗。
- 为什么不补弹窗：CreatePageRequest.portalId 必填，现有 designerCreate('page') 缺 portalId，实测会返回 400；测试服门户列表空，无法端到端验证，硬造弹窗只会制造新的死流程。
- 硬边界：不改 designerCreate / designerUpdate 端点调用与参数，不改 AdminPageHeader 组件、后端、SDK、其他页面；下方「端点操作 · 设计器写」分组中的建页入口保持原样。

延伸建议（非本次范围）：若要恢复「新建页面」，需先补 portalId 来源（如让用户在门户下拉选择）与端到端测试，再单独评估。

## 3. 修复内容
- PortalApp.vue 头部 AdminPageHeader 由带 #actions 死 CTA 改为成对空标签（保留 </AdminPageHeader> 锚点，兼容既有 t4-admin-restyle 契约截取）。
- 删除 script 中 const showEditor = ref(false)。
- 端点分组按钮、写读链方法体一字未动。

## 4. RED -> GREEN
- 契约 PortalApp.newpage.contract.test.ts 5 it：头部无死 CTA/无 showEditor 绑定；源码无 showEditor；AdminPageHeader 标题副标题保持；端点四组与建页入口保持；写读链方法体保持。
- RED：2 failed / 3 passed（死 CTA 与 showEditor 断言失败）。
- GREEN：5/5 passed。
- 全量回归：vitest 272 files / 2186 tests 0 failed；typecheck exit 0。

## 5. 真浏览器验证
测试服整目录原子部署（DEPLOY_R339_PORTAL_FULL_OK，入口 index-BEJJYA4S.js）后，1440x900 Playwright Chromium headless 实测（非物理真机）：
- 页面正常加载：h1「门户管理」、.portal-view=1；头部死 CTA「新建页面」计数 0。
- 端点分组 details.endpoint-group 共 4 组，点击首个 summary 后 .endpoint-actions 可见；「建门户」按钮可见。
- 代表性安全写回归：surfaceOps('create') 弹窗输入 marker 后 POST /api/portal/assemble/surface/create 返回 200（写孝除建门户外未授权不执行）。
- 8 项断言全 true：pageLoaded / noDeadHeaderCta / endpointGroupsExpanded / groupCountFour / createBtnVisible / surfaceCreate200 / noPageErrors / noApiFailures（pageErrors=[]、apiFailures=[]）。
- 证据：gitignored test-results/r339-portal-dead-cta-fix/ 下 01-portal-no-dead-cta.png（sha256 f14cc433…）、02-endpoint-create-ok.png（sha256 b4e3027e…），均 1440x900；evidence.json source=Playwright Chromium headless against deployed test server; NOT physical device。

## 6. 取证限制
- 破坏性写与真实建页未授权不执行；测试服门户列表空属环境数据限制，非功能缺陷。
