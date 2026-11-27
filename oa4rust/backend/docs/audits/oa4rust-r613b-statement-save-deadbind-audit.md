# R613b QueryStatementDesigner 保存按钮 saveStatement 死绑定甄别

- 任务：t213 / R613b（verification，零源码改）
- 派发 HEAD：c0947b0c1591645f5bf58c72b4b451048435251c（git rev-parse 核实，工作树 HEAD 同）
- 裁决：P2（主保存按钮静默失效，用户可感知；存在两条真实替代路径，不越权、不写库、不崩溃、不丢数据）
- 目标文件 oa4rust/frontend/apps/desktop/src/views/QueryStatementDesigner.vue sha256=c71dba6aa59f54ba576cfc3599f5fa39936b5e6ed133392f96d363c23575371b（bytes=168325），git diff HEAD 空，零源码改。

## 缺陷定位
- 第 15 行：保存按钮 <button class="btn btn-primary" :disabled="!currentStatement" @click="saveStatement">💾 保存</button>。
- 严格 grep（path=oa4rust/frontend，glob=vue/ts，scanComplete=true，matchedTotal=1）唯一命中即第 15 行模板绑定；script setup 中 function/const/let/var/import 六种定义形式均无 saveStatement。
- 可达性：第 929 行 currentStatement=ref(null)，第 1024 行 selectStatement(s) 内第 1025 行 currentStatement.value=s、第 1026 行 sql.value=s.sql；选中语句后第 15 行按钮 disabled 解除。
- 路由：oa4rust/frontend/apps/desktop/src/main.ts:225-226 path query-statement-designer / name QueryStatementDesigner / component 动态 import。

## 真执行证据（vitest + jsdom）
- 临时探针 oa4rust/tests/r613b_probe.test.ts（4 用例，用后即删）；命令 cd oa4rust && npx vitest run tests/r613b_probe.test.ts → EXIT=0，4 passed（vitest v5.0.5）。
- 日志 sha256=4cce73ca8050fe52ec8249e9186122c981b92fc7df3f769961ca2bcc84451aaa（日志原文路径不入库，占位 <PROBE_LOG_PATH>）。
- 用例 1：模板同时绑定 saveStatement(:15) / saveByFlag(:16) / modalSave(:141)，saveStatement 全文件仅 1 次命中，六种定义形式均无。
- 用例 2：严格 eval new Function("use strict; saveStatement()")() 抛 ReferenceError: saveStatement is not defined。
- 用例 3：Vue 真挂载 defineComponent + createApp，模拟编译产物 _ctx.saveStatement=undefined，btn.click() 无 emitted / 无 Vue warn；对照 _ctx.saveByFlag 为函数时点击产生 emitted=[saveByFlag]。
- 用例 4：selectStatement 在第 1024 行位于 saveByFlag(:1060) / modalSave(:1079) 之前；两替代函数体内含 api.put 与 toast.success / saveM.mutate。

## 替代路径（据实披露，判 P2 的决定性依据）
- 第 16 行按 Flag 保存按钮绑定 saveByFlag；第 1060 行 async function saveByFlag，第 1065 行 api.put 到 statement/{flag} 携带 alias 与 data，第 1073 行 toast.success(语句已按 flag 保存)。
- 第 141 行弹窗保存按钮绑定 modalSave；第 1079 行 function modalSave 调 saveM.mutate(payload)；第 1028 行 newStatement / 第 1033 行 editStatement 打开弹窗，第 1039 行 saveM=useMutation，第 1044 行 editing 且 currentStatement.id 时 api.put 到 save/{id}。
- 即主保存按钮死绑定，但用户可经按 Flag 保存或弹窗保存完成落库；替代路径真实存在，不构成 P1 的主流程阻断。

## 分级论证（三分据实）
- 排除 P0 / L1 / 0：仅单按钮失效，非全局崩溃，非加载错误，非 0 优先级噪音；选中语句后按钮可见可点，属真实可感知交互失效。
- 排除 P1：不越权、不写库、不丢数据、不崩溃；两条替代保存路径可达，用户可绕过。
- 判 P2：默认主保存按钮点击无任何响应（静默 no-op），属局部功能失效；虽可绕过但用户缺乏提示，易误以为已保存。

## 门盲区与方法局限（据实披露）
- typecheck 盲区：desktop 包 typecheck=tsc --noEmit，不解析 .vue 单文件组件，故 TS2304（saveStatement 未定义）未被捕获，缺陷存活 HEAD。
- 未跑浏览器 GUI 真页面；以 jsdom 真 Vue 挂载 + 源码严格求值 + 真 grep 锚点替代，等价于模板编译产物 _ctx 属性访问语义。
- 无真实服务器/口令/个人路径写入；日志路径以占位符表示；探针与日志均未入库。

## 校验标识（runnerSha 与文件 sha256 分列，文档不含自身 sha）
- runnerSha=96e6df283b339d4fbdd384d2f5e59451bc92e4a89292f28c4e1e0e0f7db25fb8
- runnerSha 口径：sha256(JSON.stringify(meta)) 紧凑 JSON，meta 含 task/round/verdict/parent/head/sourceSha/logSha/anchor。
- 文件 sha256 见提交记录处 sha256sum 输出（本文件不内嵌自身哈希）。
