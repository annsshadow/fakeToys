# R627c ProcessDesigner 三处死绑定/模板误用甄别审计

## 结论
验证型甄别，零源码改。真测试（vitest+jsdom+Vue 运行时 compile 探针）坐实四疑点。三分据实分项裁定：
- subRedo 死绑定(:1043)：L1（子流程面板无置真点、不可达，同 R606 先例，推翻侦察员 P1 判定）。
- groupNodes.value.length(:583)：P3（统计面板分组数运行时报错，局部显示类，可绕过）。
- histIdx.value<=0(:641)：P3（单步后退禁用态失效，内部 guard 仍在，局部）。
- stepBackward 读 history[histIdx].config(:7069)：P3（可达运行时错误，可经主流程 undo/redo 绕过）。
整体最高 P3；无 P0/P1/P2。排除 P0/P1：无安全越权、无写库、主流程不中断、涉事面板不可达或可绕过。

## 源码逐行证据（真父 02e1cca3b，ProcessDesigner.vue）
- :1043 子流程工具栏 button @click='subRedo'；全文件 subRedo 仅此 1 处，无 function/const 定义。兄弟 subUndo 定义于 :5660、模板 :1042 可点。
- :3881 const subprocessEditing=ref(false)；唯一置真语句 subprocessEditing.value=true 位于 :5568 openSubprocess 内。
- :5558 function openSubprocess(nodeIdx) 全文件仅定义 1 处、零调用点；:8709 enterSubprocessBreadcrumb 亦仅定义零调用，且只 push 栈不置 subprocessEditing。模板无任何 @click/@dblclick 指向 openSubprocess。故子流程面板(:1038 v-if subprocessEditing&&processDef)不可达。
- :583 {{ groupNodes.value.length }}；:4942 const groupNodes=computed(()=>getGroupNodes())。<script setup>(:2780) 顶层 ref/computed 模板自动解包，groupNodes 已是数组，.value 为 undefined。兄弟裸用 :169/:1164 v-for in groupNodes。
- :641 :disabled='histIdx.value<=0'；:3511 const histIdx=ref(-1)。模板解包后 histIdx.value 为 undefined，undefined<=0 恒 false，按钮永不禁用。兄弟 :12 用 !canUndo、:3512 canUndo computed 在 script 内用裸 histIdx.value（正确）。
- :4087 pushHistory 存 JSON.parse(JSON.stringify(processDef.value)) 整份（无 config 字段）；:7069 stepBackward 读 history.value[histIdx.value].config，取到 undefined。

## 路由与台账
- main.ts:212-215 path process-designer / name ProcessDesigner / component import，路由可达（:511 重定向）。页面可达不等于缺陷面板可达。
- docs/ops/current-state.md:5638/5640（R614 登记）明确列此三处为未甄别线索；R606(:5623) 已裁定：无置真点不可达的死绑定族判 L1、resetRules 局部失效判 P2。本次 R627 首次专项甄别。git grep 三锚点于 docs 仅线索行。

## 真测试证据
vitest v5.0.5 + jsdom + Vue 运行时 compile（require(vue).compile 实测 typeof=function）。探针 oa4rust/tests/r627c_probe.test.ts（已删）5 用例全过：
1. subRedo 源码计数=1、无定义、兄弟 subUndo 有；new Function(subRedo()) 抛 ReferenceError: subRedo is not defined。
2. subprocessEditing.value=true 计数=1、openSubprocess( 计数=1、enterSubprocessBreadcrumb( 计数=1，模板无 @click/@dblclick。
3. compile({{ groupNodes.length }}) 渲染出 2；compile({{ groupNodes.value.length }}) 渲染抛错含 length。
4. compile(:disabled='histIdx<=0') 按钮 disabled=true；compile(:disabled='histIdx.value<=0') 按钮 disabled=false。
5. pushHistory 整份存、stepBackward 读 .config 实测 THROW。
命令：cd oa4rust && node node_modules/vitest/vitest.mjs run tests/r627c_probe.test.ts --reporter=verbose → EXIT=0，5passed。日志见 <PROBE_LOG_PATH>。

## SHA 证据（分列；本文档不含自身 sha）
- runnerSha（sha256 紧凑 JSON meta: task/round/verdict/parent/sourceSha/routeSha/probeSha/target）= abcb0e77f23b25daba6b8ca72809ad1a05efe404e7e8c36afee09816253c1e3b
- 探针日志 sha256 = 75d0894f133477771f62bb199e93c361a3bcb0a9077ceef53b533688644dfcda
- ProcessDesigner.vue sha256 = add7505f1b754aca8762dd240231a7b8b5cb279c857946e99e919cf6e60cc586
- main.ts sha256 = e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0

## 据实披露 / Fail Loud
- 未跑浏览器 GUI，判定基于源码逐行 + Vue 编译器真执行，非端到端点击。
- desktop typecheck=tsc --noEmit 不解析 .vue，故这些模板误用属 typecheck 盲区存活。
- vitest.config.ts 的 __dirname 告警属既有配置，非本次引入。
- 首轮探针 new URL(import.meta.url) 在 vitest jsdom 环境被解析为 http 地址致失败，已改为 cwd 绝对路径后全绿，据实披露。
- 零源码改：git diff HEAD 无源码改动，仅新增本文档；探针与日志（gitignored）已清理。
- 涉事三处均非后端 handler，故本轮无临时 PG oneshot（任务硬纪律中该要求针对后端 handler 类甄别，本案为纯前端模板/脚本缺陷，据实说明跳过原因）。

## 脱敏
无密钥/口令/域名/IP/个人路径入库；探针日志路径以占位符表示。

## Evidence（完整值）
runnerSha=abcb0e77f23b25daba6b8ca72809ad1a05efe404e7e8c36afee09816253c1e3b
probeLogSha=75d0894f133477771f62bb199e93c361a3bcb0a9077ceef53b533688644dfcda
sourceSha=add7505f1b754aca8762dd240231a7b8b5cb279c857946e99e919cf6e60cc586
routeSha=e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0