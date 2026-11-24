# R490 WCAG 2.1 AA 3.3.2 表单必填标识语义甄别

## 1. 任务与范围

本轮为只读甄别（不改码），依据台账 §485 据实立项。维度严格收敛为 WCAG 3.3.2 Labels or Instructions 的「必填字段的必填标识语义」子集：只甄别必填字段是否声明 required/aria-required 或可见必填标识（星号/必填文字），以及 disabled 保存按钮旁是否有可见必填说明。

明确排除已被历史轮次覆盖的区域，不重复判缺陷：

- labelA11y.ts（R213，main.ts:618 安装）：为孤儿 label 补 for/id，坐实可访问名——本轮实测已生效（见 §4）；
- searchFieldA11y.ts（R217，main.ts:621 安装）：为搜索框补 aria-label；
- formErrorA11y.ts（R489，main.ts:614 安装）：错误态 role=alert/aria-invalid 关联。

本轮不接受「全库 aria-required 命中 0」这类源码统计作为判据；所有结论均由真浏览器 DOM getAttribute + CDP Accessibility.getPartialAXTree 实证。

## 2. 环境与方法

- 真值来源：测试服经环境变量注入（本文档只写占位符 <YOUR_TEST_DOMAIN> / <TEST_HOST>，0 真值入库）。
- 浏览器：Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，非物理真机（如实标注，不冒充真机）。
- 采集：DOM 取 required 属性、aria-required、label 关联、可见值、按钮 disabled；CDP Accessibility.getPartialAXTree 取 axRole/axName/axIgnored/required 属性。
- 实测范围：three-member 新建弹窗、form 新建弹窗（模板预填）、calendar 新建日历表单（原生 required 正面对照）、process 发起流程弹窗（XformRuntime 正面对照，运行态未绑定表单）。共两轮：主轮 8 组 + 补采 2 组，0 pageerror。

## 3. 静态侦察（决定性结构）

- 全 views 下 aria-required 显式命中 0；原生 required 仅 CalendarApp.vue:63 / 132-134 共 4 处。
- ThreeMemberApp.vue:39-43 姓名/手机/邮箱/单位/状态 label 无 for、无必填标识；:46 保存按钮 disabled=!form.name?.trim() || saving，姓名为事实上必填。
- FormApp.vue:101-109 新建表单弹窗 名称/标识/分类/JSON Schema 无 for/required；:112 创建按钮 disabled=!mform.name || saving（经模板预填全部字段，实测创建按钮 enabled）。
- XformRuntime.vue:13 required 星号 aria-hidden=true（视觉标识）；:20/:29/:55 :required=module.required（原生 required 声明）：工作流表单必填字段同时具备原生 required 语义与可见星号，属规范预期（运行态未能在真实 UI 命中绑定表单的流程，正面对照以源码为准，非缺陷）。
- CalendarApp.vue:63 日历名称原生 required + aria-label；:132-134 日程表单三字段原生 required。

## 4. 逐组实测证据（与 report.json / supplement.json programmatic 一致）

### 4.1 主轮 8 组（report.json）

| 组 | 弹窗 | 控件 | required prop | aria-required | AX required | AX name | label 已关联 | 保存按钮 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| light/three-member | 新建三方成员 | 姓名/手机/邮箱/单位/状态 | 全 false | 全空 | 全 false/- | 姓名/手机/邮箱/所属单位/状态 | 全 true（R213） | 保存 disabled=true |
| dark/three-member | 同上 | 同上 | 全 false | 全空 | 全 false/- | 同上 | 全 true | 同上 |
| light/form | 新建表单 | 名称/标识/分类/JSON Schema | 全 false | 全空 | 全 false/- | 名称/标识/分类/JSON Schema | 全 true | 创建 enabled（模板预填） |
| dark/form | 同上 | 同上 | 全 false | 全空 | 全 false/- | 同上 | 全 true | 同上 |
| light/calendar-required | 新建日历表单 | 日历名称 | true | 空 | true | 日历名称 | aria-label | 创建日历 enabled |
| dark/calendar-required | 同上 | 同上 | true | 空 | true | 同上 | aria-label | 同上 |
| light/xform-required | 发起流程 | 流程/标题 | false | 空 | false/- | 流程/标题 | 全 true | 发起 disabled=true |
| dark/xform-required | 同上 | 同上 | false | 空 | false/- | 同上 | 全 true | 同上 |

补充：calendar 组颜色控件 axRole=ColorWell、label「颜色（可选）」，非必填不适用；全局搜索框 axName=全局搜索（R217）已排除；xform 组流程/标题为 shell 字段，必填判定为 disabled 按钮隐式表达（与 three-member 姓名同型），运行态所选流程均未绑定表单（截图显示「当前流程未绑定表单」）。

### 4.2 补采 2 组（supplement.json）

- form：模板预填值 name=人员信息 / flag=人员信息_form / category=biz / schema 为完整 JSON，hasRequiredWord=false；用户无需输入必填字段，创建按钮 enabled——不适用「必填标识缺失」。
- three-member：姓名/手机/邮箱/单位均 value=空，hasRequiredWord=false，labelHTML 均为纯文字（无星号/必填）；AX required 全 false。
- XformRuntime：light/dark 各尝试 6 个流程，fieldCount=0（所选流程未绑定表单），运行态未能命中正面样本；源码 XformRuntime.vue:13/20/29/55 的 required 语义作为规范预期依据，不判缺陷。

## 5. 三分判定

- **可达真缺陷 1 项（P2）**：ThreeMemberApp.vue 新建成员弹窗姓名字段事实上必填（:46 disabled=!form.name?.trim() 坐实），但字段无 required/aria-required、无可见必填标识（* 或「必填」文字），保存按钮旁无「姓名必填」说明，CDP AXTree required=false、labelHTML=纯文字、双主题实证 hasRequiredWord=false。读屏用户与视觉用户均无法在提交前预先知悉该字段为必填，只能从按钮禁用状态反推，违反 WCAG 3.3.2「须在输入前告知必填要求」。表单的 XformRuntime 已正确实施 required+星号，说明修复有现成范式。
- form 弹窗经模板预填全部字段（补采实证 value 非空、按钮 enabled），无用户必填字段——**不适用**，不判缺陷。
- calendar 新建日历/日程字段原生 required + AX required=true——**正常规范预期**（正面对照）。
- XformRuntime 工作流表单 required+可见星号（aria-hidden 星号由原生 required 提供 AX 播报）——**正常规范预期**（源码对照）。
- label/可访问名/search/error 增强器覆盖区——**不可达 L1 0 项**（历史轮次已覆盖，本轮不重复判）。

精确修复建议（供后续轮次，本轮不改码）：ThreeMemberApp.vue:39 姓名 input 增加 required（或 aria-required=true），并在 :39 label「姓名」旁增加可见星号或「必填」文字（与 XformRuntime.vue:13 范式一致）；如偏保守不改业务属性，至少在按钮旁增加可见「姓名为必填项」说明并让读屏可及。手机/邮箱/单位/状态为可选字段，维持现状即可。

## 6. 截图 sha256（命令行核验，非物理真机）

| 截图 | sha256 |
| --- | --- |
| pc-three-member-light-r490.png | 99ac76222918b0dbf9dc3fa112db775be01f4ed75ad0fbb04bb5c6ab740b1b29 |
| pc-three-member-dark-r490.png | c34c494b0c0f6015cbb2fec680b81a8c469901c2c6f3cc0bee1e8d162366237e |
| pc-form-light-r490.png | 93450e1ca5791d76420824df49dd6e1d5c9903abcca1e138562b25d02887f26d |
| pc-form-dark-r490.png | 6e788c01e59a7affdf5f8d7baf03ce062706a8a2d3795a96ba59bfd9367a65ce |
| pc-calendar-required-light-r490.png | 1283c70e6db48d4e31f5769be4e009c284cdf551822290987f0a6287cf0a5b56 |
| pc-calendar-required-dark-r490.png | caeb95f8c9009bdb1679978306a6285cd6d7f4e991d487b2f03d3afe44f55c61 |
| pc-xform-required-light-r490.png | 5ee2dd13ebc3f2c0827e69446647e90b7bf3c3f3d7737303d7b36ab6c1543aef |
| pc-xform-required-dark-r490.png | a5413df56a182c24affdeaefb48f9e2a78850f270fc33f348fb6c6c8d36d2dcd |
| pc-form-prefill-light-r490.png | 93450e1ca5791d76420824df49dd6e1d5c9903abcca1e138562b25d02887f26d（与主轮 form 同页同态，sha 相同） |
| pc-form-prefill-dark-r490.png | 6e788c01e59a7affdf5f8d7baf03ce062706a8a2d3795a96ba59bfd9367a65ce |
| pc-three-marker-light-r490.png | 99ac76222918b0dbf9dc3fa112db775be01f4ed75ad0fbb04bb5c6ab740b1b29（与主轮 three-member 同页同态） |
| pc-three-marker-dark-r490.png | c34c494b0c0f6015cbb2fec680b81a8c469901c2c6f3cc0bee1e8d162366237e |
| pc-xform-sample-light-r490.png | cec08b51c35287156de5db01c0ab70be6907f0c99e15a86362b2c09482a6ee87 |
| pc-xform-sample-dark-r490.png | 3748489ef4b3c245edca1e751fa1bc8a653de4140c5b94923e7dd6db5189f83b |

截图目视（1440×900）：three-member 新建弹窗 5 字段 label 纯文字无星号，保存按钮灰显；form 新建弹窗由模板预填内容；calendar 新建日历含日历名称与「颜色（可选）」；xform 弹窗显示「当前流程未绑定表单」。全部为非物理真机 headless 截图，名实相符。

## 7. 脱敏

本文档无真实域名/IP/口令/私钥/个人路径；测试服地址仅以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，凭据由 runner 经 ssh 运行时注入，不写入仓库。证据文件（截图/report.json/supplement.json/分析 txt/脚本）位于 gitignored oa4rust/test-results/r490-instructions/ 与 r490-*.cjs|sh。