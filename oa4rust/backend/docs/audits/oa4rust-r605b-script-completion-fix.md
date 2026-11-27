# R605b ScriptCodeEditor 补全 word 未定义真修复

- 结论：真 bug 已修复，RED→GREEN 坐实。ScriptCodeEditor 的 xscriptCompletion 补全源引用未声明的 word，任何补全触发即抛 ReferenceError: word is not defined。
- 真父 HEAD：fd64d27c46343373899cfed33f6d142ab2b5f361（git 核实 HEAD 未前移，等于真父；真父..HEAD 对锚点源码零 diff）。
- 写域精确 3 文件：M ScriptCodeEditor.vue（仅函数体）、A 补全契约测试、A 本文档。其余零改。

## 立项与机制锚点
- 缺陷锚点 ScriptCodeEditor.vue:46-48：function xscriptCompletion(_context: CompletionContext) 直接 return 含 word.from，而 word 在函数内与模块内均未声明；参数名 _context 亦unused。
- 触发面：同文件 :53 autocompletion({ override: [xscriptCompletion] })，编辑器内输入即调用该补全源，缺陷为真实可达的运行时崩溃而非静态噪声。
- 设计意图铁证（兄弟正确写法）：SqlEditor.vue:91-94 createCompletionSource 内 const word = context.matchBefore(/\w+/)、if (!word) return null；:144-148 return { from: word.from, options, validFor: /^\w*$/ }。
- 补全项 XSCRIPT_COMPLETIONS 共 14 项（:29-44：this/workContext/effectivePerson/organization/resources/applications/console/JSON/Promise/include/print/getValue/setValue/require），本次零改。

## 修复（仅函数体，diff 1 file changed, 3 insertions(+), 1 deletion(-)）
- 参数 _context 改回 context。
- 新增 const word = context.matchBefore(/[\w$]*/)。
- 新增 if (!word) return null。
- return 中 word.from 现取自 matchBefore 结果；options=XSCRIPT_COMPLETIONS、validFor=/^[\w$]*$/ 原样保留；extensions 其余/挂载/主题/watch 零改。

## RED → GREEN（队长可独立 replay）
| 版本 | 契约结果 | 关键行 |
|---|---|---|
| 真父（stash 源码回退） | 2 failed / 2 passed | ReferenceError: word is not defined |
| 修复态 | 4 passed / 0 failed | 有匹配返回 from/options/validFor，无匹配 return null |
- replay 步骤：git stash push -- ScriptCodeEditor.vue → 真父版契约 RED → git stash pop 恢复 → 恢复后 sha 与修复态一致 dfc2c97d06a559e76c0c0b6693a62a83ee6f112eec76b9feb624c4787348ee49。
- 契约从 Vue 源码 readFileSync + sliceBlock 抽 XSCRIPT_COMPLETIONS 数组与函数体，Function 注入真执行：14 项完整且顺序正确、有匹配返回正确结构、无匹配 null、全程无 ReferenceError。

## 三 verify（全绿）
- V1：新补全契约 vitest 4 passed；ScriptWorkbench 相关（scriptWorkbenchConfirm + ScriptWorkbench.detailRace）6 passed。
- V2：pnpm --filter @oa4rust/desktop typecheck → tsc --noEmit EXIT0，无 TS2304 / TS2339。
- V3：biome check 两改动文件 EXIT0；全量 vitest 2237 passed（278 files）。
- 全量 vitest 另有 2 个 suite 加载失败，均为既有基线、非本次引入、两文件 git status 干净未改：tabStateA11y.contract.test.mjs 无 test suite；TaskCompleted.R314.contract.test.ts 需 backend cwd 否则源码路径 ENOENT。

## 六维守卫（零外溢）
- XSCRIPT_COMPLETIONS 14 项零改；extensions 其余基本设置/keymap/只读/可编辑/换行/updateListener/theme 零改；onMounted/watch/onBeforeUnmount 零改。
- SqlEditor.vue、ScriptWorkbench.vue、三视图（src/views）、package.json、backend crates、迁移、docs/ops/current-state.md 全部零改（git status 核实）。
- ScriptCodeEditor.vue diff 仅 1 file changed, 3 insertions(+), 1 deletion(-)。

## Fail Loud
- desktop 的 package.json 无 test 脚本，pnpm --filter @oa4rust/desktop test 为 EXIT0 空操作；故以 vitest run 全量作为真实测试证据，据实披露。
- 契约首版 typecheck 报 TS2339（闭包内赋值致 result 被收窄为 never），已改为直接调用 + as Result 修正，真父版 RED 仍有效。
- biome 首检报 import 排序/格式/字符串拼接告警，经 --write --unsafe 仅格式化修复后复验 EXIT0；源码与契约逻辑未变。
- 工作树含其他队友 r605a 后端改动（attendance_assemble_control lib.rs、r605a 测试），本次完全未触碰；精确 git add 仅 3 写域文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=9e838347a255b1b70b9a9c2037cf1a88787a3cabd002a82af26801b5cf821375
- src/components/ScriptCodeEditor.vue=dfc2c97d06a559e76c0c0b6693a62a83ee6f112eec76b9feb624c4787348ee49
- src/components/ScriptCodeEditor.completion.contract.test.ts=0041947855d5dc747707ad23a98acf571418cfd69b44e856ff5c2fc02bfdfd8f
- src/components/SqlEditor.vue（对照零改）=10a4e2780b0539dcc6f42b8c4d8b4b1bc1bc90ae257f4731ede93f3002c2790b
- src/components/ScriptWorkbench.vue（对照零改）=9e27b5b40d3b74c17eb3c3a919378987d113ccb94cfb4f1fc92bee260de38525

## 修复方向
- 已完工：补全源走标准 context.matchBefore 取值，无匹配返回 null，与 SqlEditor 兄弟写法一致；后续可补 e2e 覆盖真实键盘触发补全不崩。