# OA4Rust R536 H5 其余表单页可访问名只读甄别报告

- 轮次：R536（台账 §527），真父 `057c2389658f185a1835c9b8c19409b79b507447`，基线已 push。
- 性质：**只读甄别**，不改源码、不修业务；证据以 `<TEST_HOST>` 占位域名脱敏，0 真值入库。
- 范围：`oa4rust/mobile/`（uni-app H5）除 R534 已覆盖的 calendar 与 bbs/new 外的表单控件：登录、搜索、通讯录、聊天、我的（改密）、考勤（假期申请）、论坛回帖、发起流程。WCAG 2.1 SC 4.1.2 Name, Role, Value（A），关联 SC 3.3.2 Labels or Instructions（A）。
- 方法：本地 `pnpm dev:h5`（本地 Vite 开发服务，hash 路由）+ Playwright Chromium headless 移动视口 390×844 DPR2；CDP `DOM.focus(nodeId)` + `Accessibility.getPartialAXTree` / `getFullAXTree` 直取 AT 实际播报的聚焦节点；认证与列表数据用 stub 占位（0 真值入库）。

## 一、结论

**可达真缺陷 1 项**（按控件族与页面聚合，非按控件个数重复计）：WCAG 2.1 SC 4.1.2 Name, Role, Value（Level A）—— H5 其余 9 页的 13 个表单控件在 AXTree 中 role 存在（textbox / searchbox）且 focusable，但 **可访问名 name 全部为 null**；placeholder 与前置兄弟 `<text class="label">` 均未被 uni-app H5 WebView 作为可访问名。关联 SC 3.3.2 Labels or Instructions（Level A）：标签文本与控件仅 DOM 兄弟关系，无 `for/id` 或 `aria-label/aria-labelledby` 编程关联。

决定性去伪已实证：在同一环境对登录输入框注入 `aria-label="账号输入R536对照"` 后，同一节点 AX name 从 null 变为「账号输入R536对照」（报告 `r536-aria-inject-control.json`）——证明 AX 命名通道正常、缺陷是控件本身缺名，而非采集假象；注入前 placeholder「工号 / 用户名」未被计入可访问名，证伪「placeholder 作名」抗辩。

## 二、源码甄别（逐行锚点）

| 锚点 | 源码事实 |
|---|---|
| `oa4rust/mobile/src/main.ts:16-67` | `installH5NativeInputNameA11y` 增强器：:21 `host.getAttribute('aria-label')`，:22 `if(!label)return`，:82 调用；只透传宿主已有 aria-label，源码仅 placeholder 则宿主 name=null，增强器不生效 |
| `login/login.vue:49-50 / :53-54` | 账号 `<text class="label">账号</text>` 与 `<input v-model="credential" type="text" placeholder="工号 / 用户名" />`；密码 label 与 `<input type="password" placeholder="密码" />`；无 for/id / aria-*；:16-22 onShowGuard 已登录才 navigateHome，未登录可达最浅入口 |
| `search/search.vue:88-94` | 搜索框 `placeholder="搜索文档、主题、数据…"`，无编程名 |
| `contacts/contacts.vue:195-202` | 通讯录搜索框仅 placeholder，无编程名 |
| `chat/chat.vue:266` | 发消息输入框仅 placeholder，无编程名 |
| `mine/mine.vue:134 / :141 / :148` | 修改密码三框（旧 / 新 / 确认）仅 placeholder，无编程名 |
| `attendance/attendance.vue:231-232` | 假期申请 `<input class="leave-input" placeholder="假期日期（如 2026-10-09）" />` 与 `placeholder="事由（选填）"`，无编程名 |
| `bbs/topic.vue:113` | 回帖框仅 placeholder，无编程名 |
| `process/start.vue:302 / :326-334` | 标题框 `<input class="input" placeholder="工作标题（必填）" />`；动态文本字段 `<input class="input" :placeholder="f.placeholder || '请输入'" />`，label 无 for/id；:377 label 包 checkbox 在本次空态未渲染（见取证限制） |

源码 SHA-256：

| 文件 | SHA-256 |
|---|---|
| `oa4rust/mobile/src/pages/login.vue` | `d3d654fe5b89995f27ec6e8a9aa431ce230f2397273d410dffea10e3db1eb476` |
| `oa4rust/mobile/src/pages/search.vue` | `154e64f0b89da6959f64132a789d6000c3a0125d2f3a329b9163015e62f906ef` |
| `oa4rust/mobile/src/pages/contacts.vue` | `9e3ec279c18cc579b3030f82f6f8d81e40e296dc9a81469989a83281717005ab` |
| `oa4rust/mobile/src/pages/chat.vue` | `3e7db80553ce33680ca56b934f4569e895b6612223081054697aac30c8dafb46` |
| `oa4rust/mobile/src/pages/mine.vue` | `245b23b1917b0be61be2646a918aa5447f36a51307bdd6831653ee1334578c9d` |
| `oa4rust/mobile/src/pages/attendance.vue` | `91d6fa6251769af651c78deafe888c1bc35d1cb5ce4497814ea32984db8202fd` |
| `oa4rust/mobile/src/pages/topic.vue` | `637af34300977419cdcab1d65cc3a7223f8dfba1756df402fb80ccffc4e5242c` |
| `oa4rust/mobile/src/pages/process/start.vue` | `d77d1b43fb0e07d9dd1865b53dbbbceb2433dfe14f55fbf8e2c61e19e0b2de55` |
| `oa4rust/mobile/src/pages/main.ts` | `b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b` |

## 三、决定性 CDP 证据（4 组 light/dark × anon/auth，fatals=0 / pageErrors=0）

| 页面 / 控件 | 聚焦节点 | CDP partial + full AX |
|---|---|---|
| login credential / password（anon） | 原生 INPUT `.uni-input-input` | `role=textbox name=null ignored=false focused=true`；DOM 无 aria-label / id |
| search keyword / contacts keyword（auth） | 原生 INPUT | `role=searchbox name=null ignored=false` |
| chat message / topic reply（auth） | 原生 INPUT | `role=textbox name=null` |
| mine oldPwd / newPwd / confirmPwd（auth） | 原生 INPUT ×3 | `role=textbox name=null` ×3 |
| attendance leaveDate / leaveReason（auth，补探针 v3） | `.leave-input input.uni-input-input` ×2 | `role=textbox name=null` ×2；DOM 无 aria-label |
| process/start title（auth，补探针 v3） | `.card .input input.uni-input-input` | `role=textbox name=null` |
| **正面对照（注入）** | login credential 同节点 | 注入前 `textbox/null` → 注入 `aria-label` 后 `textbox/name="账号输入R536对照"`；证明通道正常、修复方向有效 |

证据文件（均入 gitignored `oa4rust/test-results/r536/`）：

- `r536-v2-report.json`（决定性 4 组，13 控件 + 双通道 AX）SHA-256 `86389e84509f64abf9ac944b2d290abe9998510d6cdae43ebca117279afee5a4`。
- `r536-v3-report.json`（attendance/processStart 补探针，2 主题）SHA-256 `019593846717f212d27aaa8569431ad213597d79f131c71929114a4eaa312a4a`。
- `r536-v4-verify` 衍生验证：fullAXTree 非空名节点（含「账号」「密码」「登 录」StaticText）证明文本通道正常；`r536-button-ax.json` SHA-256 `ae251059f321a4077ab01ff1e5b35020e6d0797970d1b5b836c47dbbb8227e70`。
- `r536-aria-inject-control.json`（注入前后对照）SHA-256 `f1536d21c09b7f5bb24591df6a8fa0ab98ddef2b58481c7c7fb3378d1dd61ba1`。
- 截图（移动 390×844 DPR2）：v2 每页每主题 `r536v2-*.png`（sha 见报告）；v3 `r536v3-attendance-light.png` / `r536v3-attendance-dark.png` SHA `b5ce3ea3855580db8c01309c3b01d4d6f6fe29da7d9c843ce5e5aac0c827a069`（H5 无主题机制，light/dark 一致）；`r536v3-processStart-light.png` / `r536v3-processStart-dark.png` SHA `7f710518bd7786e6407cbd2ed44e9a85e3e92ea03143e733adb6573c3f69b43f`。

## 四、通过项与取证限制

- 正面对照成立：运行时给控件加 aria-label 可让 AX name 由 null 变为该值，故为真实缺名；fullAXTree 中文本节点（账号/密码/登 录）有名，证明 AX 通道与渲染正常。
- 主题说明：`oa4rust/mobile/src/App.vue` 未实现 `oa4rust_theme` / `data-theme` 切换机制，light/dark 两次采集渲染一致（截图 sha 相同），据实记录，不伪造暗色差异；AX 结论与主题无关。
- stub 说明：认证与列表数据用 stub 占位，仅影响数据源；控件 DOM 与可访问名为真实 Chromium 编译渲染结果。登录页未登录可达（最浅），其余页经 who stub 认证可达，均非空态：attendance 显示假期申请表单（2 个 INPUT），processStart 显示标题表单（1 个 INPUT），均实际渲染。
- 取证限制：process/start.vue:326-378 动态表单字段及 :377 `<label>` 包 checkbox 在本次 stub「暂无可发起的流程」下未渲染（DOM checkbox=0），该动态字段不据本报告判缺陷，须有真实流程数据时另测；这不影响已坐实的 13 个可达控件。
- 方法论教训：uni-app H5 的 placeholder 未反射到原生 INPUT 的 DOM placeholder 属性，且不进入可访问名；不能静态以源码 placeholder 推断 AX name，也不能用 wrapper（UNI-INPUT，role=generic focusable=false）代判，必须 DOM.focus 到原生 INPUT 后读 AX。

## 五、三分判定

- **可达真缺陷（P2，SC 4.1.2 A / 关联 3.3.2 A）**：9 页 13 控件族缺名，真浏览器可达、非空态非死代码；AX name=null 已坐实，注入 aria-label 后 name 可读反证通道正常。
- **不可达 L1**：排除——登录未登录可达，其余经认证 stub 可达，各控件均聚焦命中原生 INPUT。
- **正常**：排除——placeholder 与兄弟 label 未提供可访问名（AX 双通道实证），注入 aria-label 立即可读，不能判正常。

## 六、最小修复方向（业务零改，仅补可访问名）

- 给上述 13 控件补 `aria-label`（如「账号」「密码」「搜索关键词」「消息内容」「旧密码」「新密码」「确认密码」「假期日期」「申请事由」「回帖内容」「流程标题」「字段输入」）；或为 `<text class="label">` 与控件建立 `id` / `aria-labelledby` 关联。
- 与 R537 增强器协同：`installH5NativeInputNameA11y` 只透传宿主已有 aria-label（labelHost 无则 return），故需在源码控件上补名，增强器方可传播；或扩展增强器用 placeholder / 兄弟 label 文本兜底（需评估副作用）。业务函数、v-model、模板结构与样式零改。

## 七、覆盖甄别

- 与已覆盖维度区分：本项聚焦 H5 表单控件可访问名，与 R534（calendar / bbs-new 控件族）、PC 侧 4.1.2 实例、R523 链接目的、R524 面包屑 current、R525 Reflow、R526 嵌套 interactive 均不同页不同控件，不重复计缺陷。
- 同页正面对照与注入对照双重成立，排除「AX 采集失效」假阳性；placeholder 不作名双次实证，与 R534 一致。
- 动态字段与 checkbox 因空态未渲染，据实标注未坐实，不臆造缺陷。