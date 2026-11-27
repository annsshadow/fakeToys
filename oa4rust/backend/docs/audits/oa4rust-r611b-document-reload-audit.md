# R611b DocumentApp 刷新/重试按钮 reload 死绑定审计（验证型甄别）

- 结论：P2。DocumentApp.vue 工具栏「刷新」与错误态「重试」均绑定 @refresh/@retry="reload"，但 script setup 全程无 reload 定义；Vue 模板把未定义 handler 编译为 _ctx.reload=undefined，点击静默 no-op，无任何请求、无报错、无界面变化。属用户可感知的局部交互失效，不丢数据、不越权、不写库、不崩溃，用户可用搜索键/搜索按钮/页签手动触发 doSearch 绕过。
- 派发真父 HEAD：3adb2a97b（git rev-parse 核实）；实际工作树 HEAD：bc97a29d06a157df17bd73327c615fea051d7656（工作树前移，目标源码字节未受影响）。
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 临时路径，用后即删。

## 立项与范围
- 主对象 frontend/apps/desktop/src/views/DocumentApp.vue:12 / :27。
- 对照：frontend/apps/desktop/src/views/AppInfoApp.vue:14 / :27 / :97（同类工具栏正确绑定 doSearch）。
- 子组件事件源：frontend/apps/desktop/src/components/admin/AdminToolbar.vue:21 / :43-48；AdminTable.vue:12。
- 旧契约：frontend/apps/desktop/src/views/DocumentApp.slots.contract.test.ts:48（锁定 @refresh="reload" 死字符串，从不校验 handler 存在）。
- 路由可达：frontend/apps/desktop/src/main.ts:164-166 path:document 动态 import，:495 redirect /app/admin/document。
- 不扩范围：不修改任何源码/组件/契约/路由/测试配置；仅甄别与取证。

## 机制锚点（真父源码逐行）
- DocumentApp.vue:12 <AdminToolbar v-model="keyword" ... @search="doSearch" @refresh="reload">；:27 <AdminTable ... @retry="reload">。全文件 reload 仅模板两处；script setup 内 function/const/let/var/import 六种定义形式 grep 全部 0 命中。
- :208 async function doSearch() 为真实存在的搜索/加载函数；:235 watch(tab,()=>{void doSearch()}) 页签切换可重新加载。
- AdminToolbar.vue:21 <button class="admin-toolbar-btn" @click="emit('refresh')">刷新</button>；:43-48 defineEmits 含 update:modelValue/search/refresh/create。
- AdminTable.vue:12 <ListLoadError v-if="error" ... @retry="emit('retry')" />：错误态「重试」真实发射 retry。
- AppInfoApp.vue:14 @refresh="doSearch"、:27 @retry="doSearch"、:97 async function doSearch()：同一对子组件事件在同仓库另一视图正确绑定到已定义函数。
- DocumentApp.slots.contract.test.ts:48 expect(tpl).toContain('@refresh="reload"')，头部注明仅断言源码模板静态结构、不引入 @vue/test-utils，故死绑定长期存活。
- typecheck 门盲区：oa4rust/package.json typecheck=pnpm --recursive run typecheck，desktop 实跑 tsc --noEmit（非 vue-tsc），不解析 .vue script setup，TS2304 未捕获。

## 决定性实测（vitest + jsdom 真 Vue 挂载）
- 探针 tests/r611b_probe.test.ts（83 行，临时已删）：文件头 // @vitest-environment jsdom；命令 cd oa4rust && npx vitest run tests/r611b_probe.test.ts → EXIT 0，4 passed / 0 failed（vitest v5.0.5）。
- 用例 1（静态锚点）：DocumentApp 模板同时绑定 @refresh="reload" 与 @retry="reload"，script 无 reload 定义；兄弟 AppInfoApp 绑定 doSearch 且已定义。
- 用例 2（严格真执行）：new Function('"use strict"; reload()')() 抛 ReferenceError: reload is not defined，坐实自由变量解析失败。
- 用例 3（编译产物语义真挂载）：createApp + defineComponent，setup 返回 ()=>h('button',{onClick:ctx[handlerName]}) 模拟 _ctx.reload=undefined；btn.click() 后无异常、无 emitted、无 warn；对照 ctx.doSearch 被点击则 emitted=[doSearch]。证明 Vue 对未定义模板 handler 静默 no-op。
- 用例 4（事件源与旧契约）：AdminToolbar/AdminTable 真实 emit refresh/retry；旧契约第 48 行锁死 @refresh="reload" 字符串。
- 决定性日志（gitignored 临时路径，用后即删）：<PROBE_LOG_PATH>。

## 三分裁决（据实，不夸大不降级）
1. 判 P2：刷新与错误态重试两个常驻可点按钮完全无响应，用户可直接感知交互失效。
2. 不判 P1：不越权、不写库、不丢数据、不抛异常、不阻断主流程；搜索键/搜索按钮/页签切换均可手动触发 doSearch 绕过。
3. 不判 P3：非边缘文案，是工具栏与错误恢复的核心交互入口，影响面明确。
4. 非安全族：与身份盲 IDOR/DEV 门控/端点错配族无关；根因是模板引用未定义 handler，且 typecheck 门与静态契约均未覆盖。

## Fail Loud
- 未跑浏览器 GUI 真页面；以 jsdom 真 Vue 挂载 + 严格模式真执行 + 源码逐行锚点替代，未静默跳过。
- 用例 3 基于 Vue 模板对未定义 handler 的编译语义（_ctx.* 属性访问为 undefined 时静默 no-op）构造最小挂载，非真编译 SFC，但足以证明点击无副作用。
- 测试夹具与日志均在 gitignored 临时路径并已删除，未入库；未读取或回显任何密钥/口令/真实地址。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=e7fb2650e99c4678f8e5c9d7f227de55bf1e56b85173d21c7506548105b15bcd
- meta={"task":"t205","round":"R611b","kind":"verification-work","target":"apps/desktop/src/views/DocumentApp.vue refresh/retry dead reload binding","verdict":"P2","evidence":"vitest 4passed + jsdom Vue mount real click + strict eval ReferenceError + sibling doSearch contrast","parent":"3adb2a97b","head":"bc97a29d06a157df17bd73327c615fea051d7656","typecheck_gap":"tsc --noEmit does not parse .vue","browser_gui":"not run (jsdom instead)"}
- frontend/apps/desktop/src/views/DocumentApp.vue=3b6a9970c9f9cd7c3dc0c45273f1b7f5d93b766152405e85f091d8936c85a8aa
- frontend/apps/desktop/src/views/AppInfoApp.vue=b09e3c74ef69c850cdbff176398547623d58950110820804300a2a05c8c81b5d
- frontend/apps/desktop/src/components/admin/AdminToolbar.vue=aef5f8156d4c2a140a6961b9a0e14fdd91b9baec7d94f498da44b3f96880593d
- frontend/apps/desktop/src/components/admin/AdminTable.vue=35844c920b1436d1d941ef8d89bd06fb2cfac53b80eefd312a62d97bb84c7dd0
- frontend/apps/desktop/src/views/DocumentApp.slots.contract.test.ts=3f9ef8fd85a6bab63e16df2533cd8b01d0b56a9ff56cf8b66d5b10f789d0141f
- frontend/apps/desktop/src/main.ts=e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0

## 修复方向（不在本任务范围）
- 将 DocumentApp.vue:12 / :27 的 reload 改为 doSearch（或定义 reload 委托 doSearch）；补一条断言「模板绑定的 handler 在 setup 中确实定义」的契约测试；desktop typecheck 迁至 vue-tsc 以捕获 .vue 的 TS2304。
