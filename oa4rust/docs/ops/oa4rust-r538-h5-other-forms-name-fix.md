# R538 H5 其余表单控件可访问名修复（WCAG 2.1 SC 4.1.2 / 3.3.2）

- 轮次：R538（repair，t41），真父 `697e03af106472db9d4c96637c97829cf7109a20`（台账 §528，已 push），父 `331e09ae3b153f54b5d52c213f4cb787f75e87ff`。
- 缺陷：R536 判 1 项 P2——H5 其余 9 页 12 个可达表单控件聚焦原生 `INPUT.uni-input-input` 的 AX name 全为 null；placeholder 与前置兄弟 label 均未被 uni-app H5 作为可访问名。
- 决定性去伪：R536 注入对照（`r536-aria-inject-control.json`）已证 AX 命名通道正常、真实缺名。

## 一、方案取舍（Fail Loud）

队长要求二选一并据实披露：
- **方案 A（扩展 main.ts 增强器用 placeholder/兄弟 label 兜底）不可行**：R536 CDP 实证所有控件原生 `activePlaceholder=null`、宿主 `hostAria=null`；uni-app H5 把源码 `placeholder` 编译为 `div.uni-input-placeholder` 文本节点（非原生 placeholder 属性），现有 `installH5NativeInputNameA11y` 的 `labelHost` 读不到宿主 aria-label 时 `return`，兜底无来源。
- **方案 B（逐页显式补 aria-label）已采用**：与 R535 同模式，源码 8 个 vue 共 12 处仅追加 `aria-label`，由 R537 既有增强器把宿主 aria-label 透传到真正被聚焦的原生 input。`main.ts` 本轮零改，R537 透传逻辑与 t35 宿主标记完整保留。

理由：方案 B 来源确定、最小外科改动、业务零改、可直接复用已验证的透传链路；方案 A 在当前编译产物下无可读取的兜底文本，强行实现需遍历兄弟 DOM 猜 label，脆弱且越界。

## 二、写域（实际改动 10 文件）

契约 inScope 列的是扁平路径（`login.vue` 等），实际仓库在 `pages/<page>/` 子目录，按真实路径修改并在此披露。

| 文件 | 变更 |
|---|---|
| `oa4rust/mobile/src/pages/login/login.vue` | :50 账号、:54 密码 各补 aria-label |
| `oa4rust/mobile/src/pages/search/search.vue` | :89 搜索关键词 |
| `oa4rust/mobile/src/pages/contacts/contacts.vue` | :197 通讯录搜索关键词 |
| `oa4rust/mobile/src/pages/chat/chat.vue` | :266 消息内容 |
| `oa4rust/mobile/src/pages/mine/mine.vue` | :134/:141/:148 旧密码/新密码/确认密码 |
| `oa4rust/mobile/src/pages/attendance/attendance.vue` | :231/:232 假期日期/申请事由 |
| `oa4rust/mobile/src/pages/bbs/topic.vue` | :113 回帖内容 |
| `oa4rust/mobile/src/pages/process/start.vue` | :302 流程标题 |
| `oa4rust/mobile/src/pages/R538.h5OtherFormsName.contract.test.ts` | 新增，10 用例源码级契约 |
| `oa4rust/docs/ops/oa4rust-r538-h5-other-forms-name-fix.md` | 本文档 |

业务零改：`v-model` / `@tap` / `@confirm` / `@change` / `placeholder` / handler 函数 / 模板结构 / 样式 / 他页 / R535 `:295` `role=button aria-label=新建日历 @tap=toggleCalendarForm` 正面对照逐字未改。git diff 实证 8 文件仅 12 行 aria-label 追加（12 insertions / 7 deletions 为整行替换 diff 计数），无业务属性变动。

**动态字段不纳入**：`process/start.vue:326-378` 动态文本字段（` :placeholder="f.placeholder || '请输入'` `）与 :377 label 包 checkbox 因本次 API stub 空态未渲染，据实不纳入本轮判定。

## 三、契约 RED → GREEN

契约 `R538.h5OtherFormsName.contract.test.ts` 10 用例：4 核心（各页精确 aria-label）+ 1 核心（aria-label 精确唯一防重复污染）+ 5 守卫（R537 增强器零改 / v-model 业务事件 / 各页事件与占位符 / R535 calendar:295 正面对照与他页零改 / aria-label 不覆盖 placeholder）。

- 真父 `331e09ae3b` 覆盖 8 vue → `pnpm test` 定点 exit=1，5 failed（核心全 fail：无 aria-label）| 5 passed（守卫全过）。
- 恢复修复态 → exit=0，10 passed。日志 `r538-red.log` / `r538-green.log`。

## 四、三 verify（全 exit 0，workdir=oa4rust/mobile）

| # | 命令 | 结果 | 日志 |
|---|---|---|---|
| V1 | `pnpm test src/pages/R538.h5OtherFormsName.contract.test.ts` | exit 0，10 passed | r538-green.log |
| V2 | `pnpm test` | exit 0，全量通过 | r538-verify2.log |
| V3 | `pnpm typecheck` | exit 0，tsc --noEmit | r538-verify3.log |

## 五、决定性 CDP 取证（t36/t42 核心判据：focused=true 非 root 节点 AX name 非空）

- runner `oa4rust/test-results/r538-e2e.cjs`，Playwright chromium headless，390x844 DPR2 isMobile/hasTouch，自起 uni H5（<TEST_HOST>），stub 认证/列表（0 真值入库）。
- 判据口径：`getFullAXTree` 中 `prop(n,'focused')===true` 且非 `RootWebArea` 的节点 AX name === 预期文案；`document.activeElement` 为 `INPUT.uni-input-input`（`isNativeChild=true`）。**未探宿主 uni-input wrapper**（role=generic focusable=false 假阳性，R535 t36 教训）。CDP 的 focused 在 `n.properties[]`，非顶层字段。
- 结果 `R538_E2E_DONE`：`names=18 namesBad=[] pageErrors=0`。18 条 = 9 个唯一控件 × light/dark 两主题，AX name 全部非空且等于预期：账号、密码、搜索关键词、通讯录搜索关键词、消息内容、旧密码、新密码、确认密码、回帖内容。
- 注入对照：登录框移除 aria-label 后 focused name=null → 注入 `R538注入对照` 后 focused name=`R538注入对照`，证明采集通道有效、缺陷是真实缺名而非探针失效。
- 正面对照：登录 `.submit` 与 R535 `:295` 按钮 role/name 保留，业务零回归。

### Fail Loud：4 个 page 级 fatal（采集阶段，非产品控件缺陷）

light/dark 各 attendance + processStart 共 4 个 page 级 fatal：`Cannot read properties of undefined (reading '0')`。根因为 runner 判据代码在聚焦异常页读取 `a.focusedAX[0]` 时该页未采集到焦点节点（attendance 的 `input.leave-input`、processStart 的 `.card input.input` 选择器在 stub 空态下命中不稳），属**采集/截图阶段**异常，非控件判据失败、非产品缺陷。据队长裁决如实披露，不强求 fatals=0。

这 3 个控件（假期日期、申请事由、流程标题）的 aria-label 已在源码施加（`git diff` 可查），并由契约核心用例与守卫钉死；其余 9 控件已由 CDP 决定性判据直接坐实 name 非空。0 pageerror 贯穿全部 4 组。

### Fail Loud：H5 无主题切换

uni-app H5 App.vue 无 `oa4rust_theme` / `data-theme`，light/dark 各组截图 sha 相同，据实记录、不伪造暗色差异。

## 六、证据 sha256

| 产物 | sha256 |
|---|---|
| login.vue | d89baaf6e8daa591d68c48e51e5e27f715ccaba8f6f88a05e6415d577ad57b2d |
| search.vue | b624eda33b3d7c268124b9529ee32a0eb49dab487b7ff4a15127b033eadfa74e |
| contacts.vue | b23e6e9ff0adfab2544ec05ed5d6ab371e5329d6c24ce50c7b4f618b35a911c3 |
| chat.vue | 8a0aeb68b466bec6e293a4adfd090f4688fe2ab429f902c41eb92632d70d2993 |
| mine.vue | 383156150783a55d3e1b7f718aa76718ece3e549980e4021f2565b0cebf87efc |
| attendance.vue | b48c036aba02ecec81813be699f18dd9a7bd8cfb04419e962dfd0e9a2ff10cd8 |
| bbs/topic.vue | b2c8fd9752d18b623e3af817f9168a95e6163c23362a73cfa27ce7cd1f4ec452 |
| process/start.vue | 5d09a10582de25936b071b7e27a749550ccb9a716cadd63c9b26717d1de68111 |
| R538 契约 | 5d94dbe59469d55cc57fb0e0d1370dc34489661b6719cef9d3db885adb065bc8 |
| r538-e2e.cjs | 8b0d9b3b7a35fdb6ebaa1da2d3df7aa28f796e1f042a3b89c522247d937849a7 |
| r538-ax-report.json | 30276bd8372bf5295596fe812ab7850c977f47b933eae0507bb78a0a2504189c |
| r538-red.log | b57066a21b34263f8cbbd7f33068647ab3ef6cbc2cf69f454a9932ae5701a7ec |
| r538-green.log | 7720d556a5b0d3b49249c897efe6e84c2e7c525cbc429d59b9dffd960397da62 |
| r538-verify2.log | bf26dcc7d4f7509bd97670184bc69f9b7d5d293896fe4ee993b95ca0d1ff1174 |
| r538-verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| r538-login.png（light=dark） | 4fb48d6306ce5237555682416d26c36308db92ec20f5d2d6341878f282ac7eb1 |
| r538-search.png（light=dark） | 1b3111c43331fb7924aa953b0de1c55597d281c5c572d8a287138b579c839d55 |
| r538-contacts.png（light=dark） | bcb674224fa4b786e63ed167de73189c28f3bbfd347b36eba3c85482843a1d09 |
| r538-chat.png（light=dark） | b61429178e6a223199f0a7ca85c3f471279cd689e709980f7a4c25ca02ba3c99 |
| r538-mine.png（light=dark） | 605945019370c10e3ae77b116e425650101631046902d44a6f0f20587025fc29 |
| r538-topic.png（light=dark） | 73d77171af365a1adee5061d3e1b81f7b0967b7ff6236effd65f4ef2465d7058 |

截图均 780x1688（390x844 @DPR2），login/chat/mine/topic 4 张已目视正常：登录页账号/密码框、聊天页输入框与发送、改密页三框、回帖页输入框与发送按钮均渲染完整，aria-label 不影响视觉布局。runner/日志/报告/截图位于 `.gitignore` 排除的 `oa4rust/test-results/`，不入库。

## 七、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 `<TEST_HOST>` 占位，认证与列表数据为 stub 占位，0 真值入库。
