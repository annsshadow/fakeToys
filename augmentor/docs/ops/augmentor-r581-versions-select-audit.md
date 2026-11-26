# R581 Versions.tsx 三枚原生 select 可访问名甄别（验证型）

## 结论
可达真缺陷 1 项，P2（WCAG 2.1 SC 3.3.2 标签与说明 / SC 1.3.1 信息与关系 / SC 4.1.2 名称、角色、值）。
创建版本弹窗的选择文件、版本对比弹窗的版本 1 与版本 2 共三枚原生 `select`，可聚焦且功能真实，但辅助技术 AX 名称为空（`name=""`）：屏幕阅读器只能播报 `combobox`，无法告知控件用途。

## 立项性质与争议披露
本轮为验证型：同子项目（augmentor/web）、同可访问名维度，但 Versions.tsx 这三枚具体实例从未专项甄别（台账 grep Versions.tsx / 选择文件 / 版本对比 / 创建版本 零命中）。按 R532「维度覆盖≠实例覆盖」先例裁决，不因 SC 维度已被 R577/R579 覆盖而判 0，也不做静态预判。
label 关联维度此前已在 oa4rust 桌面 / H5 / admin 的 R201/R213/R214/R217/R225/R235/R536/R538 反复甄别修复；本例是新子项目的新实例与局部疏漏信号，并非首次发现该维度。

## 范围与入口
源码：`augmentor/web/src/pages/Versions.tsx`（sha256 见证据表）。三处锚点：
- :270 label 选择文件 → :271-282 `select`（创建版本 Modal，`option value=""` 文本「请选择文件」+ files.map）。
- :316 label 版本 1 → :317-328 `select`（版本对比 Modal）。
- :331 label 版本 2 → :332-343 `select`（版本对比 Modal）。
三处均为裸 `<label>`，无 `htmlFor` / `id` / `aria-label` / `aria-labelledby`；三 `select` 无 `id`、无 `aria-*`，原生 select 不存在 placeholder 补名机制（`<option value="">` 是选项文本，不是控件可访问名）。
入口：`App.tsx` 侧边栏「版本管理」一跳到 `/versions`，零登录；页首「创建版本」/「版本对比」按钮分别开两 Modal（:244-251）。控件常驻可达，非造数态才出现。

## 方法
真浏览器 Playwright + Chromium CDP：`DOM.enable` / `Accessibility.enable`；`DOM.querySelectorAll` 取 nodeId → `DOM.describeNode` 取 backendNodeId → `Accessibility.getPartialAXTree({backendNodeId})` 与 `getFullAXTree` 按 backendDOMNodeId 对齐；`DOM.focus` 后读 `document.activeElement` 与 AX 节点身份。默认 / 注入 / 正面 / 交互四组探针，viewport 1440x900 DPR1，两种 Modal 分别取证，pageerror 全程为 0。服务地址以 `<TEST_HOST>:<PORT>` 占位，不入库真值；证据落 gitignored 的 `oa4rust/test-results/r581/`（据实披露借用）。

## 决定性证据
- 原生 select 身份（focusIdentity 探针）：`document.activeElement.tagName === "SELECT"`、`isBody=false`、无 aria-label，AX `role=combobox`、`name=""`、`ignored=false`；`expectNativeSelect=true`，坐实聚焦落在原生 SELECT 而非宿主 wrapper（R535 去伪）。
- 三枚 select 默认态：partial 与 full AX 均 `role=combobox / name="" / ignored=false`；DOM 属性除 style 与测试标记外无 id / aria / for。
- 注入 `aria-label=R581-inj-*` 后，三枚 AX `name` 实时变为 `R581-inj-infile` / `R581-inj-v1` / `R581-inj-v2` —— 决定性去伪：通道正常，缺陷是名称缺失，可修复。
- 备注（Fail Loud）：CDP `getFullAXTree` 的 `focused` 标志与 `axFocused` 在这些 React/antd 节点上未置位（`axFocused=[]`、干净 Tab 30 次 `hits=0`），与 R579 同类现象；原生 select 天然可聚焦（DOM 聚焦确证），本轮为可访问名维度，故不据此处判键盘不可达。

## 正面对照与去伪
- 同 Modal 内带 placeholder 控件：版本标签 `Input`（`placeholder="例如: v1.0"`）AX `role=textbox / name="例如: v1.0"`；版本描述 `Input.TextArea`（`placeholder="描述这个版本的内容"`）AX `name="描述这个版本的内容"`。证明 antd 对有 placeholder 的控件会自动补名，而原生 select 无此机制——与 R577「有 placeholder 的 Input 判 0」构成反向边界。
- 跨页正面对照（System.tsx）：`button[aria-label="校验备份完整性"]` AX `role=button / name="校验备份完整性"`，证明全局 AX 通道正常，非环境性失效。

## 交互对照（非死代码）
- 创建版本：`selectOption` 前值空 → 后值 `R581-input.json`；点击 OK 后 `createCalls=1`、弹窗关闭、页面出现「创建版本成功」，handler 真触发。
- 版本对比：两 select 分别由空 → `v1` / `v2`；点击 OK 后 `diffCalls=1`、出现「对比结果」（新增 1 / 移除 0 / 修改 0），handler 真触发。
- 等价入口排除：Modal onOk / onCancel 不提供 label 关联补偿；同页已带 placeholder 控件与 DataManagement/Augmentation 已修实例不混入判据；两 Modal 的三枚 select 逐一独立取证。

## 最小修复方向（业务零改）
三枚 `select` 补 `aria-label`（如「选择输入文件」「版本 1」「版本 2」）；或 `<label htmlFor>` 与 `select id` 显式关联。仅补可访问名，不改 `value` / `onChange` / handler 与业务语义；另立修复契约，不改动 R578/R580 既有 aria-label 计数。

## 证据 sha 表
runnerSha = sha256(JSON.stringify(report meta))；文件 sha256sum 为字节哈希，两者分列不可混用。
- runnerSha：主 `2bd8587eb501ce170660bae5896bb642cdf53b3dc3dd39396de224ca9a9a8d69` / 补 `d6899c96801e94b945b20d9794ddc2c984d5f3fd5a7fa89b346804218cfbf8c0` / 创建 `a06436e239d418a658e42410517335f48b0c57ef507a2ad107e76b319dead096` / 终补 `de612c40bcb990a0ba988fa40894dd6e089a0298198d563a1da36b69ff455845`（首版 fatal runnerSha `be4230b8721171f67f4d17a07df1dd95d35d32f69020b2c9115d986e76f9261a`，Fail Loud 未采信）。
- 报告文件：r581-report.json `299a86a2151fbc97e92a4d9c62f61c8764000074dd8a82c78845c8bccce191a5` / r581-sup-report.json `f10e7543ca3368efee58018455292c8cdca04a588803d4af14f7962f4ef49f80` / r581-create-report.json `a1b11c06a826bd4b63e0569bf79e2b2c1f9b0807b50922f92ef07f55b68765ae` / r581-final-report.json `aeb06fc8763d4fa96ef392e30ab510e63f81c1ac0e3eeaf1ba73958539e090cc`。
- 截图：default-create `cc2636e19820102a88bee58109f18f5015c1dfa48c5b119eea9fef25c0bbb65c` / default-diff `cef3c2489eee4f51ceb557dda71769f86d59febdc2063a93593bd95cf52cb182` / injected-create `47d1babf0798ff0da8ecad6f5c65fe2b4b707c1b790ecfa12d5dccadba890fa1` / sup-diff `1d9257e35d6cc76921e64d1229bd511fd69e265bf41a4011341b3f962f9f7d8a` / sup-create `e5ea95ae49d326b8c0b29d822156544d5c3f580c9ee912b8e876aaf4b5bf29dc` / final-system `6c0262221b73b4fba4f66005eaa0f9992ff307fcaa55aefe439ca2c7157c7d84`（均 1440x900，已逐张目视名实相符）。
- 源码：Versions.tsx `e38b31e5855d26dc4eda39ec4645b03aeecbcb66fc1bc2f9d44ce0b47ad90c3d` / App.tsx `f6e245c48c66be157899b98c087aecb1c4e550fecce8e4058fa53d6a383493b4` / api.ts `2534a96282729c71c528a4b5602e6a66c8f8cd348ce42e9100e5fa7561b3130c` / System.tsx `97f817e254ea16645b8a7301144f94c4ae92f0f6045c62ae84a37dbdaf36796c` / main-schemas.ts `1daef5a9b24e5b240d508267afa877f65ea7715e06a3fd6478bcdfe8f97e3551`。
