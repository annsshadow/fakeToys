# R577Augmentation 页面 label 控件可访问名甄别（augmentor/web）

## 结论

本轮为验证型甄别，聚焦数据增强页（Augmentation.tsx）6 处 label 标注控件的可访问名。经真浏览器 CDP 无障碍树实证，判 **可达真缺陷 1 项：P2**（SC 3.3.2 Labels or Instructions / 1.3.1 Info and Relationships / 4.1.2 Name, Role, Value 综合）。

- **P2（6 个控件 AX name=null）**：输入文件 Select 的 combobox、输出文件原生 input、质量检查/智能去重/断点续传/异步引擎 4 个 antd Switch。DOM 的 label 文本与控件之间无 for/id、无 aria-label/aria-labelledby 关联，辅助 tech 读到 role 但读不到名称。
- **正常 0（去伪，不硬凑缺陷）**：任务 ID 的 antd Input 虽源码只有 label+placeholder，但 antd 内部已用 placeholder 自动补名，CDP 实证 partial/full AX name 均为"增强任务的任务 ID"，故从 P2 名单剔除。

## 立项性质与争议披露（Fail Loud）

label 关联、placeholder 作可访问名的甄别维度，在 oa4rust 桌面/H5/admin 侧已由 R201/R213/R214/R217/R225/R235/R536/R538 反复甄别修复。本例 **不是首次发现该维度**，而是该维度在 augmentor/web（React 19 + antd 6 + Vite）新子项目的 **新实例 + 局部疏漏信号**。antd 对 Input 用 placeholder 自动补名、对 Switch/Select 不补名，正是需要跨框架据实验证的差异点。

## 范围、入口与可达性

- 入口：App.tsx 菜单 key=/augment（label=数据增强），Route /augment=Augmentation，零登录一跳可达；main.tsx 仅 React StrictMode，无全局 a11y 增强器。
- 可达性分层：输出 input、任务 ID Input、4 个 Switch 为页面常驻控件，零后端造数即可探测；输入文件 Select 的 Option 依赖 getDataFiles（/api/data/list），经 stub 造 1 文件后展开探测，属轻造数据实披露。
- 源码锚点（Augmentation.tsx）：输入文件 label:209→Select:210-221；输出文件 label:224→原生 input:225-235（无 id/for/aria-label）；4 个 Switch label:239/243/247/251→Switch:240/244/248/252；任务 ID label:340→antd Input:341-346（placeholder=增强任务的任务 ID）。

## 方法与决定性证据

Playwright + CDP DOM.enable/Accessibility.enable，viewport 1440x900 DPR1，3 组（default/injected/positive）+ 补探针 2 组（select/positive-dm），全部 fatals=0、pageerror=0。用 DOM.querySelectorAll→DOM.describeNode 取 backendNodeId→getPartialAXTree/getFullAXTree 读 role/name/ignored。

**default 组（原生态）**：
- 输出文件 input：attrs role/tabindex/aria/labelledby/id/for/placeholder 全 null；partial+full AX role=textbox name=null ignored=false。
- 任务 ID antd Input：placeholder=增强任务的任务 ID，attrs aria/id/for 全 null；partial+full AX role=textbox name="增强任务的任务 ID"（placeholder 自动补名，去伪）。
- 4 个 Switch：tag=BUTTON，attrs role=switch、aria/labelledby/id/for 全 null；partial+full AX role=switch name=null ignored=false。
**补探针 select 组**：Select 内部 input（选择器 .ant-select input）tag=INPUT，AX role=combobox name=null ignored=false；展开后 Option（.ant-select-item-option）AX role=generic name=null。坐实 antd Select 未自动补名，判 P2。
**injected 组（注入对照）**：对 outfile/taskid/4 switch 注入 aria-label=R577-inj-* 后，AX name 实时变为注入值（partial/full 一致），证控件可经 aria-label 修复、非框架硬限制。
**positive 组 / positive-dm 组（正面对照）**：DataManagement.tsx 带 aria-label 控件 input[aria-label="转换输出文件名"] AX role=textbox name="转换输出文件名"、input[aria-label="分割输出目录"] name="分割输出目录"（均 ignored=false），证 AX 通道正常、name 可读；另页首 8 个 antd 图标 span[aria-label] AX role=image name 可读（dashboard/database 等）。
**交互对照**：点击异步引擎 Switch，checked false→true、class 增 ant-switch-checked，证控件非死代码；Select 展开后 Option R577-input.json 可见可用。

## 等价入口与排除

全页 grep 无其他原生 button、无 aria-labelledby、无 for/id 关联可替代；6 个无名控件各自仅有该 label 视觉标注，无等价可访问入口。任务 ID 的 placeholder 补名经 CDP 坐实有效，不构成缺陷。

## 最小修复方向（业务零改）

对 6 个无名控件逐一补 aria-label（或 label 与控件 for/id 关联）：Select 加 aria-label="输入文件"、输出 input 加 aria-label="输出文件"、4 个 Switch 各加 aria-label=对应 label 文本；均可复用现有 label 文案，业务逻辑与 state 零改。

## 证据 sha256 表

| 文件 | 字节 | sha256 |
| --- | --- | --- |
| r577-report.json（主报告） | 34627 | dc27085a645fddf5918695d8968f60ecc163cc5055a2dc22104015259b0d72a6 |
| r577-sup-report.json（补报告） | 941 | 76f2d47151556b600cbe5d069e999acc73d1eb4db5e50f2205ccf6c32983f7ab |
| r577-default.png | 48576 | b71ede4b38a18a2eca00c0a29170b48b40422597a9a5cec00a08340a84499c21 |
| r577-injected.png | 48576 | b71ede4b38a18a2eca00c0a29170b48b40422597a9a5cec00a08340a84499c21 |
| r577-positive.png | 237295 | be931d8e4c1696fd6f4320ba737cff92e1e690b55bf01fe0dad38c53cf66cc66 |
| r577-sup-select.png | 52846 | c7d759b2e797606d53c0f7529cfde065deb923013e79cf6f6f3557fe12a91235 |
| r577-sup-dm.png | 237295 | be931d8e4c1696fd6f4320ba737cff92e1e690b55bf01fe0dad38c53cf66cc66 |
| Augmentation.tsx | 15025 | 87677f7e68c25d6039d57ccf1532d6d89a03ad6b412b7b384aca5b9a0775869b |
| DataManagement.tsx | 48524 | 8c179ae5ab58a85abdcd1fa9a677e535ccacfd45ec5902acf6551805576364a8 |
| Augmentation.test.tsx | 19958 | f894c0c281edbbb8da926ac5a95f428f662039dd0e7ea031c2f693c182330de8 |
| App.tsx | 4945 | f6e245c48c66be157899b98c087aecb1c4e550fecce8e4058fa53d6a383493b4 |
| api.ts | 75161 | 2534a96282729c71c528a4b5602e6a66c8f8cd348ce42e9100e5fa7561b3130c |

runner 紧凑 JSON sha（与文件字节 sha256 分列，不可混用）：主 381cf8d654aa5557a3e5d3ebd022b94c550bed53b22be8504afa28aa20a03247；补 c01f2f538dcb0f6e962f3aa67fec039a07244aa308a3ad3f224d353461546868。

证据文件（gitignored，借用 oa4rust/test-results/r577/，未入库）：含主/补探针 .cjs、解析脚本及上述报告与截图。服务基址用占位 <TEST_HOST>:<PORT> 表述，未写入任何真值域名/端口/路径。
