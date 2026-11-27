# R604b 验证型甄别：脚本补全 word 未定义（P1 确凿）

- 标识: oa4rust-r604b-script-completion-audit / 轮次 R604b / 任务 t174
- 类型: 验证型甄别（零源码改动，唯一脱敏审计文档）
- 父提交: a088b980d
- 结论: **P1 确凿** —— XScript 脚本编辑器补全函数引用未声明的 word，运行即抛 ReferenceError；且该唯一补全源经 override 顶替内置补全，导致补全 100% 失效。运行态真实可达，非安全问题、非越权。

## 一、立项与范围

台账未见对该编辑器补全的专项甄别。本次只甄别、不改任何业务源码；唯一入库产出为本脱敏文档。

## 二、机制锚点（逐行核实）

- 前端 oa4rust/frontend/apps/desktop/src/components/ScriptCodeEditor.vue:46 `function xscriptCompletion(_context: CompletionContext) {`，:47 `return { from: word.from, options: XSCRIPT_COMPLETIONS, validFor: /^[\w$]*$/ }` —— word 在全文件 110 行内无任何声明（const/let/var 均无），亦无 `context.matchBefore` 调用，属自由变量。
- 同文件 :29-44 `XSCRIPT_COMPLETIONS` 数组 14 项（this/workContext/effectivePerson/organization/resources/applications/console/JSON/Promise/include/print/getValue/setValue/require），补全数据本身完整。
- 同文件 :50 `autocompletion({ override: [xscriptCompletion] })` —— override 仅列该坏源，顶替 CodeMirror 内置补全且无兜底，故补全 100% 失效。
- 兄弟正确写法 oa4rust/frontend/apps/desktop/src/components/SqlEditor.vue `createCompletionSource()`:93 `const word = context.matchBefore(/\w+/)`，:94 `if (!word) return null`，:145 `return { from: word.from, ... }` —— 对照坐实 ScriptCodeEditor 漏声明 word。
- 消费链 ScriptWorkbench.vue:41 `<ScriptCodeEditor v-model=form.code />`；三视图 PortalScriptDesignerApp.vue:11 / CmsScriptDesignerApp.vue:12 / ProcessScriptDesignerApp.vue:9；路由 main.ts:339/369/393 真实可达。

## 三、方法（真测试，非静态推断）

- 夹具: oa4rust/tests/r604b_probe.test.ts（临时，运行后已删除）。
- 日志: /tmp/r604b-probe.log，文件 sha256 `907ede2a9e25f68a655d554cd3a8c5c81ef821dca854c77cd47a7ea8520256a4`。
- 命令: `cd oa4rust && npx vitest run tests/r604b_probe.test.ts`，EXIT 0，5 passed / 0 failed。
- P1-1 从 Vue 源码正则抽 xscriptCompletion 函数体，new Function 真执行：调用抛 `ReferenceError: word is not defined`，坐实运行态必崩。
- P1-2 源码守卫 override 数组正则匹配，去空白后仅 `xscriptCompletion`，坐实无内置兜底。
- P1-3 源码守卫 ScriptCodeEditor 无 `const/let/var word`、无 `context.matchBefore`，坐实漏声明。
- guard 兄弟 SqlEditor 含 `const word = context.matchBefore(/\w+/)` 与 `from: word.from`，对照通过。
- guard 消费链 ScriptWorkbench.vue 含 `<ScriptCodeEditor`，坐实组件被真实挂载。

## 四、typecheck 门为何未阻断（据实披露）

- desktop 包 typecheck 脚本为 `tsc --noEmit`（非 `vue-tsc`）；tsc 不解析 .vue 单文件组件的 `script setup` 内容，故 `word` 未声明不会被 TS2304 捕获。
- 队长实测 `pnpm --filter @oa4rust/desktop typecheck` EXIT=0、无 TS2304，缺陷存活于 HEAD，非误报。
- 结论：该缺陷仅能通过运行态或 vue-tsc 暴露；当前 CI 门存在盲区，建议后续纳入 .vue 类型检查（不在本任务写域，仅记录）。

## 五、结论与建议（不在本任务写域，仅记录）

- 影响：脚本编辑器补全 100% 失效；用户触发补全即抛运行异常（可能被 CodeMirror 内部吞掉表现为补全无响应），三视图与路由真实可达。
- 修复方向（供后续实施）：在 xscriptCompletion 内补 `const word = _context.matchBefore(/[\w$]+/)`，`if (!word) return null`，并以 `word.from` 替换裸引用（或对齐 SqlEditor 写法）。
- 三分裁决：运行态功能 100% 失效、真实可达、无越权/无敏感数据泄露 → 判 P1（高优先功能缺陷），非 P0 安全问题。

## 六、证据 sha 表

- runnerSha（紧凑 JSON，`sha256(JSON.stringify(meta))`）：`db0d214a2ac8225ac9df93f3af4de584fd88cd763c4c56635715f6940c4403f6`
- 探针日志 sha256：`907ede2a9e25f68a655d554cd3a8c5c81ef821dca854c77cd47a7ea8520256a4`（临时，非入库交付物）。
- 文档不含自身 sha（避免自引用悖论）；脱敏：无数据库/服务器地址、无口令、无个人路径，端口占位 `<TEST_HOST>:<PORT>`（本任务未连库，仅占位声明），0 真值入库。
- SKIP/未覆盖：未跑浏览器 GUI 真页面（本环境无可用 GUI 会话；以 Vue 函数体真执行 + 源码守卫 + 兄弟对照三层真测试坐实核心判定，据实标注）。
