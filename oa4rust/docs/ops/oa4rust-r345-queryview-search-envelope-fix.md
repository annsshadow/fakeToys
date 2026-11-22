# R345 PC 查询视图搜索信封 P1 修复（t151）

## 来源与范围
- 任务：t151 [R345] 修复 QueryViewApp 搜索信封 P1，Kind=implementation。
- 依据：t150 [R344] 只读深度审计判 P1（docs/ops/oa4rust-r344-queryview-audit.md §S1）：POST /api/queryview/search 返回分组信封 {data:{designList,key,viewList}}，QueryViewApp.vue doSearch(:177) `views.value = r.data ?? []` 直塞对象，模板 views.length=undefined、v-for 遍历对象键 → 恒 3 行垃圾、真实结果不渲染、无结果也无法走空态。
- 队长裁决方案 A（fail-loud）：只改 doSearch 解包与映射，对齐 SearchApp.vue:90-96 R315 范式但适配本页模板。
- 硬边界：不动端点调用/payload/searchRunner/listQuery/模板/六写端点/后端/SDK/其它页/DDL；冲突先回报队长。
- 凭据仅运行期从测试服受控文件读取（环境变量注入），不回显不入库；测试域名统一用 <YOUR_TEST_DOMAIN> 占位。

## 改动
1. `oa4rust/frontend/apps/desktop/src/views/QueryViewApp.vue`：doSearch 内把 `views.value = r.data ?? []` 替换为——
   `const envelope: any = r?.data ?? {}`；
   `const designList: any[] = Array.isArray(envelope.designList) ? envelope.designList : []`；
   `const viewList: any[] = Array.isArray(envelope.viewList) ? envelope.viewList : []`；
   `views.value = [...designList, ...viewList].map((x: any) => ({ id: x.id, flag: x.flag ?? x.id, name: x.name ?? x.viewName ?? x.title ?? '' }))`。
   含设计意图中文注释；其余代码零改动（见 git diff --stat 仅本方法）。
2. `oa4rust/frontend/apps/desktop/src/views/QueryViewApp.search.envelope.contract.test.ts`（新增）：从源码提取 doSearch 函数体，剥离 TS 类型注解后 new Function 注入 api/searchRun/ref 桩**真执行**，6 用例覆盖分组信封解包映射（id/flag/name 与 viewName/title 回退）、无结果空数组、坏数据容错、端点 payload 不变、模板未回退、无旧 bug 行；自定义 helper 参数均带 `: string` 标注。
3. 本文档（脱敏，真值不入库）。

## RED→GREEN 与门禁
- RED：契约落盘后先在原源码跑，4 failed|2 passed（直塞信封致 views 为对象、arrays 断言失败）。
- GREEN：方案 A 落地后 6/6 passed；契约 harness 修正两处（剥离标量 `: any` 注解；api.post 桩 `return resp.value` 使 r.data 即真实信封）。
- 全量 vitest：275 files / 2205 tests 全绿（cd oa4rust && pnpm exec vitest run）。
- desktop typecheck：pnpm --filter @oa4rust/desktop typecheck exit 0；pnpm build exit 0（QueryViewApp-C8B4Lc8u.js、index-DeDOlHPW.js）。

## 测试服真浏览器实测（来源标注）
- 部署：本地产物原子替换为测试服 web（备份 web.bak-r345-* / web.old-r345-*），PC_INDEX=assets/index-DeDOlHPW.js，服务 active，http 回环 200（https 回环 curl 000 仅本地 TLS 问题，真实浏览器不受影响）。
- 脚本与证据：test-results/r345-e2e.cjs + r345-e2e-run.sh（凭据运行期提取，域名从 gitignored 历史脚本注入），证据落 gitignored `test-results/r345-queryview/`：pc-queryview-before.png / pc-queryview-search-a.png / pc-queryview-empty.png / report.json。
- 来源标注：Playwright Chromium headless 1440x900 访问测试服真实页面，**非物理真机**。
- 真实流程：登录 → /app/admin/queryview → 填关键词 a 点「搜索」按钮 → 断言搜索响应 POST /api/queryview/search status=200、key=a、designList=3/viewList=0，UI 渲染 3 行真实视图（S3 查询 …）；再搜无结果关键词 → designList=0/viewList=0，UI rows=0、空态 1；0 pageerror。
- 取证教训：本页 input 的 Enter 会被全局搜索劫持到 /app/search（历史既有行为、不在 R345 scope），故实测用「搜索」按钮触发 doSearch；初次用错 AdminToolbar 输入框亦已通过截图复盘纠正。两项均如实记录，未改范围外代码。

## 事实边界
- ≥1000 轮持续迭代中的一环，非终态；本轮只修 QueryViewApp 搜索信封 P1，不声称其它页面或后端问题已解决。
