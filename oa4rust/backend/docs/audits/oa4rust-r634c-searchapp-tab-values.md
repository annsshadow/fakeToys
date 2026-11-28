# R634c SearchApp 分类tab值域错配甄别（零源码改）

## 结论

判 **P3（低）**：分类筛选语义确属失效但可绕过，核心检索与全部结果列表正常，无越权/无写库/无主流程中断，不硬凑更高等级。

## 事实链（真父 856e56ff，源码零改）

- 目标：`frontend/apps/desktop/src/views/SearchApp.vue`（sha256 见文末 sourceSha）。
- 写入侧：`doSearch()` 解包 `{designList,viewList}` 信封后，`:95` `designList.map → type:'design'`、`:96` `viewList.map → type:'view'`；两值域仅 `{design, view}`。
- 过滤侧：模板 `:35` `type==='doc'`、`:44` `type==='process'`、`:53` `type==='org'`；三值域 `{doc, process, org}`。
- 交集为空 `{design,view} ∩ {doc,process,org} = ∅` ⇒ 文档/流程/组织三个分类 tab 恒空。
- 图标 `:27` 仅识别 doc/process/org，对 design/view 恒回退 📋 ⇒ 全部结果 tab 所有条显示同一图标。
- 兄弟对照：`:26` 全部结果 tab `v-for="r in results"` 不过滤，正常渲染（可绕过，用户仍能看到并点击全部结果）。
- 路由：入口 `main.ts:116-119` `path:'search' / name:'SearchApp'` 可达，非不可达死代码。

## 真测试坐实（vitest 5.0.5 + new Function 真实执行，6/6 通过）

临时探针 `tests/r634c_probe.test.ts`（取证后已删除，不入库）用 `new Function` 真实执行写入 map 与模板表达式，非字符串静态推测：
1. 写入 map 真实执行：喂入 designList/viewList ⇒ 产出 `type=['design','view']`，不产出 doc/process/org。
2. 三个分类 filter 表达式真实执行 ⇒ 对真实写入结果各命中 0 条（恒空坐实）。
3. 图标表达式真实执行 ⇒ 对 design/view 均返回 📋（恒定坐实）；全部结果 tab 不过滤。
4. 过滤表达式有效性反证：喂入 doc/process/org ⇒ 三个 filter 各命中 1 条，证明是值域错配而非过滤失效。
5. 契约盲区：R315 契约锁定 design/view 写入；business 契约只查 tab 文案与 `v-for="r in results"`，不校验 type 值域。
6. 路由可达；台账三份仅记 aria-selected 备料，无分类 tab 值域专项甄别。

## 三分据实（判级理由）

- 不判 L1（不可达死代码）：该页面路由可达、全部结果 tab 正常渲染，用户可绕过分类 tab；与无置真点不可达的死绑定不同。
- 不判 P2：无身份/越权、无写库、无数据泄露；核心搜索结果在全部结果 tab 完整可见，仅分类视图失效。
- 不判 P1：不阻断主流程，不影响搜索可用性。
- 判 P3：纯粹的值域映射不一致导致分类过滤与图标降级；可绕过、影响有限、确定性可复现。

## 据实披露

- 未跑浏览器 GUI 人工点验；结论基于源码逐行复核 + 真实表达式执行探针。
- desktop 包 `tsc --noEmit` 不解析 .vue（非 vue-tsc），故类型层面无法捕获该值域错配，属盲区。
- 探针与日志仅本地取证，未入库；文档仅含占位符与哈希，无真值。
- 建议（不在本甄别范围）：统一写入 type 与 tab 过滤的值域，或在图标分支补 design/view 映射。

## 证据 SHA（分列，均非真值）

- sourceSha  = 0f3c5d680fbb766acbfd34c6098c11e7024bad05843294f937fa021a2fdc692b
- probeLogSha = 4ec3e0182431cc6da9b04ecce7b74a03b365d7db37034ef06637014c798b9cb6
- parent     = 856e56ffe43dcde249caed8c590bb7e1c494077c

- docSha     = 4e066a9a97d2c948e341e6f33a1ca11e3e3a77d14ff4cba23f1f3e05b62056be
- runnerSha  = c8ebc1ab04f33f102105a41b4e12863d94f9670d4e0cc36f9f27ef3c39dda727
