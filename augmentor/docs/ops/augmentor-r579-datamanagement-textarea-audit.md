# R579 DataManagement 编辑弹窗 TextArea 可访问名甄别

## 结论
判可达真缺陷 1 项，严重度 P2，违反 WCAG 2.1 SC 3.3.2（标签与名称）关联 4.1.2（名称、角色、值）：编辑数据弹窗两处 Input.TextArea 的可访问名（AX name）为 null，屏幕阅读器无法读出字段用途。
立项性质为验证型甄别：label 关联/可访问名维度已在 oa4rust 桌面/H5/admin 及本子项目 R577 反复甄别修复；本例为 augmentor/web DataManagement 编辑弹窗 TextArea 的具体实例，按维度覆盖不等于实例覆盖（R532 先例）据实披露，非首次发现该维度。
三分裁决：P2＝两处 TextArea AX name=null；正常 0＝正面对照本页 35 处 aria-label 控件 name 可读、注入 aria-label 后两处 TextArea name 实时变为预期值；无 L1（控件稳定可达，非死代码）。

## 范围与入口
- 对象：augmentor/web/src/pages/DataManagement.tsx（sha 见下表），React 19 + antd 6 + Vite 8。
- 源码锚点：:671-698 Modal 编辑数据；:681 label 问题 (instruction) → :682-686 Input.TextArea rows=3，无 placeholder/id/aria-label/for；:689 label 回答 (output) → :690-694 Input.TextArea rows=5，同样无任何命名属性。
- 状态与写路径：:145-147 editModalVisible/editingItem/editingIndex；:262-266 handleEdit 打开弹窗；:268-279 handleSaveEdit → updateDataItem(selectedFile, editingIndex, editingItem) + message.success(保存成功) + loadDataList；:563-568 表格编辑按钮 onClick=handleEdit。
- 入口：App.tsx:149 Route path=/ element=DataManagement 启动首页 0 跳；点击表格行编辑按钮即可达弹窗（stub 造 2 条演示数据，零后端写造数）。
- 可达性据实：两处 TextArea 随弹窗常驻，键盘 Tab 可从 instruction 移到 output（补充探针坐实）；可访问名缺失是唯一缺陷维度。

## 方法
真浏览器 CDP 实证，非静态预判：Playwright chromium 1440x900 DPR1，page.route 仅拦 `/api/**` 造 data/list、demo/data、data/upload、data/load、data/update（计数写请求）；DOM.enable + Accessibility.enable。
probe：querySelectorAll + DOM.describeNode({nodeId}) 取 backendNodeId，getPartialAXTree({backendNodeId}) 与 getFullAXTree 按 backendDOMNodeId 映射，双通道核对 role/name。
3 主组 default/injected/positive：default 读原生属性与 AX name、测保存交互；injected 注入 aria-label 复测；positive 取本页 aria-label 控件作正面对照。
2 补组 sup（首跑 fatal 已 Fail Loud 披露未采信）/sup2（修正取消按钮选择器后 fatals=0）：坐实 focused 节点身份、键盘 Tab、弹窗开关、保存真写请求。
服务地址仅以 `<TEST_HOST>:<PORT>` 占位，真值不入库。

## 决定性证据
- default（fatals=0 pe=0）：两处 TextArea DOM tag=TEXTAREA，attrs role/tabindex/aria/labelledby/id/for/placeholder 全 null，partial 与 full AX role=textbox name=null ignored=false；两 label AX role=LabelText name=null。
- 关键裁决：antd 6 对无 placeholder 的 TextArea 未自动补名（与 R577 有 placeholder 的 Input 经 placeholder 补名判 0 不同），name=null 坐实可访问名缺陷。
- injected：注入 aria-label=R579-inj-instruction / R579-inj-output 后，两处 TextArea AX role=textbox name 实时变为对应值，证缺陷可修复、AX 通道正常。
- positive：转换输出文件名 name=转换输出文件名、分割输出目录 name=分割输出目录、上传源格式 role=combobox name=上传源格式，证本页命名控件 AX 通道正常，非全局静默。
- sup2（fatals=0 pe=0）：focus 后 activeElement 为 TEXTAREA（isTextarea=true inModal=true，data-r579-active 身份独立确认，非宿主 wrapper）；AX focused=false / axFocusedNodes=[] 系 CDP AX 树与 antd6 特性，name 取自 AX 节点本身与 focused 无关，不影响裁决，据实披露。
- sup2：键盘 Tab 从 instruction 移到 output（afterTab.isOutput=true）；取消后可见弹窗 0、重开后 1、保存后可见弹窗 0 + successMsg=1 + updateCalls=1，坐实交互真走 PUT 写请求、非死代码。

## 等价入口与排除
- 全页 aria-label 共 35 处（转换/合并/分割/采样/聚合/RAG/翻译控件与回执 div），均为已命名控件，正面对照成立。
- 两处 TextArea 无其它命名途径：无 for/id 关联、无 placeholder、无 wrapping label 可访问名传递，排除隐式命名。
- 编辑弹窗仅此两处 TextArea 承载 instruction/output 字段，无等价替代输入控件。
- pageerror 全程 0，无脚本异常掩盖缺陷。

## 最小修复方向
给 :682-686 与 :690-694 两处 Input.TextArea 各补 aria-label（如 aria-label=问题内容 / 回答内容），或为 label 补 htmlFor 并为 TextArea 补对应 id；业务零改、仅补可访问名，复用现有 state 与 handleSaveEdit。补后按 injected 组复测 AX name 应非空。测试缺口：Augmentation.test.tsx 同类用 label.parentElement.querySelector 未断言可访问名，建议补 getByLabelText 断言防回归。

## 证据 sha256 表
runner 紧凑 JSON sha（JSON.stringify 摘要）与文件字节 sha256 分列，不可混用：
- 主 runner 紧凑 sha = fd3d383f998e9e916d8a17444ae3c3114d4460f0076ff9adb1de87aa3058987a
- 主报告 r579-report.json 文件 sha256 = cef8b1ec64c73d1061738ab743288be7be879ab080359e62c3237ed2bbf9194d (12109B)
- 补 runner 紧凑 sha = 90823fca92c1ad5aec03f2c9c73a8125b8d9ec03edb7e0edcf02a902aa2701f4
- 补报告 r579-sup2-report.json 文件 sha256 = 982e13844aa19265d2ac187050990479c57710af83b9c892f656c014e7404673 (1202B)
- 截图 r579-default.png = e51c0a047163aaeda2aed2f967016eea104c485a8d7e7d9496a498aff6fdd348 (257889B)
- 截图 r579-positive.png = 72a7ff296d034a6913d0c14137c29aa9882429308694012d6ab23197e3bcbb88 (235462B)
- 截图 r579-injected.png = 8ffac9aae82516baa1d298f34b0cd44b89277a35da99144e69208968cb9b7cf5 (257926B)
- 截图 r579-sup2-focus.png = c3da7cd67ce8893b70e8b0a40f78b58122b5168c8e958248e7b47aefec64e7e0 (239080B)
- 截图 r579-sup2-after-save.png = 57e84f04ed890e39546e1e077a50f8b3afd3c625dc4dd8a6bdfa25171638ec07 (259028B)
- 源码 DataManagement.tsx = 8c179ae5ab58a85abdcd1fa9a677e535ccacfd45ec5902acf6551805576364a8
- 源码 api.ts = 2534a96282729c71c528a4b5602e6a66c8f8cd348ce42e9100e5fa7561b3130c
- 源码 App.tsx = f6e245c48c66be157899b98c087aecb1c4e550fecce8e4058fa53d6a383493b4
- 源码 main-schemas.ts = 1daef5a9b24e5b240d508267afa877f65ea7715e06a3fd6478bcdfe8f97e3551
- 源码 full-schemas.ts = 4bee4baf4ee2987f31f6bd64254663bb49ef075604e615d5fce185828d22f904

证据文件（报告/截图/探针）位于 gitignored 的 oa4rust/test-results/r579/，据实披露借用；零源码改动，仅提交本文档。