# R582 Versions.tsx 三枚原生 select 可访问名修复（接续 R581）

## 结论
修复完成：创建版本弹窗「选择文件」、版本对比弹窗「版本 1」「版本 2」三枚原生 select 补齐可访问名（aria-label=选择输入文件 / 版本 1 / 版本 2）。真浏览器 CDP 坐实三处 AX role=combobox、name 由空变为预期中文语义，ignored=false；R581 判定的 P2（WCAG 2.1 SC 3.3.2 / 1.3.1 / 4.1.2）已闭合。

## 立项性质与方案取舍（Fail Loud）
本轮为修复型，接续 R581 甄别结论（真父 HEAD be7038d33）。方案在「select 补 aria-label」与「label htmlFor + select id 关联」二选一：据实选前者——aria-label 只在三处 select 开标签追加一个属性，不新增 id、不改 label、不触碰 Option value 与现有 CSS，是与 R578/R580 既有 aria-label 范式一致的最小改动；htmlFor+id 需同时改 label 与 select 两处并引入新 id，改动面更大且无额外收益。
缺陷维度（label 关联/可访问名）此前已在 oa4rust 桌面/H5/admin R201/R213/R214/R217/R225/R235/R536/R538 覆盖，augmentor/web 由 R577/R579/R581 覆盖三个新实例；本例是 R581 实例的修复收官，非首次发现该维度。

## 范围与写域
精确三文件（零越界）：
- M `augmentor/web/src/pages/Versions.tsx`：仅三处 select 开标签追加 aria-label（:272 选择输入文件 / :319 版本 1 / :335 版本 2），label 与 value/onChange/handler/state/样式原样。
- A `augmentor/web/src/pages/R582.versionsSelectA11y.contract.test.ts`：127 行 7 用例（核心 1/2/3 + 守卫 4/5/6/7）。
- A 本文档。
Out of scope：System.tsx / DataManagement.tsx / Augmentation.tsx / main.ts / App.tsx / oa4rust/** / laiyipao/** / docs/ops/current-state.md 均未改。

## 真父独立 RED → GREEN replay
以真父 HEAD 的 Versions.tsx（aria-label 计数 0）覆盖修复版跑定点契约测试：RED_EXIT=1（7 tests、3 failed，核心 1/2/3 在 :49/:56/:63 断言 expected false to be true，守卫 4/5/6/7 全 pass）。恢复修复版：GREEN_EXIT=0（7 passed）。FINAL HEAD 未变 be7038d33，恢复后源码 revision 0601e60b，diffstat 3 insertions(+)。

## 三 verify（cd augmentor/web，全 exit 0）
- npx vitest run src/pages/R582.versionsSelectA11y.contract.test.ts：V1=0（7 passed）。
- npm run test：V2=0（全量套件通过，仅 antd deprecation warning，非失败）。
- npm run typecheck：V3=0（tsc --noEmit 无错误）。

## 决定性 CDP 证据（真浏览器 Playwright + Chromium）
DOM.enable + Accessibility.enable；querySelectorAll → describeNode({nodeId}) 取 backendNodeId → getPartialAXTree({backendNodeId,fetchRelatives:false}) 与 getFullAXTree 按 backendDOMNodeId 对齐；DOM.focus 后读 activeElement。viewport 1440x900 DPR1，两 Modal 分别取证，pageerror 全程 0。服务地址以 `<TEST_HOST>:<PORT>` 占位，不入库真值；证据落 gitignored 的 `oa4rust/test-results/r582/`（据实披露借用）。
- 默认态三 select：partial 与 full AX 均 role=combobox、name=预期（选择输入文件 / 版本 1 / 版本 2）、ignored=false；DOM attrs 含 aria-label 同名。
- 注入对照（决定性去伪）：REMOVE aria-label → partial/full name=''（复现 R581 缺陷态）；RESTORE → name 实时恢复为预期值。证明通道正常、修复生效、缺陷确为名称缺失。
- 焦点身份：DOM.focus 后 activeElement.tagName=SELECT、isBody=false、aria-label=预期，坐实焦点落在原生 SELECT 而非宿主 wrapper（R535 去伪）。
Fail Loud：CDP getFullAXTree 的 focused 标志与 axFocused 在 React/antd 节点未置位（axFocused=[]、isTarget=false、nativeSelectFocused=false），与 R579/R581 同类现象；本轮是可访问名维度且 name 已可读，不据 AX focused 判键盘可达。

## 正面对照与去伪
- 同 Modal 带 placeholder 控件：版本标签 Input（placeholder=例如: v1.0）AX name=例如: v1.0；版本描述 TextArea（placeholder=描述这个版本的内容）AX name=描述这个版本的内容——证 antd 补名通道正常。
- 跨页正面对照（System.tsx）：button[aria-label=校验备份完整性] AX role=button、name=校验备份完整性、ignored=false（count=1），证全局 AX 通道正常。
Fail Loud：正面对照探针 b 选择器写为 button[aria-label=备份源文件]，而源码该 aria-label 位于 Input（System.tsx:735），故 count=0，未采信；a 已命中且 name 可读，正面对照结论成立。

## 交互对照（零回归）
- 创建版本：select 由空 → R582-input.json，OK 后 createCalls=1、出现创建版本成功、弹窗关闭。
- 版本对比：两 select 由空 → v1 / v2，OK 后 diffCalls=1、出现对比结果。
- 等价入口排除：Modal onOk/onCancel 无 label 关联补偿；两 Modal 三枚 select 独立取证；同页已带 placeholder 控件与 DataManagement/Augmentation 已修实例不混入判据。

## 守卫（业务零改）
契约测试守卫 4/5/6/7 在 RED（3 failed 时）与 GREEN（全 pass）两态均通过：handler（create/diff/load/show）零改、select 原生结构与 value/onChange 全量保留、R578/R580 既有修复零改且无全局增强器、System.tsx 正面对照与 state/import 零回归。R578 Aug 恰 6 处、R580 DM 恰 37 处 aria-label 既有计数零改。

## 证据 sha 表
runnerSha = sha256(JSON.stringify(report meta))；文件 sha256sum 为字节哈希，两者分列不可混用。
- runnerSha：`9b856a145fd60afff832bbbe493ec7a41ee9e1f45909bb97643c14f6bda4e0c3`。
- 报告文件：r582-report.json `58ad90ef78b48ff83b3fca8e47bf71292f867db87ce151271d1f3d094f2745dd`。
- 截图（均 1440x900，已逐张目视名实相符）：default-create `213f976a0b81c1dc9c938889d6516c506d233e6cc1a570359ebd991756a2f739` / default-diff `67448b139073969f9276fd597afdfdaf0efe5146186ab2ad6749e148a88f9ca1` / injected-create `5218a10fdc6729caa309bab3f8d8fb48126f0adb22c726ac5c5f81054e5ac96f` / positive-system `6665a459ab7834986faa74e0c00ded9de359487dd3dda3830bd676adb746c630`。
- 源码：Versions.tsx（修复后）`0601e60b529d4a7f91e259e486bf5a288651bdaeb9e89521a9bfa310a0846433` / R582.versionsSelectA11y.contract.test.ts `200e265308906b0fa4227d2e2e53036b75b8b9efef7230b6e32f81d47c5fca31`。
- RED/GREEN replay 日志：`oa4rust/test-results/r582/red.log`（RED_EXIT=1）/ `green.log`（GREEN_EXIT=0）；三 verify 日志 v1-contract.log / v2-alltest.log / v3-typecheck.log 全 exit 0。
