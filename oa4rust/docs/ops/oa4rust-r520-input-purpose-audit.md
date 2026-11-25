# R520 只读甄别：WCAG 2.1 AA 1.3.5 Identify Input Purpose（autocomplete）

- 类型：只读甄别（不改码、不提交源码）
- 真父：447ce21d2dbc425ecc5efbb5879ef891e321603b（台账 §512）
- 目标：PC 1440×900 headless 双主题下 ThreeMemberApp 新建/编辑三方成员模态框姓名/手机/邮箱输入、ProcessDesigner 流程设计器评论面板作者姓名输入，SC 1.3.5 Identify Input Purpose（WCAG 2.1 AA，Appendix A name/tel/email）
- 运行环境：Playwright Chromium headless，1440×900 DPR1，light/dark 双主题，真浏览器 + CDP DOM.getAttribute/property + Accessibility.getPartialAXTree；个人中心改密表单作同环境正面对照
- 脱敏：报告与日志中的本地开发地址统一替换为占位符 <LOCAL_DEV_BASE>/<LOCAL_HOST>，不入库任何真实域名/IP/凭据；runner 脚本、日志、截图与完整证据 JSON 均位于 gitignored 目录 oa4rust/test-results/r520-*

## 一、结论

**三分判定：可达真缺陷 0 处；不可达 L1 1 处（ProcessDesigner 评论面板作者姓名输入框）；正常 2 类（ThreeMemberApp 三方成员姓名/手机/邮箱输入 + 个人中心改密正面对照）。**

- ThreeMemberApp 新建/编辑三方成员模态框的姓名/手机/邮箱 input 在 light/dark 双主题下 DOM autocomplete 属性均为 null、property 均为空字符串，AXTree 可访问名为「姓名/手机/邮箱」、role=textbox、ignored=false（值为空属正常）。但 WCAG 2.1 SC 1.3.5 Appendix A 的 name/tel/email token 仅适用于「收集用户本人信息」的字段；本表单由管理员录入第三方成员资料，业务语义不属于用户本人信息，严格解释下 SC 1.3.5 不适用，判正常（非缺陷）。R480 曾将其判为可达真缺陷，本轮据真业务语义纠正。
- ProcessDesigner 评论面板作者姓名 input（`newCommentAuthorName`，placeholder=姓名，class=cm-author-input）静态源码 :1812 缺 autocomplete，但真浏览器普通 UI 操作中评论面板打开路径不可达：两张主题的 normal 采集均未见 `.cm-author-input`，triggerProbe.togglesClicked=[]、commentInputSeen=false。仅通过 Vue setupState 强制把 showCommentPanel 设为 true 后才渲染该 input（attr=null/prop=""），证据明确标注 reachability=synthetic-only (Vue setupState forced; NOT reachable by ordinary UI in this build)。依三分口径判不可达 L1，不纳入可达真缺陷。
- 同环境正面对照 Personal.vue 改密表单 #oldPwd=current-password、#newPwd/#confirmPwd=new-password，DOM 属性/property 均正确暴露，排除「框架剥离 autocomplete/CDP 假阴性」辩解。全局无 autocomplete 注入增强器（0 命中），契约 inputPurposeA11y.contract.test.ts:39-61 仅覆盖登录/改密，未覆盖本轮候选。

## 二、源码甄别（逐行锚点）

| 文件 | 行 | 关键代码 | 判定 |
| --- | --- | --- | --- |
| `oa4rust/frontend/apps/desktop/src/views/ThreeMemberApp.vue` | 36 | `<div v-if="showCreate||showEdit" class="modal-overlay" ...>` | 新建/编辑共用模态框，经真实「+ 新建」按钮可达 |
| 同上 | 39 | `<div class="form-group"><label>姓名<span class="required" aria-hidden="true">*</span></label><input v-model="form.name" placeholder="姓名" class="form-input" required /></div>` | 无 autocomplete；label 为裸 label（1.3.1 维度，本轮不混入） |
| 同上 | 40 | `<div class="form-group"><label>手机</label><input v-model="form.mobile" placeholder="手机号" class="form-input mono" /></div>` | 无 autocomplete |
| 同上 | 41 | `<div class="form-group"><label>邮箱</label><input v-model="form.email" placeholder="邮箱" class="form-input mono" /></div>` | 无 autocomplete |
| `oa4rust/frontend/apps/desktop/src/views/ProcessDesigner.vue` | 1808 | `<div v-if="showCommentPanel" class="comment-panel">` | 评论面板受 showCommentPanel 控制 |
| 同上 | 1812 | `<input v-model="newCommentAuthorName" placeholder="姓名" class="cm-author-input" />` | 无 autocomplete；普通 UI 中 showCommentPanel 不可达 |
| `oa4rust/frontend/apps/desktop/src/views/Personal.vue` | 38/43/47 | `<input id="oldPwd" ... autocomplete="current-password" />`、`#newPwd/#confirmPwd ... autocomplete="new-password"` | 同环境正面对照，DOM 正确暴露 |
| `oa4rust/frontend/packages/ui/src/components/LoginScreen.vue` | 27/42 | `autocomplete="username"`、`autocomplete="current-password"` | 契约覆盖的正面对照 |
| `oa4rust/frontend/apps/desktop/src/utils/inputPurposeA11y.contract.test.ts` | 39-61 | 5 个 it 仅断言登录 username/current-password 与改密 3 项 | 未覆盖本轮候选，属覆盖空白非运行时缺陷 |

全项目 views 下 `autocomplete=(name|tel|email)` 静态 0 命中；全局增强器无 autocomplete 注入（0 命中），无兜底机制。

## 三、契约与判据

- WCAG 2.1 SC 1.3.5 Identify Input Purpose（Level AA）：收集用户本人信息的输入字段须用 HTML autocomplete token 标明用途，Appendix A 含 name/tel/email/username/current-password/new-password 等。关键适用性判据：**仅适用于收集用户信息（about the user）的字段**；替他人录入的第三方资料、业务搜索、配置字段不在范围内。
- 三分判定口径：①可达真缺陷＝真浏览器实证适用字段缺 token 且控件经普通 UI 可达；②不可达 L1＝控件不可达/需特殊状态强制才出现；③正常＝token 正确、或 SC 不适用、或语义不构成用户本人信息。
- 反预判：所有结论均来自 headless Chromium 真浏览器 CDP 运行时探针；静态锚点仅用于定位，不以源码推断代替实证；裸 label 属已覆盖的 1.3.1 维度，本轮不混入。

## 四、决定性去伪与语义核对

| 去伪点 | 证据 | 结论 |
| --- | --- | --- |
| ThreeMemberApp 是否收集用户本人信息 | 页面标题「三方成员管理」、表格列姓名/手机/邮箱/单位/状态、按钮「新建三方成员」、`<h3>{{ showEdit?'编辑':'新建' }}三方成员</h3>`，系管理员维护第三方成员名录 | SC 1.3.5 严格解释不适用，判正常；R480 将其判缺陷属适用性误判，本轮纠正 |
| ProcessDesigner 评论署名是否用户本人 name | 普通 UI 下评论面板不可达，且字段值可作昵称/假名；即使强制渲染语义仍不明确 | 不可达 L1，不判真缺陷 |
| 是否框架剥离 autocomplete 导致假阴性 | Personal 改密三框同环境 attr/prop 正确暴露 | 排除框架剥离/CDP 假阴性 |
| 是否 CDP AXTree 读不到 autocomplete | AXTree 仅核对 role/name/ignored；autocomplete 以 DOM getAttribute/property 为准（R480 已确认 AXTree 不暴露该属性） | 方法论一致，无假阴性 |
| 控制台错误是否影响判定 | 两主题各有 3 条 `console.error: Error: <g> attribute transform: Expected transform function, "[object Object]".`，与 autocomplete 无关 | 不纳入 |

## 五、真浏览器取证结果（CDP 四维探针）

Runner：`oa4rust/test-results/r520-input-purpose-runner.cjs`（主采）+ `oa4rust/test-results/r520-forced-comment-runner.cjs`（强制态补采）；证据：`oa4rust/test-results/r520-evidence/evidence.json`（20 个 sha256，1440×900，sourceCommit=真父）；日志 `oa4rust/test-results/r520-*.log`。DOM 探针读 autocomplete 属性/property/全部 input；CDP 探针读 Accessibility.getPartialAXTree 的 role/name/ignored/properties。

| 主题 | 对象 | autocomplete attr/prop | AXTree 摘要 | 可见/可达 | 判定 |
| --- | --- | --- | --- | --- | --- |
| light | three-create 姓名 | null / "" | role=textbox name=姓名 ignored=false required=true | visible=true（真实点击新建） | 正常（SC 不适用） |
| light | three-create 手机 | null / "" | role=textbox name=手机 ignored=false | visible=true | 正常（SC 不适用） |
| light | three-create 邮箱 | null / "" | role=textbox name=邮箱 ignored=false | visible=true | 正常（SC 不适用） |
| light | personal #oldPwd | current-password / current-password | role=textbox 改密表单 | visible=true | 正常（正面对照） |
| light | personal #newPwd/#confirmPwd | new-password / new-password | 同上 | visible=true | 正常（正面对照） |
| light | designer comment 普通态 | 无 `.cm-author-input` | commentInputSeen=false, togglesClicked=[] | 不可达 | 不可达 L1 |
| light | designer comment 强制态 | null / "" | className=cm-author-input, outerHTML=`<input data-v-21a4c630="" placeholder="姓名" class="cm-author-input" style="">` | synthetic-only | 不可达 L1 |
| dark | 同上 7 行 | 同 light | 同 light | 同 light | 同 light |

 designer 普通态与强制态 evidence 原文：`reachability: "synthetic-only (Vue setupState forced; NOT reachable by ordinary UI in this build)"`，强制态 `forcedState.found=true, label=setupState, componentName=ProcessDesigner, before=false, after=true`。three-create 三 input outerHTML 示例：`<input data-v-9081005b="" placeholder="姓名" class="form-input" required="" id="oa-label-control-000001" style="">`（手机/邮箱同类，均无 autocomplete 属性）。

## 六、截图与哈希（脱敏，均在 gitignored 目录）

代表性截图 SHA-256（完整 20 项见 `oa4rust/test-results/r520-evidence/evidence.json`）：

| 截图 | SHA-256 |
| --- | --- |
| `oa4rust/test-results/r520-evidence/dark-three-create.png` | 94e4e6026e0e73b9844d8b8bec9fcd91325c45ffd10d05606c2583ce7a3c3c4d |
| `oa4rust/test-results/r520-evidence/light-three-create.png` | dbb1a0c743d567035c8c7092a5d4c8822f8f8ac9dc5907a07c430b9ad8779838 |
| `oa4rust/test-results/r520-evidence/dark-personal-password.png` | 见 evidence.json sha256 映射 |
| `oa4rust/test-results/r520-evidence/light-designer-comment-forced.png` | 见 evidence.json sha256 映射 |

主证据 JSON：`oa4rust/test-results/r520-evidence/evidence.json`（76761 bytes，2433 行，revision 3e105549a85d8fdabaae1088a6dd992bcf937009bf81ef021e2d520abfec4916），内含全部 DOM/AXTree 快照与 sha256 映射。

## 七、覆盖甄别与转介

- R480 结论冲突：R480 将三方成员姓名/手机/邮箱判「可达真缺陷（P2）」，本轮据真浏览器业务语义（管理员录入第三方资料，非用户本人信息）纠正为 SC 1.3.5 不适用、判正常；两轮均实证 autocomplete 属性缺失，分歧仅在适用性解释，本轮口径为写给队长的依据。
- 评论面板作者姓名：静态缺 token 属实，但普通 UI 不可达；强制态语义（本人姓名/昵称）亦存疑，两重去伪后判不可达 L1，不转缺陷修复。
- 只读边界：本轮未修改任何入库源码；未改 ThreeMemberApp.vue / ProcessDesigner.vue / Personal.vue / 契约测试。若队长决定按宽松解释（把管理员录入字段也纳入 1.3.5）处置，可另排在 ThreeMemberApp.vue:39-41 补 autocomplete="name"/"tel"/"email"，不属本甄别交付。
- 交付边界：唯一入库文档为本文件；runner 脚本、日志、报告、截图全部位于 gitignored 的 `oa4rust/test-results/r520-*`；精确 `git add` 本文档，禁 `-A` / `git add .`；push 若失败 FAIL-LOUD 报告队长补推。
