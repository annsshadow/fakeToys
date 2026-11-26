# R580 DataManagement 编辑弹窗 TextArea 可访问名修复（augmentor/web）

- 团队：oa4rust-pc-ai-refactor；任务：t125（implementation），attempt_id=b62ad5d0-696d-466b-a590-8aa0a06286e4。
- 真父 HEAD：94c4fa76f414daa126482dc666cf4f73eddd3afa（R579 甄别判 P2 真 bug）。
- 缺陷：augmentor/web 数据管理页“编辑数据”弹窗两处 \`Input.TextArea\` 裸 label，无 \`for/id/aria/placeholder\`，antd 6 不自动补名；CDP AX name=null。关联 WCAG 2.1 SC 3.3.2 Labels or Instructions / 1.3.1 Info and Relationships / 4.1.2 Name, Role, Value。
- 修复形态（Fail Loud）：方案 B 逐控件显式补 \`aria-label\`，而非 label \`htmlFor+id\`——aria-label 更直接，且注入对照已证实时生效。全局增强器（MutationObserver 等）越界他页，Fail Loud 排除。

## 写域（精确 3 文件）

- M \`augmentor/web/src/pages/DataManagement.tsx\`：2 insertions，纯 aria-label 追加。
  - 弹窗问题 textarea（rows=3，instruction）：开标签补 \`aria-label="问题内容"\`。
  - 弹窗回答 textarea（rows=5，output）：开标签补 \`aria-label="回答内容"\`。
  - 修复版 sha256=a358e50e7ba2e0beb4ed5732255a2da4f9d4fe2e86397d6d2b2a4896ef7e3e5d；真父 sha256=8c179ae5ab58a85abdcd1fa9a677e535ccacfd45ec5902acf6551805576364a8。
- A \`augmentor/web/src/pages/R580.dataManagementTextareaA11y.contract.test.ts\`：7 用例（3 核心 + 4 守卫），String.raw 源码字符串断言（沿用 R503/R578 惯例）。核心 1/2 断言两处 textareaBlock 开标签 aria-label；核心 3 断言全页 aria-label 总数 37（真父 35）。守卫 4 handler（handleEdit/handleSaveEdit/absoluteIndex）零改；守卫 5 onChange/state/弹窗开关零改；守卫 6 Augmentation R578 六处 aria-label 零改、无 MutationObserver；守卫 7 本页转换输出文件名/分割输出目录/上传源格式正面对照零改，TextArea=2、Modal=1。
- A 本文档 \`augmentor/docs/ops/augmentor-r580-datamanagement-textarea-fix.md\`（脱敏，sha 表不含自身，避免自引用悖论）。

零改清单：业务 handler/state/样式/他页/main.tsx/App.tsx/Augmentation R578 修复/本页既有 35 处 aria-label 控件/既有测试/两处 onChange/弹窗开关。

## 契约 RED → GREEN

- GREEN（修复版 Source）：EXIT=0，7 passed。
- RED（真父 Source，git show 覆盖）：EXIT=1，3 failed（核心 1/2/3）+ 4 passed（守卫 4–7），符合“核心 fail、守卫 pass”纪律。
- 恢复修复版 sha256 复核=a358e50e…，HEAD 测试阶段未变 94c4fa76f。

## 三 verify（workdir augmentor/web，字面命令）

1. \`cd D:/WORKSPACE/fakeToys/augmentor/web && npx vitest run src/pages/R580.dataManagementTextareaA11y.contract.test.ts\` → EXIT=0，7 passed。
2. \`cd D:/WORKSPACE/fakeToys/augmentor/web && npm run test\` → EXIT=0，28 files passed / 464 tests passed，无 skip。
3. \`cd D:/WORKSPACE/fakeToys/augmentor/web && npm run typecheck\` → EXIT=0，tsc --noEmit 无错。

## 真浏览器 CDP 决定性判据

- 环境：自起 Vite \`<TEST_HOST>:<PORT>\`（占位；实际本地回环端口未入档），Playwright 1440×900 DPR1 headless，CDP \`DOM.getAttributes\` + \`describeNode\` backendNodeId → \`getPartialAXTree\`/\`getFullAXTree\` 映射，focused 过滤非 root。
- runner：\`oa4rust/test-results/r580/r580-e2e.cjs\`，基于 R579 + R578 模板改造。
- 汇总：R580_DONE runner 紧凑 JSON sha256=b43b9607eb8de7bc8189254419d058b1b15170328012758f7ec786f8147eb773，cases=4，fatals=0,0,0,0，pe=0,0,0,0，updateCalls=1。
- 决定性判据全绿：
  - default / injected 两处 TEXTAREA \`aria-label=问题内容/回答内容\`，partial + full AX \`role=textbox\`、\`name=预期\`。
  - focused \`activeElement=TEXTAREA\`，\`isTarget=true\`、\`isBody=false\`、\`inModal=true\`，AX by backendNodeId role/name 达标（非宿主 wrapper 假阳性，R535 教训）；Tab 从 instruction 到 output（\`isOutput=true\`）。
  - 注入对照 stripped 两处 name=null → injected \`R580-inj-instruction/output\` 恢复。
  - 正面对照：本页转换输出文件名 textbox / 分割输出目录 textbox / 上传源格式 combobox name 达标；Augmentation R578 修复 select combobox=输入文件 / output textbox=输出文件 / q switch=质量检查达标。
  - 0 pageerror / 0 fatal。
- 据实披露（探针方法瑕疵，非回归）：主 runner 用 \`.ant-modal\` count 判断弹窗关闭，antd 关闭后仍保留 DOM（display:none），故 save/reopen/cancel 显示 modalCount=1、reopenRaw 为空（v-for 重建丢 data 标记）。专项探针 \`oa4rust/test-results/r580/r580-modal.cjs\` 改用 visible 判定（getComputedStyle display/visibility + getClientRects）+ 每步重标记：R580_MODAL_DONE 紧凑 JSON sha256=780e20a536574a5bfcc4c3c7a1c837779c44f1e07f01d2ad8ce0e37bbe225f42，fatals=0、pe=0，opened=1、afterSave={visibleModal:0,successMsg:1,updateCalls:1}、reopened=1、afterCancel={visibleModal:0}，坐实弹窗生命周期零回归。
- 据实披露（造数/界面提示）：stub 未覆盖 checkpoints 接口，Augmentation 截图顶部出现 2 条“加载断点列表失败”前端提示，与本次 DataManagement TextArea 修复零关联；Select Option 由 \`GET /api/data/list\` 轻造数据提供。

## 截图（1440×900 视口，fullPage 1701×1908 或页面原生高）

- \`r580-default.png\`：编辑数据弹窗，问题/回答 textarea 填入 R580-instr-1 / R580-output-1，Cancel/OK 正常（aria-label 为 DOM 属性，视觉不可见）。
- \`r580-injected.png\`：同视图注入 aria 后截图（视觉同 default）。
- \`r580-positive.png\`：数据管理主页面正面对照控件齐全。
- \`r580-augmentation.png\`：数据增强页，R578 六控件与任务 ID Input（placeholder 自动补名）齐全。
- \`r580-modal-reopen.png\`：弹窗重开 + “保存成功” toast，坐实生命周期零回归。
- 5 图均目视名实相符；sha256 见下表。

## sha256 证据表（A 组 runner 紧凑 JSON 与 B 组文件 sha256sum 分列）

A 组（runner 汇总紧凑 JSON，与文件 sha256sum 不同，不可混用）：

| 证据 | sha256（JSON.stringify 紧凑串） |
| --- | --- |
| r580-report.json 汇总 | b43b9607eb8de7bc8189254419d058b1b15170328012758f7ec786f8147eb773 |
| r580-modal-report.json 汇总（专项弹窗探针） | 780e20a536574a5bfcc4c3c7a1c837779c44f1e07f01d2ad8ce0e37bbe225f42 |

B 组（文件 sha256sum，逐项实测）：

| 文件 | sha256 |
| --- | --- |
| augmentor/web/src/pages/DataManagement.tsx | a358e50e7ba2e0beb4ed5732255a2da4f9d4fe2e86397d6d2b2a4896ef7e3e5d |
| augmentor/web/src/pages/R580.dataManagementTextareaA11y.contract.test.ts | 975abaaba688cebcc6fbe69a76f92ca01f520be3c267b0bd3c4b3ddf27ce3f3c |
| oa4rust/test-results/r580/r580-e2e.cjs | a6010c4fcdb99978c03665d561acfe6eedb967d7ce6b444b35f8b9d66895d8e8 |
| oa4rust/test-results/r580/r580-report.json | 8dcd37fdb10da7bd4f99f66c7128c0fd9dc6e57e6ab8539eb593e1bea19e5791 |
| oa4rust/test-results/r580/r580-modal.cjs | c5c8459447f8fb2d8248cb3c3802d1e3f69241b7989b75264a8386c594ca052e |
| oa4rust/test-results/r580/r580-modal-report.json | 9b3ebad909fe2602a862ee960c01ca2073a5aa90e6591e4cafea8a2a9e7ba920 |
| oa4rust/test-results/r580/DataManagement-fixed.bak.tsx | a358e50e7ba2e0beb4ed5732255a2da4f9d4fe2e86397d6d2b2a4896ef7e3e5d |
| oa4rust/test-results/r580/DataManagement-parent.bak.tsx | 8c179ae5ab58a85abdcd1fa9a677e535ccacfd45ec5902acf6551805576364a8 |
| oa4rust/test-results/r580/r580-default.png | 479ccd477c02f0f698a0b1fa938a7adadce574b638aacafabcfe0ea14c6d4de9 |
| oa4rust/test-results/r580/r580-injected.png | a70e875b56589ece0180151bbe23cf723e71b79e74aed6591bccb555d458b426 |
| oa4rust/test-results/r580/r580-positive.png | 23c115046e912f4e4e79b31f3e0186097f4510e891ce7aca3064f48ab40d3054 |
| oa4rust/test-results/r580/r580-augmentation.png | 80594739bc0bffa88453f25a96f9b5086168121e22fa74f04f79c5838f4dd341 |
| oa4rust/test-results/r580/r580-modal-reopen.png | 8131e9cb04d3af66b49699d0e96677eeb897d1d7399e44b342ca37d2742dfef8 |
| /tmp/r580-red.log | bbce8dff32526c5ab92aa3b8d62b4526ab84e04be68f565bcbd20250dcc9d1a8 |
| /tmp/r580-green.log | 4401cb0bf0840abda935ab04010824745e776c8928f9618485fc631d07651600 |
| /tmp/r580-v1.log | fdce06ecf8c7fed65cc8845b0d0112bb2e6b7336b85de9b02f9d9a73a33d8740 |
| /tmp/r580-v2.log | c5b1009e404147bb414d7f6b41744601a6421383b630323f5ba709f890d07b11 |
| /tmp/r580-v3.log | 308f72d15ad935773f505e41a22444c64b8cca155628fa2d60610233bf142ec1 |
| /tmp/r580-e2e.log | 7ddaabf78955251fb6b733a7f0b8c36c6e9f2fc8debadbd341a5114c07829bbb |

## 脱敏自检

- 全文仅 \`<TEST_HOST>:<PORT>\` 占位符，0 真实服务器地址/IP/域名/端口/令牌/个人路径。证据均位于 gitignored 的 \`oa4rust/test-results/r580/\`（未 track）。
